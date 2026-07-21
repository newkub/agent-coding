// CLI runner - parses argv and dispatches to the appropriate handler.
// This is the only entry point the binary should call into.

use clap::Parser;

use crate::presentation::cli::commands::{self, Cli};
use crate::shared::kernel::result::AppResult;

/// Entry point for the CLI presentation layer.
/// Parses the command line and dispatches to a handler.
pub(crate) async fn run() -> AppResult<()> {
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
