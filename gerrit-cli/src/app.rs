use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "gerrit", about = "CLI for Gerrit Code Review", version)]
pub struct Cli {
    /// Gerrit server URL (overrides config and auto-detection)
    #[arg(long, global = true)]
    pub url: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List/query changes on the Gerrit server
    #[command(alias = "changes")]
    Ls {
        /// Query string (e.g. "status:open owner:self")
        #[arg(short, long)]
        query: Option<String>,

        /// Filter by status (open, merged, abandoned)
        #[arg(short, long)]
        status: Option<String>,

        /// Filter by project (auto-detected from git remote if not specified)
        #[arg(short, long)]
        project: Option<String>,

        /// Filter by owner
        #[arg(short, long)]
        owner: Option<String>,

        /// Filter by branch
        #[arg(short, long)]
        branch: Option<String>,

        /// Maximum number of results
        #[arg(short, long, default_value = "25")]
        number: u32,
    },

    /// Show full details of a change
    Show {
        /// Change number or Change-Id
        change: String,
    },

    /// Fetch and checkout a change locally
    #[command(alias = "co")]
    Checkout {
        /// Change number
        change: i64,

        /// Patchset number (latest if not specified)
        #[arg(short, long)]
        patchset: Option<i32>,

        /// Branch name to create (default: change/<number>)
        #[arg(short, long)]
        branch: Option<String>,
    },

    /// Push current branch for review
    Push {
        /// Target branch (default: auto-detect from upstream or "main")
        #[arg(short, long)]
        branch: Option<String>,

        /// Set a topic
        #[arg(short, long)]
        topic: Option<String>,

        /// Add reviewers (comma-separated)
        #[arg(short, long)]
        reviewers: Option<String>,

        /// Push as draft/WIP
        #[arg(long)]
        wip: bool,
    },

    /// View comments and messages on a change
    Comments {
        /// Change number or Change-Id
        change: String,

        /// Show inline file comments instead of change messages
        #[arg(short, long)]
        inline: bool,
    },

    /// Post a review on a change
    Review {
        /// Change number or Change-Id
        change: String,

        /// Review message
        #[arg(short, long)]
        message: Option<String>,

        /// Code-Review score (-2 to +2)
        #[arg(long)]
        code_review: Option<i32>,

        /// Verified score (-1 to +1)
        #[arg(long)]
        verified: Option<i32>,
    },

    /// Submit a change for merging
    Submit {
        /// Change number or Change-Id
        change: String,
    },

    /// Abandon a change
    Abandon {
        /// Change number or Change-Id
        change: String,

        /// Reason for abandoning
        #[arg(short, long)]
        message: Option<String>,
    },

    /// Install Gerrit commit-msg hook into current repo
    InstallHooks,

    /// List projects on the Gerrit server
    Projects {
        /// Filter projects by name (substring match)
        #[arg(short, long)]
        filter: Option<String>,

        /// Filter projects by regex
        #[arg(short, long, conflicts_with = "filter")]
        regex: Option<String>,

        /// Maximum number of results
        #[arg(short, long, default_value = "100")]
        number: u32,
    },

    /// Clone a Gerrit project with hooks pre-configured
    Clone {
        /// Project name (e.g. "my/project")
        project: String,

        /// Target directory (default: last component of project name)
        directory: Option<String>,

        /// Use HTTPS clone URL instead of SSH
        #[arg(long)]
        http: bool,
    },

    /// Manage CLI configuration
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}

#[derive(Subcommand)]
pub enum ConfigAction {
    /// Show current configuration
    Show,

    /// Set a configuration value
    Set {
        /// Key to set (e.g. "default.remote", "remotes.myserver.url")
        key: String,
        /// Value to set
        value: String,
    },

    /// Interactive configuration setup
    Init,
}
