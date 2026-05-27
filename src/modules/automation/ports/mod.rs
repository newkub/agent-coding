use async_trait::async_trait;

use crate::modules::automation::domain::models::issue_pr::{Issue, PullRequest, AutomationWorkflow, AutomationConfig};
use crate::shared::kernel::result::AppError;

/// Port for GitHub API operations
#[async_trait]
pub trait GitHubClient: Send + Sync {
    /// Get issue by number
    async fn get_issue(&self, repository: &str, number: u32) -> Result<Issue, AppError>;
    
    /// Create pull request
    async fn create_pull_request(
        &self,
        repository: &str,
        title: &str,
        body: &str,
        source_branch: &str,
        target_branch: &str,
    ) -> Result<PullRequest, AppError>;
    
    /// Update pull request
    async fn update_pull_request(&self, repository: &str, number: u32, pr: &PullRequest) -> Result<PullRequest, AppError>;
    
    /// Add labels to issue
    async fn add_labels(&self, repository: &str, issue_number: u32, labels: Vec<String>) -> Result<(), AppError>;
    
    /// Add reviewers to pull request
    async fn add_reviewers(&self, repository: &str, pr_number: u32, reviewers: Vec<String>) -> Result<(), AppError>;
    
    /// Get repository default branch
    async fn get_default_branch(&self, repository: &str) -> Result<String, AppError>;
}

/// Port for Git operations
#[async_trait]
pub trait GitOperations: Send + Sync {
    /// Create new branch
    async fn create_branch(&self, branch_name: &str) -> Result<(), AppError>;
    
    /// Checkout branch
    async fn checkout_branch(&self, branch_name: &str) -> Result<(), AppError>;
    
    /// Commit changes
    async fn commit(&self, message: &str) -> Result<(), AppError>;
    
    /// Push branch to remote
    async fn push(&self, branch_name: &str) -> Result<(), AppError>;
    
    /// Get current branch
    async fn get_current_branch(&self) -> Result<String, AppError>;
    
    /// Check if branch exists locally
    async fn branch_exists(&self, branch_name: &str) -> Result<bool, AppError>;
    
    /// Check if branch exists remotely
    async fn remote_branch_exists(&self, branch_name: &str) -> Result<bool, AppError>;
}

/// Port for automation workflow execution
#[async_trait]
pub trait AutomationWorkflowExecutor: Send + Sync {
    /// Execute automation workflow
    async fn execute(&self, workflow: &mut AutomationWorkflow, config: &AutomationConfig) -> Result<(), AppError>;
    
    /// Get workflow status
    async fn get_status(&self, workflow_id: &str) -> Result<AutomationWorkflow, AppError>;
    
    /// Cancel workflow
    async fn cancel(&self, workflow_id: &str) -> Result<(), AppError>;
}
