use crate::modules::performance::domain::models::metrics::{
    OptimizationSuggestion, PerformanceMetrics,
};
use crate::shared::kernel::result::AppError;

/// Pure function to validate performance metrics
pub fn validate_performance_metrics(metrics: &PerformanceMetrics) -> Result<(), AppError> {
    if metrics.cpu_usage < 0.0 || metrics.cpu_usage > 100.0 {
        return Err(AppError::ValidationError(
            "CPU usage must be between 0 and 100".to_string(),
        ));
    }

    if metrics.memory_usage > metrics.memory_total {
        return Err(AppError::ValidationError(
            "Memory usage cannot exceed total memory".to_string(),
        ));
    }

    if metrics.error_rate < 0.0 || metrics.error_rate > 1.0 {
        return Err(AppError::ValidationError(
            "Error rate must be between 0 and 1".to_string(),
        ));
    }

    Ok(())
}

/// Pure function to validate optimization suggestion
pub fn validate_optimization_suggestion(
    suggestion: &OptimizationSuggestion,
) -> Result<(), AppError> {
    if suggestion.title.is_empty() {
        return Err(AppError::ValidationError(
            "Suggestion title cannot be empty".to_string(),
        ));
    }

    if suggestion.description.is_empty() {
        return Err(AppError::ValidationError(
            "Suggestion description cannot be empty".to_string(),
        ));
    }

    if suggestion.estimated_improvement < 0.0 || suggestion.estimated_improvement > 1.0 {
        return Err(AppError::ValidationError(
            "Estimated improvement must be between 0 and 1".to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_performance_metrics_invalid_cpu() {
        let mut metrics = PerformanceMetrics::new();
        metrics.cpu_usage = 150.0;
        assert!(validate_performance_metrics(&metrics).is_err());
    }

    #[test]
    fn test_validate_performance_metrics_success() {
        let metrics = PerformanceMetrics::new();
        assert!(validate_performance_metrics(&metrics).is_ok());
    }

    #[test]
    fn test_validate_optimization_suggestion_empty_title() {
        let suggestion = OptimizationSuggestion::new(
            crate::modules::performance::domain::models::metrics::OptimizationCategory::Caching,
            String::new(),
            "Test".to_string(),
            crate::modules::performance::domain::models::metrics::ImpactLevel::Medium,
            crate::modules::performance::domain::models::metrics::EffortLevel::Low,
            0.5,
        );
        assert!(validate_optimization_suggestion(&suggestion).is_err());
    }
}
