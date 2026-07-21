// Performance handler - delegates to the AnalyzePerformanceUseCase
// Wires the concrete adapters (metrics collector, snapshot manager, optimization manager)
// through the use case and renders the result.

use crate::adapters::external::metrics_collector::SystemMetricsCollector;
use crate::adapters::external::optimization_manager::InMemoryOptimizationManager;
use crate::adapters::external::snapshot_manager::InMemorySnapshotManager;
use crate::modules::performance::application::usecases::analyze_performance::AnalyzePerformanceUseCase;
use crate::modules::performance::ports::MetricsCollector;
use crate::presentation::cli::output;
use crate::shared::kernel::result::AppResult;

pub(crate) async fn run(action: String) -> AppResult<()> {
    output::print_section(&format!("Performance action: {}", action));

    let collector = SystemMetricsCollector::new();
    let snapshot_manager = InMemorySnapshotManager::new();
    let optimization_manager = InMemoryOptimizationManager::new();
    let use_case =
        AnalyzePerformanceUseCase::new(collector, snapshot_manager, optimization_manager);

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
        other => output::print_error(&format!("Unknown action: {}", other)),
    }

    Ok(())
}
