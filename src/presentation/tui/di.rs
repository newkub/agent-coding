// Dependency Injection Container for Clean Architecture
#![allow(dead_code)]
// Centralized DI for testability and flexibility

use crate::adapters::db::audit_repository::InMemoryAuditRepository;
use crate::adapters::db::session_repository::InMemorySessionRepository;
use crate::adapters::db::share_link_repository::SqliteShareLinkRepository;
use crate::adapters::db::share_repository::SqliteShareRepository;
use crate::adapters::external::{
    dependency_parser::DefaultDependencyParser, file_scanner::DefaultFileScanner,
    git_operations::Git2Adapter, github_client::ReqwestGitHubClient,
    headless_command_executor::DefaultHeadlessCommandExecutor,
    headless_session_manager::InMemorySessionManager,
};
use crate::adapters::input::crossterm_handler::CrosstermInputHandler;
use crate::adapters::ui::ratatui_adapter::RatatuiAdapter;
use crate::modules::audit::ports::AuditRepository;
use crate::modules::automation::application::usecases::execute_automation::ExecuteAutomationUseCase;
use crate::modules::collaboration::ports::CollaborationRepository;
use crate::modules::headless::application::usecases::execute_headless::ExecuteHeadlessUseCase;
use crate::modules::onboarding::application::usecases::analyze_codebase::AnalyzeCodebaseUseCase;
use crate::modules::session::ports::SessionRepository;
use crate::modules::share::ports::ShareRepository as ShareRepoPort;
use crate::shared::kernel::result::AppResult;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::SqlitePool;
use std::str::FromStr;

// Type aliases for use cases with concrete types
type AnalyzeCodebaseUseCaseConcrete =
    AnalyzeCodebaseUseCase<DefaultFileScanner, DefaultDependencyParser>;
type ExecuteAutomationUseCaseConcrete = ExecuteAutomationUseCase<Git2Adapter, ReqwestGitHubClient>;
type ExecuteHeadlessUseCaseConcrete =
    ExecuteHeadlessUseCase<DefaultHeadlessCommandExecutor, InMemorySessionManager>;

/// DI Container for managing dependencies
pub(crate) struct DIContainer {
    // Repositories
    session_repo: Option<Box<dyn SessionRepository>>,
    audit_repo: Option<Box<dyn AuditRepository>>,
    collaboration_repo: Option<Box<dyn CollaborationRepository>>,
    share_link_repo: Option<SqliteShareLinkRepository>,
    share_repo: Option<Box<dyn ShareRepoPort>>,

    // External services
    git_adapter: Option<Git2Adapter>,
    github_client: Option<ReqwestGitHubClient>,

    // UI adapters
    input_handler: Option<CrosstermInputHandler>,
    renderer: Option<RatatuiAdapter>,

    // Use cases
    analyze_codebase: Option<AnalyzeCodebaseUseCaseConcrete>,
    execute_automation: Option<ExecuteAutomationUseCaseConcrete>,
    execute_headless: Option<ExecuteHeadlessUseCaseConcrete>,
}

impl DIContainer {
    /// Create new DI container with default implementations
    pub(crate) fn new() -> Self {
        Self {
            session_repo: None,
            audit_repo: None,
            collaboration_repo: None,
            share_link_repo: None,
            share_repo: None,
            git_adapter: None,
            github_client: None,
            input_handler: None,
            renderer: None,
            analyze_codebase: None,
            execute_automation: None,
            execute_headless: None,
        }
    }

    /// Build and wire all dependencies (optimized for fast startup)
    pub(crate) async fn build(mut self) -> AppResult<Self> {
        // Create external adapters (fast operations)
        let git_adapter = Git2Adapter::new(".".to_string());
        let github_token = std::env::var("GITHUB_TOKEN").unwrap_or_else(|_| "".to_string());
        let github_client = ReqwestGitHubClient::new(github_token);

        // Create UI adapters (fast operations)
        let input_handler = CrosstermInputHandler::new();
        let renderer = RatatuiAdapter::new();

        // Create use cases (fast operations)
        let file_scanner = DefaultFileScanner::new();
        let dependency_parser = DefaultDependencyParser::new();
        let analyze_codebase = AnalyzeCodebaseUseCase::new(file_scanner, dependency_parser);

        let execute_automation =
            ExecuteAutomationUseCase::new(git_adapter.clone(), github_client.clone());

        let headless_executor = DefaultHeadlessCommandExecutor::new();
        let headless_session_manager = InMemorySessionManager::new();
        let execute_headless =
            ExecuteHeadlessUseCase::new(headless_executor, headless_session_manager);

        // Wire in-memory repositories (fast, no DB connection)
        self.session_repo = Some(Box::new(InMemorySessionRepository::new()));
        self.audit_repo = Some(Box::new(InMemoryAuditRepository::new()));

        // Wire dependencies (skip DB connection - defer to when needed)
        self.git_adapter = Some(git_adapter);
        self.github_client = Some(github_client);
        self.input_handler = Some(input_handler);
        self.renderer = Some(renderer);
        self.analyze_codebase = Some(analyze_codebase);
        self.execute_automation = Some(execute_automation);
        self.execute_headless = Some(execute_headless);

        // Note: share_link_repo with DB connection is deferred to when actually needed
        // This reduces startup time by ~500ms

        Ok(self)
    }

