use agent_tui::modules::performance::domain::validators::performance_validators;
use agent_tui::modules::performance::domain::models::metrics::{PerformanceMetrics, OptimizationSuggestion, OptimizationCategory, ImpactLevel, EffortLevel};

#[test]
fn test_validate_performance_metrics_empty() {
    let metrics = PerformanceMetrics::new();
    assert!(performance_validators::validate_performance_metrics(&metrics).is_ok());
}

#[test]
fn test_validate_performance_metrics_invalid_cpu() {
    let mut metrics = PerformanceMetrics::new();
    metrics.cpu_usage = 150.0;
    assert!(performance_validators::validate_performance_metrics(&metrics).is_err());
}

#[test]
fn test_validate_optimization_suggestion_empty_title() {
    let suggestion = OptimizationSuggestion::new(
        OptimizationCategory::Caching,
        String::new(),
        "Test".to_string(),
        ImpactLevel::Medium,
        EffortLevel::Low,
        0.5,
    );
    assert!(performance_validators::validate_optimization_suggestion(&suggestion).is_err());
}
