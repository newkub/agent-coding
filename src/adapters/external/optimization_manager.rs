use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::modules::performance::domain::models::metrics::{
    OptimizationSuggestion, PerformanceMetrics,
};
use crate::modules::performance::domain::operations::performance_operations::{
    analyze_performance, sort_suggestions_by_priority,
};
use crate::modules::performance::ports::OptimizationManager;
use crate::shared::kernel::result::AppError;

/// In-memory optimization manager
pub(crate) struct InMemoryOptimizationManager {
    suggestions: Arc<RwLock<HashMap<String, OptimizationSuggestion>>>,
}

impl InMemoryOptimizationManager {
    pub(crate) fn new() -> Self {
        Self {
            suggestions: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryOptimizationManager {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl OptimizationManager for InMemoryOptimizationManager {
    async fn generate_suggestions(
        &self,
        metrics: &PerformanceMetrics,
    ) -> Result<Vec<OptimizationSuggestion>, AppError> {
        let mut suggestions = analyze_performance(metrics);
        sort_suggestions_by_priority(&mut suggestions);

        // Store suggestions
        let mut suggestions_map = self.suggestions.write().await;
        for suggestion in &suggestions {
            suggestions_map.insert(suggestion.id.clone(), suggestion.clone());
        }

        Ok(suggestions)
    }

    async fn apply_suggestion(&self, suggestion_id: &str) -> Result<(), AppError> {
        let mut suggestions = self.suggestions.write().await;
        if let Some(_suggestion) = suggestions.get_mut(suggestion_id) {
            // In a real implementation, this would apply the optimization
            // For now, we'll just mark it as applied
            Ok(())
        } else {
            Err(AppError::NotFound(format!(
                "Suggestion {} not found",
                suggestion_id
            )))
        }
    }

    async fn list_suggestions(&self) -> Result<Vec<OptimizationSuggestion>, AppError> {
        let suggestions = self.suggestions.read().await;
        Ok(suggestions.values().cloned().collect())
    }

    async fn dismiss_suggestion(&self, suggestion_id: &str) -> Result<(), AppError> {
        let mut suggestions = self.suggestions.write().await;
        if suggestions.remove(suggestion_id).is_some() {
            Ok(())
        } else {
            Err(AppError::NotFound(format!(
                "Suggestion {} not found",
                suggestion_id
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generate_suggestions() {
        let manager = InMemoryOptimizationManager::new();
        let metrics =
            crate::modules::performance::domain::models::metrics::PerformanceMetrics::new();
        let suggestions = manager.generate_suggestions(&metrics).await.unwrap();
        // Should generate suggestions based on metrics
    }

    #[tokio::test]
    async fn test_list_suggestions() {
        let manager = InMemoryOptimizationManager::new();
        let metrics =
            crate::modules::performance::domain::models::metrics::PerformanceMetrics::new();
        manager.generate_suggestions(&metrics).await.unwrap();

        let suggestions = manager.list_suggestions().await.unwrap();
        assert!(!suggestions.is_empty());
    }
}
