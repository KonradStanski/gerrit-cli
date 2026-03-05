use anyhow::Result;

use crate::output;

pub fn run(client: &gerrit_api::GerritClient, change: &str, inline: bool) -> Result<()> {
    if inline {
        let comments = client.get_comments(change)?;
        output::print_comments(&comments);
    } else {
        let messages = client.get_messages(change)?;
        output::print_messages(&messages);
    }

    Ok(())
}
