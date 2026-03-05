use anyhow::Result;
use colored::Colorize;

pub fn run(client: &gerrit_api::GerritClient, change: &str) -> Result<()> {
    let result = client.submit_change(change)?;

    let status = result.status.as_deref().unwrap_or("UNKNOWN");
    let subject = result.subject.as_deref().unwrap_or("(no subject)");

    println!(
        "Change {change} submitted: {} — {subject}",
        status.blue().bold()
    );

    Ok(())
}
