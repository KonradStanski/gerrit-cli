use anyhow::Result;
use colored::Colorize;

pub fn run(client: &gerrit_api::GerritClient, change: &str, message: Option<&str>) -> Result<()> {
    let result = client.abandon_change(change, message)?;

    let status = result.status.as_deref().unwrap_or("UNKNOWN");
    let subject = result.subject.as_deref().unwrap_or("(no subject)");

    println!(
        "Change {change} abandoned: {} — {subject}",
        status.red().bold()
    );

    Ok(())
}
