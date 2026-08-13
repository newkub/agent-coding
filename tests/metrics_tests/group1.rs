//! Metrics domain tests - Group 1

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
