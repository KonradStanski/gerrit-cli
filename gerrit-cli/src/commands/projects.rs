use anyhow::Result;

use crate::output;

pub fn run(
    client: &gerrit_api::GerritClient,
    filter: Option<&str>,
    regex: Option<&str>,
    limit: u32,
) -> Result<()> {
    let projects = client.list_projects(filter, regex, Some(limit))?;
    output::print_projects_table(&projects);
    Ok(())
}
