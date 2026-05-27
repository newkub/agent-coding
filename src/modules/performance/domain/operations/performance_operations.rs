use crate::modules::performance::domain::models::metrics::{PerformanceMetrics, OptimizationSuggestion, OptimizationCategory, ImpactLevel, EffortLevel};

/// Pure function to analyze performance metrics and generate suggestions
pub fn analyze_performance(metrics: &PerformanceMetrics) -> Vec<OptimizationSuggestion> {
    let mut suggestions = Vec::new();

    // CPU usage analysis
    if metrics.cpu_usage > 80.0 {
        suggestions.push(OptimizationSuggestion::new(
            OptimizationCategory::AsyncOperations,
            "Reduce CPU usage".to_string(),
            "Consider optimizing hot paths and using async operations".to_string(),
            ImpactLevel::High,
            EffortLevel::Medium,
            0.3,
        ));
    }

    // Memory usage analysis
    if metrics.memory_usage_percentage() > 80.0 {
        suggestions.push(OptimizationSuggestion::new(
            OptimizationCategory::MemoryOptimization,
            "Reduce memory usage".to_string(),
            "Consider using more efficient data structures and releasing unused resources".to_string(),
            ImpactLevel::High,
            EffortLevel::Medium,
            0.25,
        ));
    }

    // Response time analysis
    if metrics.response_time_ms > 1000 {
        suggestions.push(OptimizationSuggestion::new(
            OptimizationCategory::Caching,
            "Improve response time".to_string(),
            "Implement caching for frequently accessed data".to_string(),
            ImpactLevel::High,
            EffortLevel::Low,
            0.4,
        ));
    }

    // Error rate analysis
    if metrics.error_rate > 0.05 {
        suggestions.push(OptimizationSuggestion::new(
            OptimizationCategory::AlgorithmImprovement,
            "Reduce error rate".to_string(),
            "Improve error handling and add better validation".to_string(),
            ImpactLevel::Critical,
            EffortLevel::Medium,
            0.5,
        ));
    }

    // Throughput analysis
    if metrics.throughput < 10.0 {
        suggestions.push(OptimizationSuggestion::new(
            OptimizationCategory::AsyncOperations,
            "Increase throughput".to_string(),
            "Use parallel processing and batch operations".to_string(),
            ImpactLevel::Medium,
            EffortLevel::High,
            0.2,
        ));
    }

    suggestions
}

/// Pure function to calculate performance score
pub fn calculate_performance_score(metrics: &PerformanceMetrics) -> f64 {
    let cpu_score = if metrics.cpu_usage < 50.0 {
        1.0
    } else if metrics.cpu_usage < 80.0 {
        0.7
    } else {
        0.3
    };

    let memory_score = if metrics.memory_usage_percentage() < 50.0 {
        1.0
    } else if metrics.memory_usage_percentage() < 80.0 {
        0.7
    } else {
        0.3
    };

    let response_score = if metrics.response_time_ms < 500 {
        1.0
    } else if metrics.response_time_ms < 1000 {
        0.7
    } else {
        0.3
    };

    let error_score = if metrics.error_rate < 0.01 {
        1.0
    } else if metrics.error_rate < 0.05 {
        0.7
    } else {
        0.3
    };

    (cpu_score + memory_score + response_score + error_score) / 4.0
}

/// Pure function to sort suggestions by priority
pub fn sort_suggestions_by_priority(suggestions: &mut [OptimizationSuggestion]) {
    suggestions.sort_by(|a, b| {
        b.priority_score()
            .partial_cmp(&a.priority_score())
            .unwrap_or(std::cmp::Ordering::Equal)
    });
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_sort_suggestions_by_priority() {
        let mut suggestions = vec![
            OptimizationSuggestion::new(
                OptimizationCategory::Caching,
                "Low priority".to_string(),
                "Test".to_string(),
                ImpactLevel::Low,
                EffortLevel::High,
                0.1,
            ),
            OptimizationSuggestion::new(
                OptimizationCategory::Caching,
                "High priority".to_string(),
                "Test".to_string(),
                ImpactLevel::High,
                EffortLevel::Low,
                0.5,
            ),
        ];
        
        sort_suggestions_by_priority(&mut suggestions);
        assert!(suggestions[0].title.contains("High"));
    }
}
