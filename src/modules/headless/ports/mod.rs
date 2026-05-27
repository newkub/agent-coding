use async_trait::async_trait;

use crate::modules::headless::domain::models::command::{HeadlessCommand, HeadlessConfig};
use crate::shared::kernel::result::AppError;

/// Port for command execution in headless mode
#[async_trait]
pub trait HeadlessCommandExecutor: Send + Sync {
    /// Execute a headless command
    async fn execute(&self, command: &mut HeadlessCommand, config: &HeadlessConfig) -> Result<(), AppError>;
    
    /// Execute multiple commands in sequence
    async fn execute_batch(&self, commands: &mut [HeadlessCommand], config: &HeadlessConfig) -> Result<(), AppError>;
    
    /// Get command status
    async fn get_status(&self, command_id: &str) -> Result<HeadlessCommand, AppError>;
}

/// Port for output formatting and streaming
#[async_trait]
pub trait OutputFormatter: Send + Sync {
    /// Format command output
    fn format(&self, output: &str, config: &HeadlessConfig) -> String;
    
    /// Stream output (if streaming is enabled)
    async fn stream(&self, output: &str, config: &HeadlessConfig) -> Result<(), AppError>;
}

/// Port for session management in headless mode
#[async_trait]
pub trait HeadlessSessionManager: Send + Sync {
    /// Create new session
    async fn create_session(&self) -> Result<String, AppError>;
    
    /// Load existing session
    async fn load_session(&self, session_id: &str) -> Result<(), AppError>;
    
    /// Save session
    async fn save_session(&self, session_id: &str) -> Result<(), AppError>;
    
    /// List sessions
    async fn list_sessions(&self) -> Result<Vec<String>, AppError>;
    
    /// Delete session
    async fn delete_session(&self, session_id: &str) -> Result<(), AppError>;
}
