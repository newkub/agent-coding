// Macro handler - record/stop/playback/delete macros via the macro
// repository and executor use cases.

use crate::modules::macros::application::usecases;
use crate::presentation::cli::commands::MacroCommands;
use crate::presentation::cli::output;
use crate::presentation::tui::di::DIContainer;
use crate::shared::kernel::result::{AppError, AppResult};

pub(crate) async fn run(command: MacroCommands) -> AppResult<()> {
    let container = DIContainer::new().build().await?;
    let repo = container
        .macro_repo()
        .ok_or_else(|| AppError::State("Macro repository not available".to_string()))?;
    let executor = container
        .macro_executor()
        .ok_or_else(|| AppError::State("Macro executor not available".to_string()))?;

    match command {
        MacroCommands::List => {
            output::print_section("Macros");
            let macros = usecases::list_macros(repo).await?;
            output::print_macro_list(&macros);
        }
        MacroCommands::Record { name } => {
            output::print_section(&format!("Recording macro: {name}"));
            let id = usecases::start_recording(repo, name, String::new()).await?;
            output::print_info(&format!("Recording started: {}", id.as_str()));
        }
        MacroCommands::Stop => {
            output::print_section("Stopping macro recording");
            // In-memory storage: finish the most recently started recording.
            let macros = usecases::list_macros(repo).await?;
            match macros.last() {
                Some(macro_def) => match usecases::stop_recording(repo, &macro_def.id).await? {
                    Some(finished) => output::print_info(&format!(
                        "Recording stopped: {} ({} steps)",
                        finished.name,
                        finished.step_count()
                    )),
                    None => output::print_info("No active recording found (already finished)"),
                },
                None => output::print_info("No recording in progress"),
            }
        }
        MacroCommands::Playback { name } => {
            output::print_section(&format!("Playing macro: {name}"));
            let macros = usecases::list_macros(repo).await?;
            let macro_def = macros
                .iter()
                .find(|m| m.name == name)
                .ok_or_else(|| AppError::NotFound(format!("Macro not found: {name}")))?;
            let result = usecases::playback_macro(executor, macro_def, None).await?;
            output::print_info(&format!(
                "Playback finished: {} step(s), success={}",
                result.step_results.len(),
                result.success
            ));
        }
        MacroCommands::Delete { name } => {
            output::print_section(&format!("Deleting macro: {name}"));
            let macros = usecases::list_macros(repo).await?;
            let macro_def = macros
                .iter()
                .find(|m| m.name == name)
                .ok_or_else(|| AppError::NotFound(format!("Macro not found: {name}")))?;
            usecases::delete_macro(repo, &macro_def.id).await?;
            output::print_info(&format!("Deleted macro: {}", macro_def.id.as_str()));
        }
    }
    Ok(())
}
