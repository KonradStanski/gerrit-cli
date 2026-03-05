use anyhow::{Context, Result};

use crate::git;

pub fn run(
    client: &gerrit_api::GerritClient,
    change_number: i64,
    patchset: Option<i32>,
    branch_name: Option<&str>,
) -> Result<()> {
    // Get change details to find the latest patchset if not specified
    let change_id = change_number.to_string();
    let detail = client.get_change_detail(&change_id)?;

    let ps = if let Some(ps) = patchset {
        ps
    } else {
        // Find the latest patchset from the current revision
        detail
            .revisions
            .as_ref()
            .and_then(|revs| {
                detail
                    .current_revision
                    .as_ref()
                    .and_then(|cr| revs.get(cr))
                    .and_then(|r| r.number)
            })
            .context("Could not determine latest patchset number")?
    };

    let refspec = git::change_refspec(change_number, Some(ps));
    let branch = branch_name
        .map(|s| s.to_string())
        .unwrap_or_else(|| format!("change/{change_number}"));

    println!("Fetching {refspec}...");
    git::fetch_ref("origin", &refspec)?;

    println!("Checking out to branch {branch}...");
    git::checkout_branch(&branch, "FETCH_HEAD")?;

    let subject = detail.subject.as_deref().unwrap_or("(no subject)");
    println!("Checked out change {change_number} patchset {ps}: {subject}");

    Ok(())
}
