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
        #[command(subcommand)]
        command: HeadlessCommands,
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
        /// Action (analyze, snapshot, report, list, suggestions)
        action: String,
    },

    /// Manage collaboration sessions
    Collaboration {
        #[command(subcommand)]
        command: CollaborationCommands,
    },

    /// Manage macros
    Macro {
        #[command(subcommand)]
        command: MacroCommands,
    },

    /// Manage share links
    Share {
        #[command(subcommand)]
        command: ShareCommands,
    },

    /// Manage audit logs
    Audit {
        #[command(subcommand)]
        command: AuditCommands,
    },

    /// Manage sessions
    Session {
        #[command(subcommand)]
        command: SessionCommands,
    },

    /// Show version information
    Version,
}

#[derive(Debug, Subcommand)]
pub(crate) enum HeadlessCommands {
    /// Execute a single headless command
    Execute {
        /// Command to execute
        command: String,
        /// Working directory
        #[arg(short, long, default_value = ".")]
        directory: String,
        /// Output format (text, json, markdown)
        #[arg(short, long, default_value = "text")]
        format: String,
    },
    /// List headless sessions
    List,
    /// Create a headless session
    Create,
    /// Delete a headless session
    Delete {
        /// Session ID
        id: String,
    },
    /// Load a headless session
    Load {
        /// Session ID
        id: String,
    },
    /// Save a headless session
    Save {
        /// Session ID
        id: String,
    },
    /// Execute a batch of commands in sequence
    Batch {
        /// Commands to execute
        commands: Vec<String>,
    },
}

#[derive(Debug, Subcommand)]
pub(crate) enum CollaborationCommands {
    /// List active collaboration sessions
    List,
    /// Create a new collaboration session
    Create {
        /// Session name
        name: String,
    },
    /// Join an existing collaboration session
    Join {
        /// Session ID
        id: String,
        /// Participant name
        name: String,
    },
    /// Send a chat message to a session
    Send {
        /// Session ID
        id: String,
        /// Message content
        message: String,
    },
    /// Leave a collaboration session
    Leave {
        /// Session ID
        id: String,
    },
}

#[derive(Debug, Subcommand)]
pub(crate) enum MacroCommands {
    /// List recorded macros
    List,
    /// Start recording a new macro
    Record {
        /// Macro name
        name: String,
    },
    /// Stop the most recent recording
    Stop,
    /// Playback a recorded macro
    Playback {
        /// Macro name
        name: String,
    },
    /// Delete a macro
    Delete {
        /// Macro name
        name: String,
    },
}

#[derive(Debug, Subcommand)]
pub(crate) enum SubagentCommands {
    /// Get subagent details
    Get {
        /// Subagent name or ID
        name: String,
    },
    /// List available subagents
    List,
    /// Execute a subagent task
    Execute {
        /// Subagent name
        agent: String,
        /// Task input
        input: String,
    },
    /// Delete a subagent
    Delete {
        /// Subagent name or ID
        name: String,
    },
}

#[derive(Debug, Subcommand)]
pub(crate) enum ShareCommands {
    /// Create a share link for a session
    Create {
        /// Session name
        session_name: String,
    },
    /// Deactivate a share link by token
    Deactivate {
        /// Share link token
        token: String,
    },
    /// Access a shared session by token
    Access {
        /// Share link token
        token: String,
    },
}

#[derive(Debug, Subcommand)]
pub(crate) enum AuditCommands {
    /// Log an audit entry
    Log {
        /// Audit message
        message: String,
        /// Audit category
        #[arg(short, long, default_value = "system")]
        category: String,
    },
    /// Query audit logs
    Query {
        /// Start time (RFC 3339)
        #[arg(short, long)]
        from: Option<String>,
        /// Filter by category
        #[arg(short, long)]
        level: Option<String>,
    },
}

#[derive(Debug, Subcommand)]
pub(crate) enum SessionCommands {
    /// List all sessions
    List,
    /// Create a new session
    Create {
        /// Session name
        name: String,
    },
    /// Delete a session by ID
    Delete {
        /// Session ID
        id: String,
    },
    /// Search sessions by name
    Search {
        /// Search query
        query: String,
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
        Commands::Headless { command } => handlers::headless::run(command).await,
        Commands::Subagent { command } => handlers::subagent::run(command).await,
        Commands::Guardrail {
            input,
            guardrail_type,
        } => handlers::guardrail::run(input, guardrail_type).await,
        Commands::Performance { action } => handlers::performance::run(action).await,
        Commands::Collaboration { command } => handlers::collaboration::run(command).await,
        Commands::Macro { command } => handlers::macros::run(command).await,
        Commands::Share { command } => handlers::share::run(command).await,
        Commands::Audit { command } => handlers::audit::run(command).await,
        Commands::Session { command } => handlers::session::run(command).await,
        Commands::Version => handlers::version::run(),
    }
}
