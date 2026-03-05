use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use anyhow::{Context, Result};

use crate::git;

const GERRIT_HOOK_MARKER: &str = "Change-Id";

/// Install the Gerrit commit-msg hook into the given repo root.
/// Returns true if the hook was newly installed, false if already present.
pub fn install_commit_msg_hook(base_url: &str, repo_root: &str) -> Result<bool> {
    let hooks_dir = Path::new(repo_root).join(".git").join("hooks");
    let hook_path = hooks_dir.join("commit-msg");

    // Check if hook already exists and looks like the Gerrit hook
    if hook_path.exists() {
        let contents = fs::read_to_string(&hook_path).unwrap_or_default();
        if contents.contains(GERRIT_HOOK_MARKER) {
            return Ok(false);
        }
    }

    // Download from Gerrit (unauthenticated endpoint)
    let url = format!("{}/tools/hooks/commit-msg", base_url.trim_end_matches('/'));
    let client = reqwest::blocking::Client::builder()
        .user_agent("gerrit-cli/0.1.0")
        .build()?;
    let resp = client.get(&url).send()?;

    if !resp.status().is_success() {
        anyhow::bail!(
            "Failed to download commit-msg hook from {}: {}",
            url,
            resp.status()
        );
    }

    let hook_bytes = resp.bytes()?;

    // Ensure hooks directory exists
    fs::create_dir_all(&hooks_dir)
        .with_context(|| format!("Failed to create hooks directory: {}", hooks_dir.display()))?;

    // Write the hook
    fs::write(&hook_path, &hook_bytes)
        .with_context(|| format!("Failed to write hook to {}", hook_path.display()))?;

    // Make executable (755)
    fs::set_permissions(&hook_path, fs::Permissions::from_mode(0o755))
        .with_context(|| format!("Failed to chmod hook at {}", hook_path.display()))?;

    Ok(true)
}

/// CLI command: install hooks into current repo.
pub fn run(base_url: &str) -> Result<()> {
    let root = git::repo_root().context("Not inside a git repository")?;

    match install_commit_msg_hook(base_url, &root)? {
        true => println!("Installed Gerrit commit-msg hook into {root}/.git/hooks/commit-msg"),
        false => println!("Gerrit commit-msg hook already installed."),
    }

    Ok(())
}
