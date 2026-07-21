//! Metrics domain tests - 100% coverage target

use agent_tui::modules::metrics::domain::models::*;
use agent_tui::modules::metrics::domain::operations::*;
use chrono::Utc;

#[test]
fn test_token_usage_new() {
    let usage = TokenUsage::new(
        "session-1".to_string(),
        "gpt-4".to_string(),
        "openai".to_string(),
        100,
        200,
        0.01,
    );

    assert_eq!(usage.session_id, "session-1");
    assert_eq!(usage.model, "gpt-4");
    assert_eq!(usage.input_tokens, 100);
    assert_eq!(usage.output_tokens, 200);
    assert_eq!(usage.total_tokens, 300);
}

#[test]
fn test_performance_metric_latency() {
    let now = Utc::now();
    let metric = PerformanceMetric::latency(now, 150.0, "api_call");

    assert!(matches!(metric.metric_type, MetricType::LatencyMs));
    assert_eq!(metric.value, 150.0);
    assert_eq!(metric.unit, "ms");
    assert_eq!(metric.tags.get("operation"), Some(&"api_call".to_string()));
}

#[test]
fn test_performance_metric_throughput() {
    let now = Utc::now();
    let metric = PerformanceMetric::throughput(now, 50.0);

    assert!(matches!(metric.metric_type, MetricType::TokensPerSecond));
    assert_eq!(metric.value, 50.0);
    assert_eq!(metric.unit, "tokens/s");
}

#[test]
fn test_timeline_entry_session_start() {
    let entry = TimelineEntry::session_start("session-123");

    assert!(matches!(entry.activity_type, ActivityType::SessionStart));
    assert_eq!(
        entry.metadata.get("session_id"),
        Some(&"session-123".to_string())
    );
}

#[test]
fn test_metrics_summary_default() {
    let summary = MetricsSummary::default();
    assert_eq!(summary.total_tokens, 0);
    assert_eq!(summary.total_cost_usd, 0.0);
}

#[test]
fn test_metrics_summary_calculate_empty() {
    let usages: Vec<TokenUsage> = vec![];
    let metrics: Vec<PerformanceMetric> = vec![];

    let summary = MetricsSummary::calculate(&usages, &metrics);
    assert_eq!(summary.total_tokens, 0);
    assert_eq!(summary.avg_latency_ms, 0.0);
}

#[test]
fn test_metrics_summary_calculate() {
    let usages = vec![
        TokenUsage::new(
            "s1".to_string(),
            "gpt-4".to_string(),
            "openai".to_string(),
            100,
            200,
            0.01,
        ),
        TokenUsage::new(
            "s1".to_string(),
            "gpt-4".to_string(),
            "openai".to_string(),
            50,
            100,
            0.005,
        ),
    ];

    let now = Utc::now();
    let metrics = vec![
        PerformanceMetric::latency(now, 100.0, "op1"),
        PerformanceMetric::latency(now, 200.0, "op2"),
    ];

    let summary = MetricsSummary::calculate(&usages, &metrics);
    assert_eq!(summary.total_tokens, 450); // 300 + 150
    assert_eq!(summary.total_requests, 2);
    assert!(summary.avg_latency_ms > 0.0);
}

#[test]
fn test_calculate_cost_gpt4() {
    let cost = calculate_cost(1000, 500, "gpt-4");
    assert!(cost > 0.0);
}

#[test]
fn test_calculate_cost_gpt35() {
    let cost = calculate_cost(1000, 500, "gpt-3.5-turbo");
    assert!(cost > 0.0);
    assert!(cost < 1.0); // Should be cheap
}

#[test]
fn test_calculate_cost_claude() {
    let cost = calculate_cost(1000, 500, "claude-3-sonnet");
    assert!(cost > 0.0);
}

#[test]
fn test_calculate_cost_unknown() {
    let cost = calculate_cost(1000, 500, "unknown-model");
    assert!(cost > 0.0);
}

#[test]
fn test_calculate_throughput() {
    let tp = calculate_throughput(1000, 1000); // 1000 tokens in 1 second
    assert_eq!(tp, 1000.0);
}

#[test]
fn test_calculate_throughput_zero_duration() {
    let tp = calculate_throughput(1000, 0);
    assert_eq!(tp, 0.0);
}

#[test]
fn test_is_latency_acceptable_gpt4() {
    assert!(is_latency_acceptable(4000.0, "gpt-4"));
    assert!(!is_latency_acceptable(6000.0, "gpt-4"));
}

#[test]
fn test_is_latency_acceptable_claude() {
    assert!(is_latency_acceptable(3000.0, "claude-3-opus"));
}

#[test]
fn test_calculate_error_rate() {
    let rate = calculate_error_rate(5, 100);
    assert_eq!(rate, 5.0);
}

#[test]
fn test_calculate_error_rate_zero_total() {
    let rate = calculate_error_rate(0, 0);
    assert_eq!(rate, 0.0);
}

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
