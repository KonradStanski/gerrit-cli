use std::path::PathBuf;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Config {
    #[serde(default)]
    pub default: DefaultConfig,
    #[serde(default)]
    pub remotes: std::collections::HashMap<String, RemoteConfig>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct DefaultConfig {
    pub remote: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct RemoteConfig {
    pub url: Option<String>,
    pub username: Option<String>,
}

impl Config {
    /// Load config from the default config file path.
    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = std::fs::read_to_string(&path)
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;
        let config: Config =
            toml::from_str(&content).with_context(|| "Failed to parse config file")?;
        Ok(config)
    }

    /// Save config to the default config file path.
    pub fn save(&self) -> Result<()> {
        let path = Self::config_path()?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self)?;
        std::fs::write(&path, content)?;
        Ok(())
    }

    pub fn config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir().context("Could not determine config directory")?;
        Ok(config_dir.join("gerrit-cli").join("config.toml"))
    }

    /// Get the remote config for the given name, or the default remote.
    pub fn remote(&self, name: Option<&str>) -> Option<&RemoteConfig> {
        let key = name
            .map(|s| s.to_string())
            .or_else(|| self.default.remote.clone())?;
        self.remotes.get(&key)
    }
}

/// Resolve the Gerrit base URL, trying in order:
/// 1. --url CLI flag
/// 2. GERRIT_URL env var
/// 3. Config file remote
/// 4. Auto-detect from git remote
pub fn resolve_base_url(cli_url: Option<&str>) -> Result<String> {
    // 1. CLI flag
    if let Some(url) = cli_url {
        return Ok(url.to_string());
    }

    // 2. Env var
    if let Ok(url) = std::env::var("GERRIT_URL") {
        return Ok(url);
    }

    // 3. Config file
    let config = Config::load()?;
    if let Some(remote) = config.remote(None) {
        if let Some(url) = &remote.url {
            return Ok(url.clone());
        }
    }

    // 4. Auto-detect from git remote
    if let Ok(remote_url) = crate::git::remote_url(None) {
        if let Some(base) = extract_gerrit_url(&remote_url) {
            return Ok(base);
        }
    }

    anyhow::bail!(
        "Could not determine Gerrit URL. Set GERRIT_URL, use --url, or run `gerrit config init`."
    )
}

/// Resolve username, trying:
/// 1. GERRIT_USERNAME env var
/// 2. Config file
/// 3. Git config user
pub fn resolve_username() -> Result<String> {
    if let Ok(user) = std::env::var("GERRIT_USERNAME") {
        return Ok(user);
    }

    let config = Config::load()?;
    if let Some(remote) = config.remote(None) {
        if let Some(username) = &remote.username {
            return Ok(username.clone());
        }
    }

    // Try extracting from remote URL
    if let Ok(remote_url) = crate::git::remote_url(None) {
        if let Some(user) = extract_username_from_url(&remote_url) {
            return Ok(user);
        }
    }

    // Fall back to system username
    if let Ok(user) = std::env::var("USER") {
        return Ok(user);
    }

    anyhow::bail!("Could not determine username. Set GERRIT_USERNAME or run `gerrit config init`.")
}

/// Resolve password, trying:
/// 1. GERRIT_PASSWORD env var
/// 2. git credential fill (non-interactive)
/// 3. Browser login flow: open Gerrit HTTP credentials page, user pastes password
pub fn resolve_password(url: &str, username: &str) -> Result<String> {
    if let Ok(pass) = std::env::var("GERRIT_PASSWORD") {
        return Ok(pass);
    }

    // Try git credential fill non-interactively
    if let Ok(pass) = git_credential_password(url, username) {
        return Ok(pass);
    }

    // Interactive: open browser to Gerrit HTTP credentials page, prompt for password
    let parsed = url::Url::parse(url)?;
    let host = parsed.host_str().unwrap_or("unknown");
    let creds_url = format!("{url}/settings/#HTTPCredentials");

    eprintln!("No saved credentials for {host}.");
    eprintln!("Opening {creds_url}");
    eprintln!("Generate an HTTP password and paste it here.\n");

    let _ = std::process::Command::new("open")
        .arg(&creds_url)
        .spawn();

    eprint!("HTTP password: ");
    std::io::Write::flush(&mut std::io::stderr())?;
    let mut password = String::new();
    std::io::stdin().read_line(&mut password)?;
    let password = password.trim().to_string();

    if password.is_empty() {
        anyhow::bail!("No password provided.");
    }

    // Store via git credential approve so we don't ask again
    git_credential_approve(url, username, &password)?;
    eprintln!("Credentials saved.");

    Ok(password)
}

