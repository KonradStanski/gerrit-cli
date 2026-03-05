use anyhow::Result;
use gerrit_api::QueryBuilder;

use crate::output;

pub fn run(
    client: &gerrit_api::GerritClient,
    query: Option<&str>,
    status: Option<&str>,
    project: Option<&str>,
    owner: Option<&str>,
    branch: Option<&str>,
    limit: u32,
) -> Result<()> {
    let query_str = if let Some(raw) = query {
        raw.to_string()
    } else {
        let mut qb = QueryBuilder::new();

        qb = qb.status(status.unwrap_or("open"));

        if let Some(p) = project {
            qb = qb.project(p);
        } else if let Ok(p) = crate::config::resolve_project() {
            qb = qb.project(&p);
        }

        if let Some(o) = owner {
            qb = qb.owner(o);
        }

        if let Some(b) = branch {
            qb = qb.branch(b);
        }

        qb.build()
    };

    let options = &["LABELS", "DETAILED_ACCOUNTS"];

    let changes = client.query_changes(&query_str, Some(limit), options)?;
    output::print_changes_table(&changes);

    Ok(())
}
