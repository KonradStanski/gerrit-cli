use colored::Colorize;
use gerrit_api::ChangeInfo;

/// Format a list of changes as a table for terminal output.
pub fn print_changes_table(changes: &[ChangeInfo]) {
    if changes.is_empty() {
        println!("No changes found.");
        return;
    }

    // Calculate column widths
    let mut num_w = 6;
    let mut subject_w = 7;
    let mut owner_w = 5;
    let mut branch_w = 6;
    let mut status_w = 6;

    for c in changes {
        let num_len = c.number.map(|n| n.to_string().len()).unwrap_or(0);
        let subject_len = c.subject.as_deref().unwrap_or("").len();
        let owner_len = c
            .owner
            .as_ref()
            .and_then(|o| o.name.as_deref())
            .unwrap_or("")
            .len();
        let branch_len = c.branch.as_deref().unwrap_or("").len();
        let status_len = c.status.as_deref().unwrap_or("").len();

        num_w = num_w.max(num_len);
        subject_w = subject_w.max(subject_len).min(60);
        owner_w = owner_w.max(owner_len).min(20);
        branch_w = branch_w.max(branch_len).min(30);
        status_w = status_w.max(status_len);
    }

    // Header
    println!(
        "{:<num_w$}  {:<subject_w$}  {:<owner_w$}  {:<branch_w$}  {:<status_w$}",
        "Number".bold(),
        "Subject".bold(),
        "Owner".bold(),
        "Branch".bold(),
        "Status".bold(),
    );
    println!(
        "{}",
        "-".repeat(num_w + subject_w + owner_w + branch_w + status_w + 8)
    );

    for c in changes {
        let number = c.number.map(|n| n.to_string()).unwrap_or_default();
        let subject = c.subject.as_deref().unwrap_or("");
        let subject = if subject.len() > 60 {
            format!("{}...", &subject[..57])
        } else {
            subject.to_string()
        };
        let owner = c
            .owner
            .as_ref()
            .and_then(|o| o.name.as_deref())
            .unwrap_or("");
        let owner = if owner.len() > 20 {
            format!("{}...", &owner[..17])
        } else {
            owner.to_string()
        };
        let branch = c.branch.as_deref().unwrap_or("");
        let branch = if branch.len() > 30 {
            format!("{}...", &branch[..27])
        } else {
            branch.to_string()
        };
        let status = c.status.as_deref().unwrap_or("");

        let status_colored = match status {
            "NEW" => status.green().to_string(),
            "MERGED" => status.blue().to_string(),
            "ABANDONED" => status.red().to_string(),
            _ => status.to_string(),
        };

        println!(
            "{:<num_w$}  {:<subject_w$}  {:<owner_w$}  {:<branch_w$}  {:<status_w$}",
            number.yellow(),
            subject,
            owner.cyan(),
            branch,
            status_colored,
        );
    }
}

