use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Token usage record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub session_id: String,
    pub model: String,
    pub provider: String,
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub total_tokens: u32,
    pub cost_usd: f64,
}

impl TokenUsage {
    pub fn new(
        session_id: String,
        model: String,
        provider: String,
        input_tokens: u32,
        output_tokens: u32,
        cost_usd: f64,
    ) -> Self {
        let total = input_tokens + output_tokens;
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            session_id,
            model,
            provider,
            input_tokens,
            output_tokens,
            total_tokens: total,
            cost_usd,
        }
    }
}

/// Performance metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetric {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub metric_type: MetricType,
    pub value: f64,
    pub unit: String,
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricType {
    LatencyMs,
    TokensPerSecond,
    CacheHitRate,
    RequestCount,
    ErrorRate,
    MemoryUsageMb,
    CpuUsagePercent,
}

impl PerformanceMetric {
    pub fn latency(timestamp: DateTime<Utc>, duration_ms: f64, operation: &str) -> Self {
        let mut tags = std::collections::HashMap::new();
        tags.insert("operation".to_string(), operation.to_string());

        Self {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp,
            metric_type: MetricType::LatencyMs,
            value: duration_ms,
            unit: "ms".to_string(),
            tags,
        }
    }

    pub fn throughput(timestamp: DateTime<Utc>, tokens_per_sec: f64) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp,
            metric_type: MetricType::TokensPerSecond,
            value: tokens_per_sec,
            unit: "tokens/s".to_string(),
            tags: std::collections::HashMap::new(),
        }
    }
}

/// Activity timeline entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineEntry {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub activity_type: ActivityType,
    pub description: String,
    pub duration_ms: Option<u64>,
    pub metadata: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActivityType {
    SessionStart,
    SessionEnd,
    MessageSent,
    MessageReceived,
    FileOperation,
    GitOperation,
    CommandExecuted,
    ModelSwitched,
}

impl TimelineEntry {
    pub fn session_start(session_id: &str) -> Self {
        let mut meta = std::collections::HashMap::new();
        meta.insert("session_id".to_string(), session_id.to_string());

        Self {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            activity_type: ActivityType::SessionStart,
            description: format!("Session {} started", session_id),
            duration_ms: None,
            metadata: meta,
        }
    }
}

/// Aggregated metrics summary
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MetricsSummary {
    pub total_tokens: u64,
    pub total_cost_usd: f64,
    pub total_requests: u64,
    pub avg_latency_ms: f64,
    pub avg_throughput: f64,
    pub cache_hit_rate: f64,
    pub error_count: u64,
}

impl MetricsSummary {
    pub fn calculate(usages: &[TokenUsage], metrics: &[PerformanceMetric]) -> Self {
        let total_tokens: u64 = usages.iter().map(|u| u.total_tokens as u64).sum();
        let total_cost: f64 = usages.iter().map(|u| u.cost_usd).sum();
        let total_requests = usages.len() as u64;

        let latencies: Vec<f64> = metrics
            .iter()
            .filter(|m| matches!(m.metric_type, MetricType::LatencyMs))
            .map(|m| m.value)
            .collect();
        let avg_latency = if latencies.is_empty() {
            0.0
        } else {
            latencies.iter().sum::<f64>() / latencies.len() as f64
        };

        let throughputs: Vec<f64> = metrics
            .iter()
            .filter(|m| matches!(m.metric_type, MetricType::TokensPerSecond))
            .map(|m| m.value)
            .collect();
        let avg_tp = if throughputs.is_empty() {
            0.0
        } else {
            throughputs.iter().sum::<f64>() / throughputs.len() as f64
        };

        let cache_rates: Vec<f64> = metrics
            .iter()
            .filter(|m| matches!(m.metric_type, MetricType::CacheHitRate))
            .map(|m| m.value)
            .collect();
        let cache_rate = if cache_rates.is_empty() {
            0.0
        } else {
            cache_rates.iter().sum::<f64>() / cache_rates.len() as f64
        };

        let errors = metrics
            .iter()
            .filter(|m| matches!(m.metric_type, MetricType::ErrorRate))
            .count() as u64;

        Self {
            total_tokens,
            total_cost_usd: total_cost,
            total_requests,
            avg_latency_ms: avg_latency,
            avg_throughput: avg_tp,
            cache_hit_rate: cache_rate,
            error_count: errors,
        }
    }
}
