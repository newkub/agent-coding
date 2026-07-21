use async_trait::async_trait;

use crate::modules::subagents::domain::models::subagent::{Subagent, SubagentTask};
use crate::shared::kernel::result::AppError;

/// Port for subagent management
#[async_trait]
pub trait SubagentManager: Send + Sync {
    /// Create a new subagent
    async fn create_subagent(&self, subagent: Subagent) -> Result<Subagent, AppError>;

    /// Get subagent by ID
    async fn get_subagent(&self, id: &str) -> Result<Subagent, AppError>;

    /// List all subagents
    async fn list_subagents(&self) -> Result<Vec<Subagent>, AppError>;

    /// Update subagent
    async fn update_subagent(&self, subagent: Subagent) -> Result<Subagent, AppError>;

    /// Delete subagent
    async fn delete_subagent(&self, id: &str) -> Result<(), AppError>;

    /// Get available subagents for task
    async fn get_available_subagents(&self) -> Result<Vec<Subagent>, AppError>;
}

/// Port for subagent task execution
#[async_trait]
pub trait SubagentTaskExecutor: Send + Sync {
    /// Execute a subagent task
    async fn execute_task(&self, task: &mut SubagentTask) -> Result<(), AppError>;

    /// Execute multiple tasks in parallel
    async fn execute_tasks_parallel(&self, tasks: &mut [SubagentTask]) -> Result<(), AppError>;

    /// Execute tasks in sequence
    async fn execute_tasks_sequential(&self, tasks: &mut [SubagentTask]) -> Result<(), AppError>;

    /// Cancel a task
    async fn cancel_task(&self, task_id: &str) -> Result<(), AppError>;

    /// Get task status
    async fn get_task_status(&self, task_id: &str) -> Result<SubagentTask, AppError>;
}

/// Port for subagent task queue management
#[async_trait]
pub trait TaskQueue: Send + Sync {
    /// Add task to queue
    async fn enqueue(&self, task: SubagentTask) -> Result<(), AppError>;

    /// Dequeue next task
    async fn dequeue(&self) -> Result<SubagentTask, AppError>;

    /// Get queue size
    async fn queue_size(&self) -> Result<usize, AppError>;

    /// Clear queue
    async fn clear_queue(&self) -> Result<(), AppError>;
}
