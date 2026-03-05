use anyhow::Result;

use crate::git;

pub fn run(
    branch: Option<&str>,
    topic: Option<&str>,
    reviewers: Option<&str>,
    wip: bool,
) -> Result<()> {
    let target_branch = if let Some(b) = branch {
        b.to_string()
    } else if let Ok(Some(upstream)) = git::upstream_branch() {
        // Extract branch name from "origin/main" -> "main"
        upstream.rsplit('/').next().unwrap_or("main").to_string()
    } else {
        "main".to_string()
    };

    let mut refspec = format!("HEAD:refs/for/{target_branch}");

    // Build push options as %key=value suffixes
    let mut opts: Vec<String> = Vec::new();

    if let Some(t) = topic {
        opts.push(format!("topic={t}"));
    }

    if wip {
        opts.push("wip".to_string());
    }

    if let Some(r) = reviewers {
        for reviewer in r.split(',') {
            let reviewer = reviewer.trim();
            if !reviewer.is_empty() {
                opts.push(format!("r={reviewer}"));
            }
        }
    }

    if !opts.is_empty() {
        refspec.push('%');
        refspec.push_str(&opts.join(","));
    }

    println!("Pushing to refs/for/{target_branch}...");
    git::push_ref("origin", &refspec)?;

    Ok(())
}
