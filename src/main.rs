// Binary entry point - thin shell that delegates to the library's
// presentation layer. All business logic lives in the `agent_tui` lib.

use agent_tui::presentation::cli::runner;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    runner::run().await.map_err(Into::into)
}
