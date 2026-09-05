// Performance handler - delegates to the AnalyzePerformanceUseCase
// obtained from the DI container (SystemMetricsCollector +
// InMemorySnapshotManager + InMemoryOptimizationManager) and renders the result.

use crate::presentation::cli::output;
use crate::presentation::tui::di::DIContainer;
use crate::shared::kernel::result::{AppError, AppResult};

pub(crate) async fn run(action: String) -> AppResult<()> {
    output::print_section(&format!("Performance action: {}", action));

    let container = DIContainer::new().build().await?;
    let use_case = container
        .analyze_performance_use_case()
        .ok_or_else(|| AppError::State("Analyze performance use case not available".to_string()))?;

    match action.as_str() {
        "analyze" => match use_case.analyze_current().await {
            Ok(result) => output::print_performance_analysis(&result),
            Err(e) => output::print_error(&format!("Error analyzing performance: {}", e)),
        },
        "snapshot" => match use_case.create_snapshot("manual".to_string()).await {
            Ok(snapshot) => output::print_performance_snapshot(&snapshot),
            Err(e) => output::print_error(&format!("Error creating snapshot: {}", e)),
        },
        "report" => match use_case.analyze_current().await {
            Ok(result) => output::print_performance_report(&result),
            Err(e) => output::print_error(&format!("Error generating report: {}", e)),
        },
        "list" => match use_case.list_snapshots().await {
            Ok(snapshots) => output::print_snapshot_list(&snapshots),
            Err(e) => output::print_error(&format!("Error listing snapshots: {}", e)),
        },
        "suggestions" => match use_case.get_suggestions().await {
            Ok(suggestions) => output::print_suggestions(&suggestions),
            Err(e) => output::print_error(&format!("Error listing suggestions: {}", e)),
        },
        other => output::print_error(&format!("Unknown action: {}", other)),
    }

    Ok(())
}
