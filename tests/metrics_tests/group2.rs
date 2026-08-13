//! Metrics domain tests - Group 2

use agent_tui::modules::metrics::domain::models::*;
use chrono::Utc;

#[test]
fn test_metric_type_variants() {
    assert!(matches!(MetricType::LatencyMs, MetricType::LatencyMs));
    assert!(matches!(
        MetricType::TokensPerSecond,
        MetricType::TokensPerSecond
    ));
    assert!(matches!(MetricType::CacheHitRate, MetricType::CacheHitRate));
    assert!(matches!(MetricType::RequestCount, MetricType::RequestCount));
    assert!(matches!(MetricType::ErrorRate, MetricType::ErrorRate));
    assert!(matches!(
        MetricType::MemoryUsageMb,
        MetricType::MemoryUsageMb
    ));
    assert!(matches!(
        MetricType::CpuUsagePercent,
        MetricType::CpuUsagePercent
    ));
}

#[test]
fn test_activity_type_variants() {
    assert!(matches!(
        ActivityType::SessionStart,
        ActivityType::SessionStart
    ));
    assert!(matches!(ActivityType::SessionEnd, ActivityType::SessionEnd));
    assert!(matches!(
        ActivityType::MessageSent,
        ActivityType::MessageSent
    ));
    assert!(matches!(
        ActivityType::MessageReceived,
        ActivityType::MessageReceived
    ));
    assert!(matches!(
        ActivityType::FileOperation,
        ActivityType::FileOperation
    ));
    assert!(matches!(
        ActivityType::GitOperation,
        ActivityType::GitOperation
    ));
    assert!(matches!(
        ActivityType::CommandExecuted,
        ActivityType::CommandExecuted
    ));
    assert!(matches!(
        ActivityType::ModelSwitched,
        ActivityType::ModelSwitched
    ));
}

#[test]
fn test_token_usage_serialization() {
    let usage = TokenUsage::new(
        "session-1".to_string(),
        "gpt-4".to_string(),
        "openai".to_string(),
        100,
        200,
        0.01,
    );

    let json = serde_json::to_string(&usage).unwrap();
    let parsed: TokenUsage = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.total_tokens, 300);
}

#[test]
fn test_performance_metric_serialization() {
    let metric = PerformanceMetric::latency(Utc::now(), 100.0, "test");

    let json = serde_json::to_string(&metric).unwrap();
    let parsed: PerformanceMetric = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.value, 100.0);
}

#[test]
fn test_metrics_summary_serialization() {
    let summary = MetricsSummary {
        total_tokens: 1000,
        total_cost_usd: 0.05,
        total_requests: 10,
        avg_latency_ms: 150.0,
        avg_throughput: 500.0,
        cache_hit_rate: 0.8,
        error_count: 1,
    };

    let json = serde_json::to_string(&summary).unwrap();
    let parsed: MetricsSummary = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.total_tokens, 1000);
}

#[test]
fn test_timeline_entry_serialization() {
    let entry = TimelineEntry {
        id: "test".to_string(),
        timestamp: Utc::now(),
        activity_type: ActivityType::SessionStart,
        description: "Session started".to_string(),
        duration_ms: Some(100),
        metadata: std::collections::HashMap::new(),
    };

    let json = serde_json::to_string(&entry).unwrap();
    let parsed: TimelineEntry = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.description, "Session started");
}

#[test]
fn test_performance_metric_with_tags() {
    let mut tags = std::collections::HashMap::new();
    tags.insert("key1".to_string(), "value1".to_string());

    let metric = PerformanceMetric {
        id: "test".to_string(),
        timestamp: Utc::now(),
        metric_type: MetricType::LatencyMs,
        value: 100.0,
        unit: "ms".to_string(),
        tags,
    };

    assert_eq!(metric.tags.get("key1"), Some(&"value1".to_string()));
}
