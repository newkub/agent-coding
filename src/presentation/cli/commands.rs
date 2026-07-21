// CLI command definitions
// Defines the command tree using clap derive macros and dispatches to handlers

use crate::presentation::cli::handlers;
use crate::shared::kernel::result::AppResult;
use clap::{Parser, Subcommand};

/// agent-tui command-line interface
#[derive(Debug, Parser)]
#[command(
    name = "agent-tui",
    about = "Terminal-based AI assistant interface with clean architecture",
    long_about = "A modern terminal-based AI assistant for code analysis, automation, and development workflows."
)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    /// Run the TUI interface (default)
    Tui,

    /// Analyze codebase structure and dependencies
    Onboarding {
        /// Path to the project directory
        path: String,
    },

    /// Automate issue-to-PR workflow
    Automate {
        /// Repository in format owner/repo
        repository: String,
        /// Issue number
        number: u32,
    },

    /// Run in headless mode
    Headless {
        /// Command to execute
        command: String,
        /// Working directory
        #[arg(short, long, default_value = ".")]
        directory: String,
        /// Output format (text, json, markdown)
        #[arg(short, long, default_value = "text")]
        format: String,
    },

    /// Manage subagents
    Subagent {
        #[command(subcommand)]
        command: SubagentCommands,
    },

    /// Run guardrails check
    Guardrail {
        /// Input to check
        input: String,
        /// Guardrail type (security, quality, performance)
        #[arg(short, long, default_value = "security")]
        guardrail_type: String,
    },

    /// Analyze performance metrics
    Performance {
        /// Action (analyze, snapshot, report)
        action: String,
    },

    /// Show version information
    Version,
}

#[derive(Debug, Subcommand)]
pub(crate) enum SubagentCommands {
    /// List available subagents
    List,
    /// Execute a subagent task
    Execute {
        /// Subagent name
        agent: String,
        /// Task input
        input: String,
    },
}

/// Dispatch a parsed CLI command to its handler
pub(crate) async fn dispatch(command: Commands) -> AppResult<()> {
    match command {
        Commands::Tui => handlers::tui::run().await,
        Commands::Onboarding { path } => handlers::onboarding::run(path).await,
        Commands::Automate { repository, number } => {
            handlers::automation::run(repository, number).await
        }
        Commands::Headless {
            command,
            directory,
            format,
        } => handlers::headless::run(command, directory, format).await,
        Commands::Subagent { command } => handlers::subagent::run(command).await,
        Commands::Guardrail {
            input,
            guardrail_type,
        } => handlers::guardrail::run(input, guardrail_type).await,
        Commands::Performance { action } => handlers::performance::run(action).await,
        Commands::Version => handlers::version::run(),
    }
}
