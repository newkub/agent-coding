# Metrics Domain Tests

## TokenUsage
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_token_usage_new | Creates usage with session and token counts | `TokenUsage::new("session-1", 100, 50)` |
| ✅ | test_token_usage_serialization | Usage serializes/deserializes correctly | `serde_json::to_string(&usage)` |

## PerformanceMetric
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_performance_metric_latency | Creates latency metric | `PerformanceMetric::latency(100)` |
| ✅ | test_performance_metric_throughput | Creates throughput metric | `PerformanceMetric::throughput(1000)` |
| ✅ | test_performance_metric_serialization | Metric serializes/deserializes correctly | `serde_json::to_string(&metric)` |
| ✅ | test_performance_metric_with_tags | Metric can have tags | `metric.with_tags(vec!["tag1".to_string()])` |

## TimelineEntry
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_timeline_entry_session_start | Creates session start entry | `TimelineEntry::session_start("session-1")` |
| ✅ | test_timeline_entry_serialization | Entry serializes/deserializes correctly | `serde_json::to_string(&entry)` |

## MetricsSummary
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_metrics_summary_default | Default summary is empty | `MetricsSummary::default()` |
| ✅ | test_metrics_summary_calculate_empty | Empty data returns empty summary | `MetricsSummary::calculate(&[])` |
| ✅ | test_metrics_summary_calculate | Calculates summary from data | `MetricsSummary::calculate(&data)` |
| ✅ | test_metrics_summary_serialization | Summary serializes/deserializes correctly | `serde_json::to_string(&summary)` |

## Metrics Operations
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_calculate_cost_gpt4 | Calculates GPT-4 cost | `calculate_cost("gpt-4", 1000, 500)` |
| ✅ | test_calculate_cost_gpt35 | Calculates GPT-3.5 cost | `calculate_cost("gpt-3.5", 1000, 500)` |
| ✅ | test_calculate_cost_claude | Calculates Claude cost | `calculate_cost("claude", 1000, 500)` |
| ✅ | test_calculate_cost_unknown | Calculates unknown model cost | `calculate_cost("unknown", 1000, 500)` |
| ✅ | test_calculate_throughput | Calculates throughput | `calculate_throughput(100, 10)` |
| ✅ | test_calculate_throughput_zero_duration | Zero duration returns zero throughput | `calculate_throughput(100, 0)` |
| ✅ | test_is_latency_acceptable_gpt4 | Checks GPT-4 latency acceptance | `is_latency_acceptable("gpt-4", 100)` |
| ✅ | test_is_latency_acceptable_claude | Checks Claude latency acceptance | `is_latency_acceptable("claude", 100)` |
| ✅ | test_calculate_error_rate | Calculates error rate | `calculate_error_rate(5, 100)` |
| ✅ | test_calculate_error_rate_zero_total | Zero total returns zero rate | `calculate_error_rate(5, 0)` |

## Variants
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_metric_type_variants | All metric type variants match | `assert!(matches!(MetricType::Latency, MetricType::Latency))` |
| ✅ | test_activity_type_variants | All activity type variants match | `assert!(matches!(ActivityType::SessionStart, ActivityType::SessionStart))` |
