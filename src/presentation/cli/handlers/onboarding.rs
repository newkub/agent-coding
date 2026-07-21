// Onboarding handler - executes the AnalyzeCodebaseUseCase and prints results
// Demonstrates the Functional Core / Imperative Shell pattern:
//   - Build the DI container (imperative)
//   - Hand control to the use case (functional core via ports)
//   - Render the result (imperative shell)

use std::path::PathBuf;

use crate::presentation::cli::output;
use crate::presentation::tui::di::DIContainer;
use crate::shared::kernel::result::AppResult;

pub(crate) async fn run(path: String) -> AppResult<()> {
    output::print_section(&format!("Analyzing codebase at: {}", path));

    let container = DIContainer::new().build().await?;

    let use_case = container.analyze_codebase_use_case().ok_or_else(|| {
        crate::shared::kernel::result::AppError::State(
            "Analyze codebase use case not available".to_string(),
        )
    })?;

    let project_path = PathBuf::from(&path);
    match use_case.execute(project_path).await {
        Ok(analysis) => output::print_codebase_analysis(&analysis),
        Err(e) => {
            output::print_error(&format!("Error analyzing codebase: {}", e));
        }
    }

    Ok(())
}