/// Print a detailed view of a single change.
pub fn print_change_detail(change: &ChangeInfo) {
    let number = change.number.map(|n| n.to_string()).unwrap_or_default();
    let subject = change.subject.as_deref().unwrap_or("(no subject)");
    let status = change.status.as_deref().unwrap_or("UNKNOWN");
    let project = change.project.as_deref().unwrap_or("");
    let branch = change.branch.as_deref().unwrap_or("");
    let topic = change.topic.as_deref().unwrap_or("");
    let owner = change
        .owner
        .as_ref()
        .and_then(|o| o.name.as_deref())
        .unwrap_or("Unknown");
    let change_id = change.change_id.as_deref().unwrap_or("");
    let created = change.created.as_deref().unwrap_or("");
    let updated = change.updated.as_deref().unwrap_or("");
    let insertions = change.insertions.unwrap_or(0);
    let deletions = change.deletions.unwrap_or(0);

    println!("{} {}", "Change:".bold(), number.yellow());
    println!("{} {}", "Subject:".bold(), subject);
    println!("{} {}", "Status:".bold(), colorize_status(status));
    println!("{} {}", "Project:".bold(), project);
    println!("{} {}", "Branch:".bold(), branch);
    if !topic.is_empty() {
        println!("{} {}", "Topic:".bold(), topic);
    }
    println!("{} {}", "Owner:".bold(), owner.cyan());
    println!("{} {}", "Change-Id:".bold(), change_id);
    println!("{} {}", "Created:".bold(), created);
    println!("{} {}", "Updated:".bold(), updated);
    println!(
        "{} {} insertions(+), {} deletions(-)",
        "Size:".bold(),
        format!("{insertions}").green(),
        format!("{deletions}").red()
    );

    // Labels
    if let Some(labels) = &change.labels {
        println!("\n{}", "Labels:".bold());
        for (name, info) in labels {
            let mut parts = Vec::new();
            if let Some(ref who) = info.approved {
                parts.push(format!("+2 ({})", who.name.as_deref().unwrap_or("?")));
            }
            if let Some(ref who) = info.recommended {
                parts.push(format!("+1 ({})", who.name.as_deref().unwrap_or("?")));
            }
            if let Some(ref who) = info.disliked {
                parts.push(format!("-1 ({})", who.name.as_deref().unwrap_or("?")));
            }
            if let Some(ref who) = info.rejected {
                parts.push(format!("-2 ({})", who.name.as_deref().unwrap_or("?")));
            }
            if parts.is_empty() {
                println!("  {name}: (none)");
            } else {
                println!("  {name}: {}", parts.join(", "));
            }
        }
    }

    // Current revision commit message
    if let Some(ref revisions) = change.revisions {
        if let Some(current_rev) = &change.current_revision {
            if let Some(rev_info) = revisions.get(current_rev) {
                if let Some(ref commit) = rev_info.commit {
                    if let Some(ref message) = commit.message {
                        println!("\n{}", "Commit message:".bold());
                        for line in message.lines() {
                            println!("  {line}");
                        }
                    }
                }
            }
        }
    }

    // Messages summary
    if let Some(ref messages) = change.messages {
        let count = messages.len();
        println!("\n{} {count} message(s)", "Messages:".bold());
    }
}

/// Print change messages.
pub fn print_messages(messages: &[gerrit_api::ChangeMessageInfo]) {
    if messages.is_empty() {
        println!("No messages.");
        return;
    }

    for msg in messages {
        let author = msg
            .author
            .as_ref()
            .and_then(|a| a.name.as_deref())
            .unwrap_or("System");
        let date = msg.date.as_deref().unwrap_or("");
        let text = msg.message.as_deref().unwrap_or("");
        let ps = msg
            .revision_number
            .map(|n| format!(" (PS{n})"))
            .unwrap_or_default();

        println!("{}{} — {}", author.cyan().bold(), ps, date.dimmed());
        for line in text.lines() {
            println!("  {line}");
        }
        println!();
    }
}

/// Print inline comments grouped by file.
pub fn print_comments(comments: &std::collections::HashMap<String, Vec<gerrit_api::CommentInfo>>) {
    if comments.is_empty() {
        println!("No comments.");
        return;
    }

    for (file, file_comments) in comments {
        println!("{}", file.bold().underline());
        for comment in file_comments {
            let author = comment
                .author
                .as_ref()
                .and_then(|a| a.name.as_deref())
                .unwrap_or("Unknown");
            let line = comment
                .line
                .map(|l| format!("line {l}"))
                .unwrap_or_else(|| "file".to_string());
            let msg = comment.message.as_deref().unwrap_or("");
            let resolved = if comment.unresolved == Some(true) {
                " [UNRESOLVED]".red().to_string()
            } else {
                String::new()
            };

            println!("  {} ({}){}:", author.cyan(), line, resolved);
            for text_line in msg.lines() {
                println!("    {text_line}");
            }
            println!();
        }
    }
}

fn colorize_status(status: &str) -> String {
    match status {
        "NEW" => status.green().to_string(),
        "MERGED" => status.blue().to_string(),
        "ABANDONED" => status.red().to_string(),
        _ => status.to_string(),
    }
}
