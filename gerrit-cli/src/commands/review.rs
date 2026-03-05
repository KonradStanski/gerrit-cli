use std::collections::HashMap;

use anyhow::Result;
use gerrit_api::ReviewInput;

pub fn run(
    client: &gerrit_api::GerritClient,
    change: &str,
    message: Option<&str>,
    code_review: Option<i32>,
    verified: Option<i32>,
) -> Result<()> {
    let mut labels = HashMap::new();

    if let Some(cr) = code_review {
        labels.insert("Code-Review".to_string(), cr);
    }
    if let Some(v) = verified {
        labels.insert("Verified".to_string(), v);
    }

    let input = ReviewInput {
        message: message.map(|s| s.to_string()),
        labels: if labels.is_empty() {
            None
        } else {
            Some(labels)
        },
        tag: None,
    };

    let result = client.set_review(change, "current", &input)?;

    println!("Review posted on change {change}.");
    if let Some(labels) = &result.labels {
        for (name, value) in labels {
            let sign = if *value > 0 { "+" } else { "" };
            println!("  {name}: {sign}{value}");
        }
    }

    Ok(())
}
