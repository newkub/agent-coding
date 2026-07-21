use crate::modules::sandbox::domain::models::{Command, CommandResult, SandboxConfig};
use crate::shared::kernel::result::AppResult;
use async_trait::async_trait;

/// Port: Command Executor
#[async_trait]
pub(crate) trait CommandExecutor: Send + Sync {
    async fn execute(&self, command: &Command) -> AppResult<CommandResult>;
    async fn execute_sandboxed(&self, command: &Command) -> AppResult<CommandResult>;
}

/// Port: Approval Engine
#[async_trait]
pub(crate) trait ApprovalEngine: Send + Sync {
    async fn check_approval(&self, command: &Command) -> AppResult<bool>;
    async fn request_approval(&self, command: &Command) -> AppResult<ApprovalRequest>;
}

/// Approval request details
#[derive(Debug)]
pub(crate) struct ApprovalRequest {
    pub command: Command,
    pub risk_score: u32,
    pub requires_review: bool,
}

/// Port: Sandbox Manager
#[async_trait]
pub(crate) trait SandboxManager: Send + Sync {
    async fn create_sandbox(&self, config: &SandboxConfig) -> AppResult<SandboxId>;
    async fn execute_in_sandbox(
        &self,
        sandbox_id: &SandboxId,
        command: &str,
    ) -> AppResult<CommandResult>;
    async fn destroy_sandbox(&self, sandbox_id: &SandboxId) -> AppResult<()>;
    async fn get_sandbox_status(&self, sandbox_id: &SandboxId) -> AppResult<SandboxStatus>;
}

#[derive(Debug, Clone)]
pub(crate) struct SandboxId(pub String);

impl std::fmt::Display for SandboxId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub(crate) enum SandboxStatus {
    Running,
    Stopped,
    Error(String),
}
