use crate::modules::subagents::domain::models::subagent::{Subagent, SubagentTask, TaskType};
use crate::modules::subagents::domain::operations::subagent_operations::select_subagent_for_task;
use crate::modules::subagents::domain::validators::subagent_validators;
use crate::modules::subagents::ports::{SubagentManager, SubagentTaskExecutor};
use crate::shared::kernel::result::AppError;

/// Use case for executing subagent tasks
pub struct ExecuteSubagentTaskUseCase<M, E>
where
    M: SubagentManager,
    E: SubagentTaskExecutor,
{
    manager: M,
    executor: E,
}

impl<M, E> ExecuteSubagentTaskUseCase<M, E>
where
    M: SubagentManager,
    E: SubagentTaskExecutor,
{
    pub(crate) const fn new(manager: M, executor: E) -> Self {
        Self { manager, executor }
    }

    /// Execute a single subagent task
    pub(crate) async fn execute(&self, task: &mut SubagentTask) -> Result<(), AppError> {
        // Validate task
        subagent_validators::validate_subagent_task(task)?;

        // Get available subagents
        let subagents = self.manager.get_available_subagents().await?;

        // Select appropriate subagent
        let selected_agent = select_subagent_for_task(&subagents, &task.task_type)
            .ok_or_else(|| AppError::NotFound("No available subagent for this task type".to_string()))?;

        // Update subagent status
        let mut agent = selected_agent.clone();
        agent.status = crate::modules::subagents::domain::models::subagent::SubagentStatus::Busy;
        self.manager.update_subagent(agent).await?;

        // Execute task
        self.executor.execute_task(task).await?;

        // Update subagent status back to idle
        let mut agent = selected_agent.clone();
        agent.status = crate::modules::subagents::domain::models::subagent::SubagentStatus::Idle;
        self.manager.update_subagent(agent).await?;

        Ok(())
    }

    /// Create and execute a task
    pub(crate) async fn create_and_execute(
        &self,
        task_type: TaskType,
        input: String,
        context: crate::modules::subagents::domain::models::subagent::TaskContext,
    ) -> Result<SubagentTask, AppError> {
        // Get available subagents
        let subagents = self.manager.get_available_subagents().await?;

        // Select appropriate subagent
        let selected_agent = select_subagent_for_task(&subagents, &task_type)
            .ok_or_else(|| AppError::NotFound("No available subagent for this task type".to_string()))?;

        // Create task
        let mut task = SubagentTask::new(selected_agent.id.clone(), task_type, input, context);

        // Execute task
        self.execute(&mut task).await?;

        Ok(task)
    }

    /// List all subagents
    pub(crate) async fn list_subagents(&self) -> Result<Vec<Subagent>, AppError> {
        self.manager.list_subagents().await
    }

    /// Create a new subagent
    pub(crate) async fn create_subagent(&self, subagent: Subagent) -> Result<Subagent, AppError> {
        subagent_validators::validate_subagent(&subagent)?;
        self.manager.create_subagent(subagent).await
    }

    /// Delete a subagent
    pub(crate) async fn delete_subagent(&self, id: &str) -> Result<(), AppError> {
        self.manager.delete_subagent(id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock implementations would go here
    // For brevity, we'll skip full mock implementations
}
