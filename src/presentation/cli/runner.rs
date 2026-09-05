// CLI runner - parses argv and dispatches to the appropriate handler.
// This is the only entry point the binary should call into.

use clap::Parser;

use crate::presentation::cli::commands::{self, Cli};
use crate::shared::kernel::result::AppResult;

/// Entry point for the CLI presentation layer.
/// Parses the command line and dispatches to a handler.
pub async fn run() -> AppResult<()> {
    init_tracing();
    let cli = Cli::parse();

    match cli.command {
        // Default to the TUI when no subcommand is provided
        Some(command) => commands::dispatch(command).await,
        None => {
            // Re-parse with `Tui` to keep the "no args = TUI" UX consistent
            let default_cli = Cli::parse_from(["agent-tui", "tui"]);
            if let Some(command) = default_cli.command {
                commands::dispatch(command).await
            } else {
                Ok(())
            }
        }
    }
}

fn init_tracing() {
    let filter = std::env::var("RUST_LOG")
        .map(tracing_subscriber::EnvFilter::new)
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));
    let _ = tracing_subscriber::fmt().with_env_filter(filter).try_init();
}
