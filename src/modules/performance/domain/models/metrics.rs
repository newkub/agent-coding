use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Performance metrics entity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PerformanceMetrics {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub cpu_usage: f64,
    pub memory_usage: u64,
    pub memory_total: u64,
    pub response_time_ms: u64,
    pub throughput: f64,
    pub error_rate: f64,
    pub custom_metrics: HashMap<String, f64>,
}

/// Performance snapshot for comparison
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PerformanceSnapshot {
    pub id: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub metrics: PerformanceMetrics,
    pub baseline: Option<PerformanceMetrics>,
}

/// Performance optimization suggestion
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OptimizationSuggestion {
    pub id: String,
    pub category: OptimizationCategory,
    pub title: String,
    pub description: String,
    pub impact: ImpactLevel,
    pub effort: EffortLevel,
    pub estimated_improvement: f64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OptimizationCategory {
    Caching,
    AsyncOperations,
    MemoryOptimization,
    AlgorithmImprovement,
    DatabaseOptimization,
    NetworkOptimization,
    UIResponsiveness,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ImpactLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EffortLevel {
    Low,
    Medium,
    High,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl PerformanceMetrics {
    pub fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            cpu_usage: 0.0,
            memory_usage: 0,
            memory_total: 0,
            response_time_ms: 0,
            throughput: 0.0,
            error_rate: 0.0,
            custom_metrics: HashMap::new(),
        }
    }

    pub fn memory_usage_percentage(&self) -> f64 {
        if self.memory_total == 0 {
            0.0
        } else {
            (self.memory_usage as f64 / self.memory_total as f64) * 100.0
        }
    }

    pub fn is_healthy(&self) -> bool {
        self.cpu_usage < 80.0 && self.memory_usage_percentage() < 80.0 && self.error_rate < 0.05
    }
}

impl PerformanceSnapshot {
    pub fn new(name: String, metrics: PerformanceMetrics) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            created_at: Utc::now(),
            metrics,
            baseline: None,
        }
    }

    pub fn with_baseline(mut self, baseline: PerformanceMetrics) -> Self {
        self.baseline = Some(baseline);
        self
    }

    pub fn compare_with_baseline(&self) -> Option<PerformanceComparison> {
        let baseline = self.baseline.as_ref()?;
        Some(PerformanceComparison {
            cpu_diff: self.metrics.cpu_usage - baseline.cpu_usage,
            memory_diff: self.metrics.memory_usage as i64 - baseline.memory_usage as i64,
            response_time_diff: self.metrics.response_time_ms as i64
                - baseline.response_time_ms as i64,
            throughput_diff: self.metrics.throughput - baseline.throughput,
            error_rate_diff: self.metrics.error_rate - baseline.error_rate,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PerformanceComparison {
    pub cpu_diff: f64,
    pub memory_diff: i64,
    pub response_time_diff: i64,
    pub throughput_diff: f64,
    pub error_rate_diff: f64,
}

impl OptimizationSuggestion {
    pub fn new(
        category: OptimizationCategory,
        title: String,
        description: String,
        impact: ImpactLevel,
        effort: EffortLevel,
        estimated_improvement: f64,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            category,
            title,
            description,
            impact,
            effort,
            estimated_improvement,
            created_at: Utc::now(),
        }
    }

    pub fn priority_score(&self) -> f64 {
        let impact_score = match self.impact {
            ImpactLevel::Low => 1.0,
            ImpactLevel::Medium => 2.0,
            ImpactLevel::High => 3.0,
            ImpactLevel::Critical => 4.0,
        };

        let effort_penalty = match self.effort {
            EffortLevel::Low => 0.0,
            EffortLevel::Medium => 0.5,
            EffortLevel::High => 1.0,
        };

        (impact_score - effort_penalty) * self.estimated_improvement
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_performance_metrics_creation() {
        let metrics = PerformanceMetrics::new();
        assert_eq!(metrics.cpu_usage, 0.0);
        assert!(metrics.is_healthy());
    }

    #[test]
    fn test_memory_usage_percentage() {
        let mut metrics = PerformanceMetrics::new();
        metrics.memory_usage = 400;
        metrics.memory_total = 1000;
        assert_eq!(metrics.memory_usage_percentage(), 40.0);
    }

    #[test]
    fn test_is_healthy() {
        let mut metrics = PerformanceMetrics::new();
        metrics.cpu_usage = 90.0;
        assert!(!metrics.is_healthy());
    }

    #[test]
    fn test_optimization_suggestion_priority() {
        let suggestion = OptimizationSuggestion::new(
            OptimizationCategory::Caching,
            "Add caching".to_string(),
            "Cache frequently accessed data".to_string(),
            ImpactLevel::High,
            EffortLevel::Low,
            0.5,
        );
        assert!(suggestion.priority_score() > 0.0);
    }
}
