use crate::modules::headless::domain::models::command::{
    CommandType, HeadlessCommand, HeadlessConfig,
};
use crate::modules::headless::domain::operations::command_operations::parse_command;
use crate::modules::headless::domain::validators::command_validators;
use crate::modules::headless::ports::{HeadlessCommandExecutor, HeadlessSessionManager};
use crate::shared::kernel::result::AppError;

/// Use case for executing headless commands
pub(crate) struct ExecuteHeadlessUseCase<E, S>
where
    E: HeadlessCommandExecutor,
    S: HeadlessSessionManager,
{
    executor: E,
    session_manager: S,
}

impl<E, S> ExecuteHeadlessUseCase<E, S>
where
    E: HeadlessCommandExecutor,
    S: HeadlessSessionManager,
{
    pub(crate) const fn new(executor: E, session_manager: S) -> Self {
        Self {
            executor,
            session_manager,
        }
    }

    /// Execute a single command
    pub(crate) async fn execute(
        &self,
        input: String,
        working_directory: String,
        config: &HeadlessConfig,
    ) -> Result<HeadlessCommand, AppError> {
        // Validate input
        command_validators::validate_command_input(&input)?;
        command_validators::validate_command_context(&working_directory)?;

        // Parse command type
        let command_type = parse_command(&input).map_err(AppError::ValidationError)?;

        // Handle session-related commands
        let session_id = match command_type {
            CommandType::SessionCreate => {
                let session_id = self.session_manager.create_session().await?;
                Some(session_id)
            }
            CommandType::SessionLoad => {
                let args: Vec<&str> = input.split_whitespace().collect();
                if args.len() < 2 {
                    return Err(AppError::ValidationError(
                        "Session load requires session ID".to_string(),
                    ));
                }
                let session_id = args[1].to_string();
                self.session_manager.load_session(&session_id).await?;
                Some(session_id)
            }
            _ => None,
        };

        // Create context
        let mut context = crate::modules::headless::domain::models::command::CommandContext::new(
            working_directory,
        );
        if let Some(sid) = session_id {
            context = context.with_session(sid);
        }

        // Create command
        let mut command = HeadlessCommand::new(command_type, input, context);

        // Execute command
        self.executor.execute(&mut command, config).await?;

        Ok(command)
    }

    /// Execute multiple commands in batch
    pub(crate) async fn execute_batch(
        &self,
        inputs: Vec<String>,
        working_directory: String,
        config: &HeadlessConfig,
    ) -> Result<Vec<HeadlessCommand>, AppError> {
        let mut commands = Vec::new();

        for input in inputs {
            let command = self
                .execute(input, working_directory.clone(), config)
                .await?;
            commands.push(command);
        }

        Ok(commands)
    }

    /// List available sessions
    pub(crate) async fn list_sessions(&self) -> Result<Vec<String>, AppError> {
        self.session_manager.list_sessions().await
    }

    /// Create a new session
    pub(crate) async fn create_session(&self) -> Result<String, AppError> {
        self.session_manager.create_session().await
    }

    /// Load an existing session
    pub(crate) async fn load_session(&self, session_id: &str) -> Result<(), AppError> {
        self.session_manager.load_session(session_id).await
    }

    /// Save current session
    pub(crate) async fn save_session(&self, session_id: &str) -> Result<(), AppError> {
        self.session_manager.save_session(session_id).await
    }

    /// Delete a session
    pub(crate) async fn delete_session(&self, session_id: &str) -> Result<(), AppError> {
        self.session_manager.delete_session(session_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock implementations would go here
    // For brevity, we'll skip full mock implementations
}
