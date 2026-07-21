// Headless handler - executes the ExecuteHeadlessUseCase for CLI/automation flows

use crate::modules::headless::domain::models::command::{HeadlessConfig, OutputFormat};
use crate::presentation::cli::output;
use crate::presentation::tui::di::DIContainer;
use crate::shared::kernel::result::AppResult;

pub(crate) async fn run(command: String, directory: String, format: String) -> AppResult<()> {
    output::print_section(&format!(
        "Executing headless command: {} in {}",
        command, directory
    ));

    let container = DIContainer::new().build().await?;

    let use_case = container.execute_headless_use_case().ok_or_else(|| {
        crate::shared::kernel::result::AppError::State(
            "Execute headless use case not available".to_string(),
        )
    })?;

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

    Ok(())
}

fn parse_output_format(format: &str) -> OutputFormat {
    match format {
        "json" => OutputFormat::Json,
        "markdown" => OutputFormat::Markdown,
        _ => OutputFormat::Text,
    }
}
