// Binary entry point - thin shell that delegates to the presentation layer.
// All command parsing, dispatching, and handler logic lives in
// `presentation::cli` so this file stays minimal and focused.

mod adapters;
mod modules;
mod presentation;
mod shared;

use presentation::cli::runner;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    runner::run().await.map_err(Into::into)
}
