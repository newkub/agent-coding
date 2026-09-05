// Dependency Injection Container for Clean Architecture
#![allow(dead_code)]
// Centralized DI for testability and flexibility

use crate::adapters::db::audit_repository::SqliteAuditRepository;
use crate::adapters::db::automation_workflow_repository::SqliteAutomationWorkflowRepository;
use crate::adapters::db::collaboration_repository::SqliteCollaborationRepository;
use crate::adapters::db::guardrail_repository::SqliteGuardrailManager;
use crate::adapters::db::headless_session_repository::SqliteHeadlessSessionManager;
use crate::adapters::db::macro_repository::SqliteMacroRepository;
use crate::adapters::db::migrations;
use crate::adapters::db::performance_repository::{
    SqliteOptimizationManager, SqliteSnapshotManager,
};
use crate::adapters::db::session_repository::SqliteSessionRepository;
use crate::adapters::db::share_link_repository::SqliteShareLinkRepository;
use crate::adapters::db::share_repository::SqliteShareRepository;
use crate::adapters::db::subagent_repository::SqliteSubagentManager;
use crate::adapters::db::ui_content_repository::SqliteUiContentRepository;
use crate::adapters::external::{
    dependency_parser::DefaultDependencyParser, file_scanner::DefaultFileScanner,
    git_operations::Git2Adapter, github_client::ReqwestGitHubClient,
    headless_command_executor::DefaultHeadlessCommandExecutor,
    macro_executor::InMemoryMacroExecutor, metrics_collector::SystemMetricsCollector,
};
use crate::adapters::input::crossterm_handler::CrosstermInputHandler;
use crate::adapters::ui::ratatui_adapter::RatatuiAdapter;
use crate::modules::audit::ports::AuditRepository;
use crate::modules::automation::application::usecases::execute_automation::ExecuteAutomationUseCase;
use crate::modules::collaboration::ports::CollaborationRepository;
use crate::modules::headless::application::usecases::execute_headless::ExecuteHeadlessUseCase;
use crate::modules::macros::ports::{MacroExecutor, MacroRepository};
use crate::modules::onboarding::application::usecases::analyze_codebase::AnalyzeCodebaseUseCase;
use crate::modules::performance::application::usecases::analyze_performance::AnalyzePerformanceUseCase;
use crate::modules::session::ports::SessionRepository;
use crate::modules::share::ports::ShareRepository as ShareRepoPort;
use crate::shared::kernel::result::{AppError, AppResult};
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::SqlitePool;
use std::path::PathBuf;

// Type aliases for use cases with concrete types
type AnalyzeCodebaseUseCaseConcrete =
    AnalyzeCodebaseUseCase<DefaultFileScanner, DefaultDependencyParser>;
type ExecuteAutomationUseCaseConcrete =
    ExecuteAutomationUseCase<Git2Adapter, ReqwestGitHubClient, SqliteAutomationWorkflowRepository>;
type ExecuteHeadlessUseCaseConcrete =
    ExecuteHeadlessUseCase<DefaultHeadlessCommandExecutor, SqliteHeadlessSessionManager>;
type AnalyzePerformanceUseCaseConcrete = AnalyzePerformanceUseCase<
    SystemMetricsCollector,
    SqliteSnapshotManager,
    SqliteOptimizationManager,
>;

/// DI Container for managing dependencies
pub(crate) struct DIContainer {
    // Database pool shared by every SQLite-backed adapter
    db_pool: Option<SqlitePool>,

    // Repositories
    session_repo: Option<Box<dyn SessionRepository>>,
    audit_repo: Option<Box<dyn AuditRepository>>,
    collaboration_repo: Option<Box<dyn CollaborationRepository>>,
    macro_repo: Option<Box<dyn MacroRepository>>,
    share_link_repo: Option<SqliteShareLinkRepository>,
    share_repo: Option<Box<dyn ShareRepoPort>>,

    // External services
    git_adapter: Option<Git2Adapter>,
    github_client: Option<ReqwestGitHubClient>,
    file_scanner: Option<DefaultFileScanner>,
    dependency_parser: Option<DefaultDependencyParser>,
    metrics_collector: Option<SystemMetricsCollector>,
    subagent_manager: Option<SqliteSubagentManager>,
    guardrail_manager: Option<SqliteGuardrailManager>,
    ui_content_repo: Option<SqliteUiContentRepository>,
    macro_executor: Option<Box<dyn MacroExecutor>>,

    // UI adapters
    input_handler: Option<CrosstermInputHandler>,
    renderer: Option<RatatuiAdapter>,

