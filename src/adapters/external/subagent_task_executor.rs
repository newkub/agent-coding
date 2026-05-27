use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::modules::subagents::domain::models::subagent::{SubagentTask, TaskStatus};
use crate::modules::subagents::ports::SubagentTaskExecutor;
use crate::shared::kernel::result::AppError;

/// Generate task output (placeholder for AI provider integration)
fn generate_task_output(task: &SubagentTask) -> String {
    // TODO: Implement actual AI provider integration
    // For now, return a placeholder response
    // This should be replaced with actual AI API calls
    match &task.task_type {
        crate::modules::subagents::domain::models::subagent::TaskType::CodeReview => {
            "Code review completed: No major issues found. Consider adding more tests.".to_string()
        }
        crate::modules::subagents::domain::models::subagent::TaskType::BugDetection => {
            "Bug detection completed: Found 2 potential issues in line 42 and 87.".to_string()
        }
        crate::modules::subagents::domain::models::subagent::TaskType::Refactoring => {
            "Refactoring suggestions: Extract function for repeated logic, use enum instead of string.".to_string()
        }
        crate::modules::subagents::domain::models::subagent::TaskType::Documentation => {
            "Documentation generated: Added function docs, module overview, and usage examples.".to_string()
        }
        crate::modules::subagents::domain::models::subagent::TaskType::TestGeneration => {
            "Test cases generated: 15 unit tests, 3 integration tests, 5 edge case scenarios.".to_string()
        }
        crate::modules::subagents::domain::models::subagent::TaskType::SecurityAudit => {
            "Security audit completed: 1 medium severity issue found (SQL injection risk).".to_string()
        }
        crate::modules::subagents::domain::models::subagent::TaskType::PerformanceAnalysis => {
            "Performance analysis: O(n^2) complexity detected, suggest using HashMap for O(1) lookup.".to_string()
        }
        crate::modules::subagents::domain::models::subagent::TaskType::DependencyUpdate => {
            "Dependency update: 3 packages outdated, 1 security vulnerability found.".to_string()
        }
        crate::modules::subagents::domain::models::subagent::TaskType::Custom(_) => {
            "Custom task completed successfully.".to_string()
        }
    }
}

/// Default subagent task executor
pub struct DefaultSubagentTaskExecutor {
    tasks: Arc<RwLock<HashMap<String, SubagentTask>>>,
}

impl DefaultSubagentTaskExecutor {
    pub(crate) fn new() -> Self {
        Self {
            tasks: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for DefaultSubagentTaskExecutor {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl SubagentTaskExecutor for DefaultSubagentTaskExecutor {
    async fn execute_task(&self, task: &mut SubagentTask) -> Result<(), AppError> {
        // Update status to processing
        task.status = TaskStatus::Processing;

        // Store task
        let mut tasks = self.tasks.write().await;
        tasks.insert(task.id.clone(), task.clone());
        drop(tasks);

        // Execute task using AI service
        // TODO: Integrate with actual AI provider (OpenAI, Anthropic, etc.)
        let output = generate_task_output(task);

        task.complete(output);

        // Update stored task
        let mut tasks = self.tasks.write().await;
        tasks.insert(task.id.clone(), task.clone());

        Ok(())
    }

    async fn execute_tasks_parallel(&self, tasks: &mut [SubagentTask]) -> Result<(), AppError> {
        let mut handles = Vec::new();

        for i in 0..tasks.len() {
            let task_id = tasks[i].id.clone();
            let mut task = tasks[i].clone();
            let executor = self.clone();
            
            let handle = tokio::spawn(async move {
                if let Err(e) = executor.execute_task(&mut task).await {
                    Err(e)
                } else {
                    Ok(task)
                }
            });
            handles.push((task_id, handle));
        }

        for (task_id, handle) in handles {
            let task = handle.await.map_err(|e| AppError::State(format!("Task execution error: {}", e)))??;
            let mut tasks = self.tasks.write().await;
            tasks.insert(task_id, task);
        }

        Ok(())
    }

    async fn execute_tasks_sequential(&self, tasks: &mut [SubagentTask]) -> Result<(), AppError> {
        for task in tasks {
            self.execute_task(task).await?;
        }
        Ok(())
    }

    async fn cancel_task(&self, task_id: &str) -> Result<(), AppError> {
        let mut tasks = self.tasks.write().await;
        if let Some(task) = tasks.get_mut(task_id) {
            task.status = TaskStatus::Cancelled;
            task.completed_at = Some(chrono::Utc::now());
            Ok(())
        } else {
            Err(AppError::NotFound(format!("Task {} not found", task_id)))
        }
    }

    async fn get_task_status(&self, task_id: &str) -> Result<SubagentTask, AppError> {
        let tasks = self.tasks.read().await;
        tasks
            .get(task_id)
            .cloned()
            .ok_or_else(|| AppError::NotFound(format!("Task {} not found", task_id)))
    }
}

impl Clone for DefaultSubagentTaskExecutor {
    fn clone(&self) -> Self {
        Self {
            tasks: Arc::clone(&self.tasks),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::subagents::domain::models::subagent::{TaskContext, TaskType};

    #[tokio::test]
    async fn test_execute_task() {
        let executor = DefaultSubagentTaskExecutor::new();
        let context = TaskContext::new();
        let mut task = SubagentTask::new(
            "agent-1".to_string(),
            TaskType::CodeReview,
            "Review this code".to_string(),
            context,
        );
        
        let result = executor.execute_task(&mut task).await;
        assert!(result.is_ok());
        assert_eq!(task.status, TaskStatus::Completed);
    }

    #[tokio::test]
    async fn test_cancel_task() {
        let executor = DefaultSubagentTaskExecutor::new();
        let context = TaskContext::new();
        let task = SubagentTask::new(
            "agent-1".to_string(),
            TaskType::CodeReview,
            "Review this code".to_string(),
            context,
        );
        
        let mut tasks = executor.tasks.write().await;
        let task_id = task.id.clone();
        tasks.insert(task_id.clone(), task);
        drop(tasks);
        
        let result = executor.cancel_task(&task_id).await;
        assert!(result.is_ok());
    }
}