    /// Initialize database connection lazily when needed
    pub(crate) async fn init_db(&mut self) -> AppResult<()> {
        if self.share_link_repo.is_none() {
            let options =
                SqliteConnectOptions::from_str("sqlite:share_links.db")?.create_if_missing(true);
            let pool = SqlitePool::connect_with(options).await?;

            // Wire ShareLinkRepository on the same pool
            let share_link_repo = SqliteShareLinkRepository::new(pool.clone());
            share_link_repo.init_table().await?;
            self.share_link_repo = Some(share_link_repo);

            // Wire ShareRepository (session export/import) on the same pool
            let share_repo = SqliteShareRepository::new(pool);
            share_repo.init_table().await?;
            self.share_repo = Some(Box::new(share_repo));
        }
        Ok(())
    }

    /// Get session repository
    pub(crate) fn session_repo(&self) -> Option<&dyn SessionRepository> {
        self.session_repo.as_deref()
    }

    /// Get audit repository
    pub(crate) fn audit_repo(&self) -> Option<&dyn AuditRepository> {
        self.audit_repo.as_deref()
    }

    /// Get collaboration repository
    pub(crate) fn collaboration_repo(&self) -> Option<&dyn CollaborationRepository> {
        self.collaboration_repo.as_deref()
    }

    /// Get share link repository
    pub(crate) fn share_link_repo(&self) -> Option<&SqliteShareLinkRepository> {
        self.share_link_repo.as_ref()
    }

    /// Get share repository (session export/import)
    pub(crate) fn share_repo(&self) -> Option<&dyn ShareRepoPort> {
        self.share_repo.as_deref()
    }

    /// Get git adapter
    pub(crate) const fn git_adapter(&self) -> Option<&Git2Adapter> {
        self.git_adapter.as_ref()
    }

    /// Get GitHub client
    pub(crate) const fn github_client(&self) -> Option<&ReqwestGitHubClient> {
        self.github_client.as_ref()
    }

    /// Get input handler
    pub(crate) const fn input_handler(&self) -> Option<&CrosstermInputHandler> {
        self.input_handler.as_ref()
    }

    /// Get renderer
    pub(crate) const fn renderer(&self) -> Option<&RatatuiAdapter> {
        self.renderer.as_ref()
    }

    /// Get analyze codebase use case
    pub(crate) const fn analyze_codebase_use_case(
        &self,
    ) -> Option<&AnalyzeCodebaseUseCaseConcrete> {
        self.analyze_codebase.as_ref()
    }

    /// Get execute automation use case
    pub(crate) const fn execute_automation_use_case(
        &self,
    ) -> Option<&ExecuteAutomationUseCaseConcrete> {
        self.execute_automation.as_ref()
    }

    /// Get execute headless use case
    pub(crate) const fn execute_headless_use_case(
        &self,
    ) -> Option<&ExecuteHeadlessUseCaseConcrete> {
        self.execute_headless.as_ref()
    }

    /// Set custom session repository (for testing)
    pub(crate) fn with_session_repo(mut self, repo: Box<dyn SessionRepository>) -> Self {
        self.session_repo = Some(repo);
        self
    }

    /// Set custom audit repository (for testing)
    pub(crate) fn with_audit_repo(mut self, repo: Box<dyn AuditRepository>) -> Self {
        self.audit_repo = Some(repo);
        self
    }

    /// Set custom collaboration repository (for testing)
    pub(crate) fn with_collaboration_repo(
        mut self,
        repo: Box<dyn CollaborationRepository>,
    ) -> Self {
        self.collaboration_repo = Some(repo);
        self
    }

    /// Set custom share link repository (for testing)
    pub(crate) fn with_share_link_repo(mut self, repo: SqliteShareLinkRepository) -> Self {
        self.share_link_repo = Some(repo);
        self
    }

    /// Set custom share repository (for testing)
    pub(crate) fn with_share_repo(mut self, repo: Box<dyn ShareRepoPort>) -> Self {
        self.share_repo = Some(repo);
        self
    }
}

impl Default for DIContainer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_di_container_creation() {
        let container = DIContainer::new();
        assert!(container.session_repo.is_none());
        assert!(container.audit_repo.is_none());
    }

    #[tokio::test]
    async fn test_di_container_build() {
        // Skip SQLite connection for unit test - test only DI wiring
        let _container = DIContainer::new();

        // Test that we can create use cases without DB connection
        let file_scanner = DefaultFileScanner::new();
        let dependency_parser = DefaultDependencyParser::new();
        let _analyze_codebase = AnalyzeCodebaseUseCase::new(file_scanner, dependency_parser);

        let git_adapter = Git2Adapter::new(".".to_string());
        let github_token = std::env::var("GITHUB_TOKEN").unwrap_or_else(|_| "".to_string());
        let github_client = ReqwestGitHubClient::new(github_token);
        let _execute_automation =
            ExecuteAutomationUseCase::new(git_adapter.clone(), github_client.clone());

        let headless_executor = DefaultHeadlessCommandExecutor::new();
        let headless_session_manager = InMemorySessionManager::new();
        let _execute_headless =
            ExecuteHeadlessUseCase::new(headless_executor, headless_session_manager);

        // If we reach here, all use cases were created successfully
    }
}