    // Use cases
    analyze_codebase: Option<AnalyzeCodebaseUseCaseConcrete>,
    execute_automation: Option<ExecuteAutomationUseCaseConcrete>,
    execute_headless: Option<ExecuteHeadlessUseCaseConcrete>,
    analyze_performance: Option<AnalyzePerformanceUseCaseConcrete>,
}

impl DIContainer {
    /// Create new DI container with default implementations
    pub(crate) fn new() -> Self {
        Self {
            db_pool: None,
            session_repo: None,
            audit_repo: None,
            collaboration_repo: None,
            macro_repo: None,
            share_link_repo: None,
            share_repo: None,
            git_adapter: None,
            github_client: None,
            file_scanner: None,
            dependency_parser: None,
            metrics_collector: None,
            subagent_manager: None,
            guardrail_manager: None,
            ui_content_repo: None,
            macro_executor: None,
            input_handler: None,
            renderer: None,
            analyze_codebase: None,
            execute_automation: None,
            execute_headless: None,
            analyze_performance: None,
        }
    }

    /// Build and wire all dependencies (optimized for fast startup)
    pub(crate) async fn build(mut self) -> AppResult<Self> {
        // Warn (but do not fail) when optional integration credentials are
        // missing so the app still boots for offline/local use.
        if std::env::var("OPENAI_API_KEY").is_err() {
            tracing::warn!(
                "OPENAI_API_KEY is not set; subagent task execution will fail until configured"
            );
        }
        let github_token = std::env::var("GITHUB_TOKEN")
            .or_else(|_| std::env::var("GH_TOKEN"))
            .unwrap_or_default();
        if github_token.is_empty() {
            tracing::warn!(
                "GITHUB_TOKEN/GH_TOKEN is not set; GitHub automation features are unavailable"
            );
        }

        // Create external adapters (fast operations)
        let git_adapter = Git2Adapter::new(".".to_string());
        let github_client = ReqwestGitHubClient::new(github_token);

        // Create UI adapters (fast operations)
        let input_handler = CrosstermInputHandler::new();
        let renderer = RatatuiAdapter::new();

        // Create use cases (fast operations)
        let file_scanner = DefaultFileScanner::new();
        let dependency_parser = DefaultDependencyParser::new();
        let analyze_codebase =
            AnalyzeCodebaseUseCase::new(DefaultFileScanner::new(), DefaultDependencyParser::new());

        // System metrics collector (sysinfo-backed)
        let metrics_collector = SystemMetricsCollector::new();

        // The macro executor performs real local playback; it has no DB state.
        self.macro_executor = Some(Box::new(InMemoryMacroExecutor::new()));

        // Wire dependencies (skip DB connection - defer to `init_db()`)
        self.git_adapter = Some(git_adapter);
        self.github_client = Some(github_client);
        self.file_scanner = Some(file_scanner);
        self.dependency_parser = Some(dependency_parser);
        self.metrics_collector = Some(metrics_collector);
        self.input_handler = Some(input_handler);
        self.renderer = Some(renderer);
        self.analyze_codebase = Some(analyze_codebase);

        // Note: all SQLite-backed adapters are wired in `init_db()` so that
        // `build()` stays fast and tests can inject repositories first.

        Ok(self)
    }

