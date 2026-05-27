use agent_tui::modules::performance::domain::models::metrics::{
    PerformanceMetrics, PerformanceSnapshot
};

#[test]
fn test_performance_metrics_creation() {
    let metrics = PerformanceMetrics::new();
    assert_eq!(metrics.cpu_usage, 0.0);
    assert!(metrics.custom_metrics.is_empty());
}

#[test]
fn test_performance_snapshot_creation() {
    let metrics = PerformanceMetrics::new();
    let snapshot = PerformanceSnapshot::new("test-snapshot".to_string(), metrics);
    assert_eq!(snapshot.name, "test-snapshot");
}
