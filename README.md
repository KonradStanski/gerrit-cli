# gerrit-cli

[![CI](https://github.com/KonradStanski/gerrit-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/KonradStanski/gerrit-cli/actions/workflows/ci.yml)
[![Release](https://github.com/KonradStanski/gerrit-cli/actions/workflows/release.yml/badge.svg)](https://github.com/KonradStanski/gerrit-cli/releases/latest)

A fast CLI for [Gerrit Code Review](https://www.gerritcodereview.com/), built in Rust. Talks to the Gerrit REST API and wraps common workflows into git-style commands.

## Install

**One-liner** (Linux & macOS):

```sh
curl -fsSL https://raw.githubusercontent.com/KonradStanski/gerrit-cli/main/install.sh | sh
```

Or install to a custom directory:

```sh
GERRIT_INSTALL_DIR=~/.local/bin curl -fsSL https://raw.githubusercontent.com/KonradStanski/gerrit-cli/main/install.sh | sh
```

**From source** (requires Rust toolchain):

```sh
cargo install --git https://github.com/KonradStanski/gerrit-cli gerrit-cli
```

**Manual download**: grab a binary from the [latest release](https://github.com/KonradStanski/gerrit-cli/releases/latest).

## Quick Start

```sh
# Point gerrit at your server
gerrit config init

# List open changes for the current repo
gerrit ls

# Show details of a specific change
gerrit show 12345

# Checkout a change locally
gerrit checkout 12345

# Push your current branch for review
gerrit push

# Post a Code-Review +1
gerrit review 12345 --code-review 1 --message "LGTM"
```

## Commands

| Command | Alias | Description |
|---------|-------|-------------|
| `gerrit ls` | `changes` | List/query changes on the Gerrit server |
| `gerrit show <change>` | | Show full change details |
| `gerrit checkout <change>` | `co` | Fetch and checkout a change locally |
| `gerrit push` | | Push current branch for review (`refs/for/<branch>`) |
| `gerrit comments <change>` | | View messages on a change |
| `gerrit review <change>` | | Post a review with scores |
| `gerrit submit <change>` | | Submit a change for merging |
| `gerrit abandon <change>` | | Abandon a change |
| `gerrit config` | | Manage CLI configuration |

### `gerrit ls`

```sh
# All open changes for the current project
gerrit ls

# Filter by owner and branch
gerrit ls --owner self --branch main

# Custom query
gerrit ls --query "status:open label:Code-Review+2"

# Limit results
gerrit ls -n 10
```

### `gerrit checkout`

```sh
# Checkout latest patchset
gerrit checkout 12345

# Checkout a specific patchset
gerrit checkout 12345 --patchset 3

# Checkout into a named branch
gerrit checkout 12345 --branch my-feature
```

### `gerrit push`

```sh
# Push for review on the auto-detected target branch
gerrit push

# Push to a specific branch
gerrit push --branch develop

# Push with topic and reviewers
gerrit push --topic my-feature --reviewers alice,bob

# Push as work-in-progress
gerrit push --wip
```

### `gerrit review`

```sh
# Code-Review +2
gerrit review 12345 --code-review 2

# Verified +1 with a message
gerrit review 12345 --verified 1 --message "Tests pass"
```

### `gerrit comments`

```sh
# Show change messages (review comments, CI results, etc.)
gerrit comments 12345

# Show inline file comments
gerrit comments 12345 --inline
```

## Configuration

Config lives at `~/.config/gerrit-cli/config.toml`. Run `gerrit config init` for interactive setup, or set values directly:

```sh
gerrit config set remotes.myserver.url https://review.example.com
gerrit config set remotes.myserver.username jdoe
gerrit config set default.remote myserver
```

Show current config:

```sh
gerrit config show
```

### Authentication

Passwords are resolved in order:

1. `GERRIT_PASSWORD` environment variable
2. `git credential fill` (works with credential helpers, `.netrc`, macOS Keychain, etc.)

The password is **never** stored in the config file. Use your OS credential manager or a `.netrc` entry:

```
machine review.example.com login jdoe password your-http-password
```

### Auto-detection

If no URL is configured, `gerrit-cli` will try to detect the Gerrit server from your git remote URL. This works for most setups where the remote points at the Gerrit host.

The project name is also auto-detected from the remote, so `gerrit ls` works without any flags inside a cloned repo.

## Architecture

The repo is a Cargo workspace with two crates:

- **`gerrit-api`** — Pure Rust library for the Gerrit REST API. No CLI concerns. Can be used independently in other tools.
- **`gerrit-cli`** — The `gerrit` binary. Depends on `gerrit-api`.

## License

MIT