    /// Resolve the SQLite database path. `AGENT_TUI_DB_PATH` overrides the
    /// repository-local default used by the CLI/TUI.
    pub(crate) fn database_path() -> PathBuf {
        std::env::var("AGENT_TUI_DB_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("agent_tui.db"))
    }

    /// Initialize the shared SQLite pool and wire every persistent adapter.
    ///
    /// Runs the embedded migrations once, then creates the repositories on the
    /// same pool. Slots already populated via the `with_*` test hooks are left
    /// untouched.
    pub(crate) async fn init_db(&mut self) -> AppResult<()> {
        if self.db_pool.is_none() {
            let options = SqliteConnectOptions::new()
                .filename(Self::database_path())
                .create_if_missing(true);
            let pool = SqlitePool::connect_with(options).await?;
            migrations::run_migrations(&pool).await?;
            tracing::info!(path = %Self::database_path().display(), "sqlite database initialized");
            self.db_pool = Some(pool);
        }

        let Some(pool) = self.db_pool.clone() else {
            return Ok(());
        };

        if self.session_repo.is_none() {
            self.session_repo = Some(Box::new(SqliteSessionRepository::new(pool.clone())));
        }
        if self.audit_repo.is_none() {
            self.audit_repo = Some(Box::new(SqliteAuditRepository::new(pool.clone())));
        }
        if self.collaboration_repo.is_none() {
            self.collaboration_repo =
                Some(Box::new(SqliteCollaborationRepository::new(pool.clone())));
        }
        if self.macro_repo.is_none() {
            self.macro_repo = Some(Box::new(SqliteMacroRepository::new(pool.clone())));
        }
        if self.share_link_repo.is_none() {
            let share_link_repo = SqliteShareLinkRepository::new(pool.clone());
            share_link_repo.init_table().await?;
            self.share_link_repo = Some(share_link_repo);
        }
        if self.share_repo.is_none() {
            let share_repo = SqliteShareRepository::new(pool.clone());
            share_repo.init_table().await?;
            self.share_repo = Some(Box::new(share_repo));
        }
        if self.subagent_manager.is_none() {
            let manager = SqliteSubagentManager::new(pool.clone());
            manager.initialize_default_subagents().await?;
            self.subagent_manager = Some(manager);
        }
        if self.guardrail_manager.is_none() {
            let manager = SqliteGuardrailManager::new(pool.clone());
            manager.initialize_default_guardrails().await?;
            self.guardrail_manager = Some(manager);
        }
        if self.ui_content_repo.is_none() {
            self.ui_content_repo = Some(SqliteUiContentRepository::new(pool.clone()));
        }
        if self.execute_automation.is_none() {
            let Some(git_adapter) = self.git_adapter.clone() else {
                return Err(AppError::State("Git adapter not available".to_string()));
            };
            let Some(github_client) = self.github_client.clone() else {
                return Err(AppError::State("GitHub client not available".to_string()));
            };
            self.execute_automation = Some(ExecuteAutomationUseCase::new(
                git_adapter,
                github_client,
                SqliteAutomationWorkflowRepository::new(pool.clone()),
            ));
        }
        if self.execute_headless.is_none() {
            self.execute_headless = Some(ExecuteHeadlessUseCase::new(
                DefaultHeadlessCommandExecutor::new(),
                SqliteHeadlessSessionManager::new(pool.clone()),
            ));
        }
        if self.analyze_performance.is_none() {
            self.analyze_performance = Some(AnalyzePerformanceUseCase::new(
                SystemMetricsCollector::new(),
                SqliteSnapshotManager::new(pool.clone()),
                SqliteOptimizationManager::new(pool),
            ));
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

    /// Get macro repository
    pub(crate) fn macro_repo(&self) -> Option<&dyn MacroRepository> {
        self.macro_repo.as_deref()
    }

    /// Get macro executor
    pub(crate) fn macro_executor(&self) -> Option<&dyn MacroExecutor> {
        self.macro_executor.as_deref()
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

    /// Get file scanner
    pub(crate) const fn file_scanner(&self) -> Option<&DefaultFileScanner> {
        self.file_scanner.as_ref()
    }

    /// Get dependency parser
    pub(crate) const fn dependency_parser(&self) -> Option<&DefaultDependencyParser> {
        self.dependency_parser.as_ref()
    }

    /// Get system metrics collector
    pub(crate) const fn metrics_collector(&self) -> Option<&SystemMetricsCollector> {
        self.metrics_collector.as_ref()
    }

    /// Get the persistent subagent manager.
    pub(crate) const fn subagent_manager(&self) -> Option<&SqliteSubagentManager> {
        self.subagent_manager.as_ref()
    }

    /// Get the persistent guardrail manager.
    pub(crate) const fn guardrail_manager(&self) -> Option<&SqliteGuardrailManager> {
        self.guardrail_manager.as_ref()
    }

    /// Get persistent UI content storage.
    pub(crate) const fn ui_content_repo(&self) -> Option<&SqliteUiContentRepository> {
        self.ui_content_repo.as_ref()
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

    /// Get analyze performance use case
    pub(crate) const fn analyze_performance_use_case(
        &self,
    ) -> Option<&AnalyzePerformanceUseCaseConcrete> {
        self.analyze_performance.as_ref()
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

    /// Set custom macro repository (for testing)
    pub(crate) fn with_macro_repo(mut self, repo: Box<dyn MacroRepository>) -> Self {
        self.macro_repo = Some(repo);
        self
    }

    /// Set custom macro executor (for testing)
    pub(crate) fn with_macro_executor(mut self, executor: Box<dyn MacroExecutor>) -> Self {
        self.macro_executor = Some(executor);
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
    use crate::adapters::external::headless_session_manager::InMemorySessionManager;

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
        let workflow_repository =
            SqliteAutomationWorkflowRepository::new(crate::adapters::db::test_pool().await);
        let _execute_automation = ExecuteAutomationUseCase::new(
            git_adapter.clone(),
            github_client.clone(),
            workflow_repository,
        );

        let headless_executor = DefaultHeadlessCommandExecutor::new();
        let headless_session_manager = InMemorySessionManager::new();
        let _execute_headless =
            ExecuteHeadlessUseCase::new(headless_executor, headless_session_manager);

        // If we reach here, all use cases were created successfully
    }
}
