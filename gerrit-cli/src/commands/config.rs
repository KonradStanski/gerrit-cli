use std::io::{self, Write};

use anyhow::Result;
use colored::Colorize;

use crate::config::{Config, DefaultConfig, RemoteConfig};

pub fn run_show() -> Result<()> {
    let config = Config::load()?;
    let path = Config::config_path()?;

    println!("{} {}", "Config file:".bold(), path.display());
    println!();

    if let Some(ref remote) = config.default.remote {
        println!("{} {remote}", "Default remote:".bold());
    } else {
        println!("{} (none)", "Default remote:".bold());
    }

    if config.remotes.is_empty() {
        println!("\nNo remotes configured.");
    } else {
        println!("\n{}", "Remotes:".bold());
        for (name, remote) in &config.remotes {
            let url = remote.url.as_deref().unwrap_or("(not set)");
            let user = remote.username.as_deref().unwrap_or("(not set)");
            println!("  [{name}]");
            println!("    url:      {url}");
            println!("    username: {user}");
        }
    }

    Ok(())
}

pub fn run_set(key: &str, value: &str) -> Result<()> {
    let mut config = Config::load()?;

    let parts: Vec<&str> = key.split('.').collect();
    match parts.as_slice() {
        ["default", "remote"] => {
            config.default.remote = Some(value.to_string());
        }
        ["remotes", name, "url"] => {
            config
                .remotes
                .entry(name.to_string())
                .or_insert_with(RemoteConfig::default)
                .url = Some(value.to_string());
        }
        ["remotes", name, "username"] => {
            config
                .remotes
                .entry(name.to_string())
                .or_insert_with(RemoteConfig::default)
                .username = Some(value.to_string());
        }
        _ => {
            anyhow::bail!(
                "Unknown config key: {key}. Valid keys: default.remote, remotes.<name>.url, remotes.<name>.username"
            );
        }
    }

    config.save()?;
    println!("Set {key} = {value}");

    Ok(())
}

pub fn run_init() -> Result<()> {
    println!("{}", "Gerrit CLI Configuration".bold());
    println!();

    let remote_name = prompt("Remote name", "default")?;
    let url = prompt("Gerrit server URL", "")?;
    let username = prompt("Username", "")?;

    if url.is_empty() {
        anyhow::bail!("URL is required");
    }

    let mut config = Config::load()?;
    config.default = DefaultConfig {
        remote: Some(remote_name.clone()),
    };
    config.remotes.insert(
        remote_name,
        RemoteConfig {
            url: Some(url),
            username: if username.is_empty() {
                None
            } else {
                Some(username)
            },
        },
    );

    config.save()?;
    let path = Config::config_path()?;
    println!("\nConfiguration saved to {}", path.display());

    Ok(())
}

fn prompt(label: &str, default: &str) -> Result<String> {
    if default.is_empty() {
        print!("{label}: ");
    } else {
        print!("{label} [{default}]: ");
    }
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim();

    if input.is_empty() {
        Ok(default.to_string())
    } else {
        Ok(input.to_string())
    }
}
