// TUI handler - delegates to the TUI presentation module
// The TUI is a separate entry point that takes over the terminal

use crate::presentation::tui::runner;
use crate::shared::kernel::result::AppResult;

pub(crate) async fn run() -> AppResult<()> {
    runner::run_tui().await
}
