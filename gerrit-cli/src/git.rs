use std::process::Command;

use anyhow::{Context, Result};

/// Run a git command and capture its stdout. Fails if the command exits non-zero.
pub fn git(args: &[&str]) -> Result<String> {
    let output = Command::new("git")
        .args(args)
        .output()
        .context("Failed to execute git")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("git {} failed: {}", args.join(" "), stderr.trim());
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

/// Run a git command interactively (inheriting stdin/stdout/stderr).
pub fn git_interactive(args: &[&str]) -> Result<()> {
    let status = Command::new("git")
        .args(args)
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status()
        .context("Failed to execute git")?;

    if !status.success() {
        anyhow::bail!(
            "git {} failed with exit code {:?}",
            args.join(" "),
            status.code()
        );
    }

    Ok(())
}

/// Get the current branch name.
pub fn current_branch() -> Result<String> {
    git(&["rev-parse", "--abbrev-ref", "HEAD"])
}

/// Get the remote URL for the given remote name (default: "origin").
pub fn remote_url(remote: Option<&str>) -> Result<String> {
    let remote = remote.unwrap_or("origin");
    git(&["remote", "get-url", remote])
}

/// Get the upstream tracking branch for the current branch, if set.
pub fn upstream_branch() -> Result<Option<String>> {
    match git(&["rev-parse", "--abbrev-ref", "@{upstream}"]) {
        Ok(branch) => Ok(Some(branch)),
        Err(_) => Ok(None),
    }
}

/// Fetch a specific ref from a remote.
pub fn fetch_ref(remote: &str, refspec: &str) -> Result<()> {
    git_interactive(&["fetch", remote, refspec])
}

/// Checkout a ref, creating a branch if specified.
pub fn checkout_branch(branch: &str, start_point: &str) -> Result<()> {
    git_interactive(&["checkout", "-B", branch, start_point])
}

/// Checkout FETCH_HEAD.
pub fn checkout_fetch_head() -> Result<()> {
    git_interactive(&["checkout", "FETCH_HEAD"])
}

/// Push a refspec to a remote interactively.
pub fn push_ref(remote: &str, refspec: &str) -> Result<()> {
    git_interactive(&["push", remote, refspec])
}

/// Construct the refspec for fetching a Gerrit change.
/// Gerrit stores changes at refs/changes/XX/CHANGE/PATCHSET.
/// The `XX` is the last two digits of the change number.
pub fn change_refspec(change_number: i64, patchset: Option<i32>) -> String {
    let suffix = change_number % 100;
    match patchset {
        Some(ps) => format!("refs/changes/{suffix:02}/{change_number}/{ps}"),
        None => format!("refs/changes/{suffix:02}/{change_number}/*"),
    }
}

/// Check if we're inside a git repository.
pub fn is_git_repo() -> bool {
    git(&["rev-parse", "--is-inside-work-tree"]).is_ok()
}

/// Get the root directory of the git repository.
pub fn repo_root() -> Result<String> {
    git(&["rev-parse", "--show-toplevel"])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change_refspec() {
        assert_eq!(change_refspec(12345, None), "refs/changes/45/12345/*");
        assert_eq!(change_refspec(12345, Some(3)), "refs/changes/45/12345/3");
        assert_eq!(change_refspec(7, Some(1)), "refs/changes/07/7/1");
    }
}
