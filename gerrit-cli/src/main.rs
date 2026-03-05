mod app;
mod commands;
mod config;
#[allow(dead_code)]
mod git;
mod output;

use anyhow::Result;
use clap::Parser;

use app::{Cli, Commands, ConfigAction};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        // Commands that don't need a Gerrit client
        Commands::Push {
            branch,
            topic,
            reviewers,
            wip,
        } => {
            commands::push::run(
                branch.as_deref(),
                topic.as_deref(),
                reviewers.as_deref(),
                *wip,
            )?;
        }
        Commands::Config { action } => match action {
            ConfigAction::Show => commands::config::run_show()?,
            ConfigAction::Set { key, value } => commands::config::run_set(key, value)?,
            ConfigAction::Init => commands::config::run_init()?,
        },

        // Commands that need a Gerrit client
        cmd => {
            let client = build_client(cli.url.as_deref())?;
            match cmd {
                Commands::Ls {
                    query,
                    status,
                    project,
                    owner,
                    branch,
                    number,
                } => {
                    commands::ls::run(
                        &client,
                        query.as_deref(),
                        status.as_deref(),
                        project.as_deref(),
                        owner.as_deref(),
                        branch.as_deref(),
                        *number,
                    )?;
                }
                Commands::Show { change } => {
                    commands::show::run(&client, change)?;
                }
                Commands::Checkout {
                    change,
                    patchset,
                    branch,
                } => {
                    commands::checkout::run(&client, *change, *patchset, branch.as_deref())?;
                }
                Commands::Comments { change, inline } => {
                    commands::comments::run(&client, change, *inline)?;
                }
                Commands::Review {
                    change,
                    message,
                    code_review,
                    verified,
                } => {
                    commands::review::run(
                        &client,
                        change,
                        message.as_deref(),
                        *code_review,
                        *verified,
                    )?;
                }
                Commands::Submit { change } => {
                    commands::submit::run(&client, change)?;
                }
                Commands::Abandon { change, message } => {
                    commands::abandon::run(&client, change, message.as_deref())?;
                }
                Commands::Push { .. } | Commands::Config { .. } => unreachable!(),
            }
        }
    }

    Ok(())
}

fn build_client(cli_url: Option<&str>) -> Result<gerrit_api::GerritClient> {
    let base_url = config::resolve_base_url(cli_url)?;
    let username = config::resolve_username()?;
    let password = config::resolve_password(&base_url)?;

    let client = gerrit_api::GerritClient::new(&base_url, &username, &password)?;
    Ok(client)
}
