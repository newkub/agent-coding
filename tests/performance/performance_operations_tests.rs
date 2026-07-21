use agent_tui::modules::performance::domain::models::metrics::{
    EffortLevel, ImpactLevel, OptimizationCategory, PerformanceMetrics,
};
use agent_tui::modules::performance::domain::operations::performance_operations::{
    analyze_performance, calculate_performance_score, sort_suggestions_by_priority,
};

#[test]
fn test_analyze_performance() {
    let mut metrics = PerformanceMetrics::new();
    metrics.cpu_usage = 10.0;
    metrics.memory_usage = 100;
    metrics.memory_total = 1000;
    metrics.response_time_ms = 100;
    metrics.error_rate = 0.001;
    metrics.throughput = 100.0;
    let suggestions = analyze_performance(&metrics);
    // Low metrics should not generate suggestions
    assert!(suggestions.is_empty());
}

#[test]
fn test_analyze_performance_high_cpu() {
    let mut metrics = PerformanceMetrics::new();
    metrics.cpu_usage = 90.0;
    let suggestions = analyze_performance(&metrics);
    assert!(!suggestions.is_empty());
    assert!(suggestions.iter().any(|s| s.title.contains("CPU")));
}

#[test]
fn test_calculate_performance_score() {
    let mut metrics = PerformanceMetrics::new();
    metrics.cpu_usage = 30.0;
    metrics.memory_usage = 300;
    metrics.memory_total = 1000;
    metrics.response_time_ms = 300;
    metrics.error_rate = 0.01;

    let score = calculate_performance_score(&metrics);
    assert!(score > 0.8);
}
