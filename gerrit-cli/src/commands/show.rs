use anyhow::Result;

use crate::output;

pub fn run(client: &gerrit_api::GerritClient, change: &str) -> Result<()> {
    let detail = client.get_change_detail(change)?;
    output::print_change_detail(&detail);
    Ok(())
}
