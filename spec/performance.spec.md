# Performance Domain Tests

## Performance Metrics Models
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_performance_metrics_creation | Metrics created with 0 CPU usage and empty custom metrics | `PerformanceMetrics::new()` |
| ✅ | test_performance_snapshot_creation | Snapshot created with name and metrics | `PerformanceSnapshot::new("test", metrics)` |

## Performance Operations
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_analyze_performance | Empty metrics generate no suggestions | `analyze_performance(&empty_metrics)` |
| ✅ | test_analyze_performance_high_cpu | High CPU (90%) generates CPU-related suggestions | `analyze_performance(&high_cpu_metrics)` |
| ✅ | test_calculate_performance_score | Good metrics (30% CPU, 30% memory, 300ms response, 1% error) score > 0.8 | `calculate_performance_score(&good_metrics)` |

## Performance Validators
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_validate_performance_metrics_empty | Empty metrics pass validation | `validate_performance_metrics(&empty)` |
| ✅ | test_validate_performance_metrics_invalid_cpu | CPU usage > 100% fails validation | `validate_performance_metrics(&invalid_cpu)` |
| ✅ | test_validate_optimization_suggestion_empty_title | Suggestion with empty title fails validation | `validate_optimization_suggestion(&suggestion)` |
