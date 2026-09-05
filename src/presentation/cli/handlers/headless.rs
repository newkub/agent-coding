// Headless handler - executes the ExecuteHeadlessUseCase for CLI/automation flows

use crate::modules::headless::domain::models::command::{HeadlessConfig, OutputFormat};
use crate::presentation::cli::commands::HeadlessCommands;
use crate::presentation::cli::output;
use crate::presentation::tui::di::DIContainer;
use crate::shared::kernel::result::{AppError, AppResult};

pub(crate) async fn run(command: HeadlessCommands) -> AppResult<()> {
    let mut container = DIContainer::new().build().await?;
    container.init_db().await?;

    let use_case = container
        .execute_headless_use_case()
        .ok_or_else(|| AppError::State("Execute headless use case not available".to_string()))?;

    match command {
        HeadlessCommands::Execute {
            command,
            directory,
            format,
        } => {
            output::print_section(&format!(
                "Executing headless command: {} in {}",
                command, directory
            ));

            let output_format = parse_output_format(&format);
            let config = HeadlessConfig {
                output_format,
                ..HeadlessConfig::default()
            };

            match use_case.execute(command, directory, &config).await {
                Ok(cmd) => {
                    if let Some(result) = cmd.output {
                        println!("{}", result);
                    }
                }
                Err(e) => {
                    output::print_error(&format!("Error executing command: {}", e));
                }
            }
        }
        HeadlessCommands::List => {
            output::print_section("Headless sessions");
            let sessions = use_case.list_sessions().await?;
            output::print_headless_sessions(&sessions);
        }
        HeadlessCommands::Create => {
            output::print_section("Creating headless session");
            let id = use_case.create_session().await?;
            output::print_info(&format!("Created headless session: {id}"));
        }
        HeadlessCommands::Delete { id } => {
            output::print_section(&format!("Deleting headless session: {id}"));
            use_case.delete_session(&id).await?;
            output::print_info(&format!("Deleted headless session: {id}"));
        }
        HeadlessCommands::Load { id } => {
            output::print_section(&format!("Loading headless session: {id}"));
            use_case.load_session(&id).await?;
            output::print_info(&format!("Loaded headless session: {id}"));
        }
        HeadlessCommands::Save { id } => {
            output::print_section(&format!("Saving headless session: {id}"));
            use_case.save_session(&id).await?;
            output::print_info(&format!("Saved headless session: {id}"));
        }
        HeadlessCommands::Batch { commands } => {
            output::print_section(&format!("Executing {} headless commands", commands.len()));
            let config = HeadlessConfig::default();
            match use_case
                .execute_batch(commands, ".".to_string(), &config)
                .await
            {
                Ok(results) => {
                    for result in results {
                        if let Some(out) = result.output {
                            println!("{}", out);
                        }
                    }
                }
                Err(e) => {
                    output::print_error(&format!("Error executing batch: {}", e));
                }
            }
        }
    }

    Ok(())
}

fn parse_output_format(format: &str) -> OutputFormat {
    match format {
        "json" => OutputFormat::Json,
        "markdown" => OutputFormat::Markdown,
        _ => OutputFormat::Text,
    }
}
