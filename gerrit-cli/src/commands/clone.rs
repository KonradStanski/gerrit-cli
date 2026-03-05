use anyhow::{Context, Result};

use crate::config;
use crate::git;

use super::install_hooks;

pub fn run(base_url: &str, project: &str, directory: Option<&str>, http: bool) -> Result<()> {
    let clone_url = if http {
        // HTTPS clone URL: {base_url}/a/{project}
        format!("{}/a/{project}", base_url.trim_end_matches('/'))
    } else {
        // SSH clone URL: ssh://{username}@{host}:{port}/{project}
        let parsed = url::Url::parse(base_url)?;
        let host = parsed
            .host_str()
            .context("Could not determine host from Gerrit URL")?;
        let username = config::resolve_username()?;
        let port = 29418;
        format!("ssh://{username}@{host}:{port}/{project}")
    };

    let target_dir = directory.unwrap_or_else(|| {
        // Use last component of project as directory name
        project.rsplit('/').next().unwrap_or(project)
    });

    println!("Cloning {project} from {clone_url}...");
    git::git_interactive(&["clone", &clone_url, target_dir])?;

    // Auto-install commit-msg hook into the cloned repo
    let repo_root = std::fs::canonicalize(target_dir)
        .with_context(|| format!("Could not resolve path: {target_dir}"))?;
    let root_str = repo_root.to_string_lossy();

    match install_hooks::install_commit_msg_hook(base_url, &root_str) {
        Ok(true) => println!("Installed Gerrit commit-msg hook."),
        Ok(false) => {}
        Err(e) => eprintln!("Warning: could not install commit-msg hook: {e}"),
    }

    println!("Cloned {project} into {target_dir}");
    Ok(())
}