fn git_credential_password(url: &str, username: &str) -> Result<String> {
    use std::io::Write;
    use std::process::{Command, Stdio};

    let parsed = url::Url::parse(url)?;
    let host = parsed.host_str().unwrap_or("");
    let protocol = parsed.scheme();

    let input = format!("protocol={protocol}\nhost={host}\nusername={username}\n\n");

    let mut child = Command::new("git")
        .args(["credential", "fill"])
        .env("GIT_TERMINAL_PROMPT", "0")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()?;

    if let Some(stdin) = child.stdin.as_mut() {
        stdin.write_all(input.as_bytes())?;
    }

    let output = child.wait_with_output()?;

    if !output.status.success() {
        anyhow::bail!("git credential fill failed");
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    for line in stdout.lines() {
        if let Some(pass) = line.strip_prefix("password=") {
            if !pass.is_empty() {
                return Ok(pass.to_string());
            }
        }
    }

    anyhow::bail!("git credential fill did not return a password")
}

fn git_credential_approve(url: &str, username: &str, password: &str) -> Result<()> {
    use std::io::Write;
    use std::process::{Command, Stdio};

    let parsed = url::Url::parse(url)?;
    let host = parsed.host_str().unwrap_or("");
    let protocol = parsed.scheme();

    let input = format!(
        "protocol={protocol}\nhost={host}\nusername={username}\npassword={password}\n\n"
    );

    let mut child = Command::new("git")
        .args(["credential", "approve"])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;

    if let Some(stdin) = child.stdin.as_mut() {
        stdin.write_all(input.as_bytes())?;
    }

    child.wait()?;
    Ok(())
}

/// Try to extract a Gerrit base URL from a git remote URL.
/// Handles both SSH (ssh://user@host:port/repo) and HTTPS URLs.
fn extract_gerrit_url(remote_url: &str) -> Option<String> {
    // HTTPS remote: https://review.example.com/a/project
    if remote_url.starts_with("https://") || remote_url.starts_with("http://") {
        let parsed = url::Url::parse(remote_url).ok()?;
        let base = format!("{}://{}", parsed.scheme(), parsed.host_str()?);
        if let Some(port) = parsed.port() {
            return Some(format!("{base}:{port}"));
        }
        return Some(base);
    }

    // SSH remote: ssh://user@host:29418/project
    if remote_url.starts_with("ssh://") {
        let parsed = url::Url::parse(remote_url).ok()?;
        let host = parsed.host_str()?;
        let port = parsed.port().unwrap_or(29418);
        // Gerrit REST API is typically on HTTPS port 443 on the same host
        // but we can't be sure, so return https://host
        let _ = port; // SSH port doesn't tell us the HTTP port
        return Some(format!("https://{host}"));
    }

    // SCP-style: user@host:project
    if let Some(at_pos) = remote_url.find('@') {
        let after_at = &remote_url[at_pos + 1..];
        if let Some(colon_pos) = after_at.find(':') {
            let host = &after_at[..colon_pos];
            return Some(format!("https://{host}"));
        }
    }

    None
}

fn extract_username_from_url(remote_url: &str) -> Option<String> {
    if let Ok(parsed) = url::Url::parse(remote_url) {
        let user = parsed.username();
        if !user.is_empty() {
            return Some(user.to_string());
        }
    }

    // SCP-style: user@host:project
    if let Some(at_pos) = remote_url.find('@') {
        let user = &remote_url[..at_pos];
        // Skip "ssh://" prefix if present
        let user = user.strip_prefix("ssh://").unwrap_or(user);
        if !user.is_empty() {
            return Some(user.to_string());
        }
    }

    None
}

/// Extract the Gerrit project name from a git remote URL.
pub fn resolve_project() -> Result<String> {
    let remote_url = crate::git::remote_url(None)?;
    extract_project_from_url(&remote_url)
        .context("Could not determine project name from git remote URL")
}

fn extract_project_from_url(remote_url: &str) -> Option<String> {
    if let Ok(parsed) = url::Url::parse(remote_url) {
        let path = parsed.path().trim_start_matches('/');
        // Strip /a/ prefix if present (Gerrit authenticated clone URL)
        let path = path.strip_prefix("a/").unwrap_or(path);
        let path = path.strip_suffix(".git").unwrap_or(path);
        if !path.is_empty() {
            return Some(path.to_string());
        }
    }

    // SCP-style: user@host:project.git
    if let Some(colon_pos) = remote_url.rfind(':') {
        let project = &remote_url[colon_pos + 1..];
        let project = project.strip_suffix(".git").unwrap_or(project);
        if !project.is_empty() {
            return Some(project.to_string());
        }
    }

    None
}
