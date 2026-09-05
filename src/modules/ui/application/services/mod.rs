use super::super::domain::models::{AppState, PackageItem, SkillItem, ToastKind, WorkflowItem};
use crate::adapters::config::loader::load_automation_config;
use crate::modules::audit::application::usecases::{log_entry, AuditQuery};
use crate::modules::audit::domain::models::{Actor, ActorType, AuditAction, Resource};
use crate::modules::onboarding::ports::DependencyParser;
use crate::modules::performance::ports::MetricsCollector;
use crate::modules::subagents::ports::SubagentManager;
use crate::modules::ui::ports::UiContentRepository;
use crate::presentation::tui::di::DIContainer;
use crate::shared::kernel::result::AppResult;
use sqlx::Row;
use std::path::Path;
use task_tui::task_manager::adapters::storage::JsonFileStorage;
use task_tui::task_manager::ports::WorkspaceRepository;

/// Service: Initialize app state
pub(crate) fn initialize_app_state() -> AppState {
    let mut state = AppState::new();
    load_tasks(&mut state);
    state
}

/// Load persisted task workspaces from `task-tui` JSON storage into the task manager.
pub(crate) fn load_tasks(state: &mut AppState) {
    let Ok(storage) = JsonFileStorage::default_location() else {
        return;
    };
    if let Ok(workspaces) = storage.load() {
        if !workspaces.is_empty() {
            state.tasks_tab_state.task_manager.tree.workspaces = workspaces;
        }
    }
}

/// Persist the task manager tree to `task-tui` JSON storage.
pub(crate) fn persist_tasks(state: &mut AppState) {
    let Ok(storage) = JsonFileStorage::default_location() else {
        state.push_toast(
            ToastKind::Error,
            "Task storage is not available".to_string(),
        );
        return;
    };
    if let Err(e) = storage.save(&state.tasks_tab_state.task_manager.tree.workspaces) {
        state.push_toast(ToastKind::Error, format!("Failed to save tasks: {e}"));
    }
}

/// Service: Get current tab content
pub(crate) fn get_current_tab_content(state: &AppState) -> String {
    let content = state.current_tab_content();
    format!(
        "Left: {}\nCenter: {}\nRight: {}",
        content.left, content.center, content.right
    )
}

impl AppState {
    /// Preload real data from the DI container into the per-tab state.
    ///
    /// Every section is loaded best-effort: a failing source leaves the
    /// corresponding tab in its default (empty) state instead of aborting
    /// startup, so the TUI always opens.
    pub(crate) async fn load_from_di(&mut self, di: &DIContainer) -> AppResult<()> {
        refresh_agent_sessions(self, di).await;
        refresh_files(self, di).await;
        refresh_git_status(self, di);
        refresh_packages(self, di).await;
        record_audit_event(
            di,
            AuditAction::ConfigChange {
                key: "tui.start".to_string(),
            },
        )
        .await;
        refresh_audit_logs(self, di).await;
        refresh_system_metrics(self, di).await;
        refresh_performance(self, di).await;
        refresh_database_tables(self, di).await;
        refresh_skills(self, di).await;
        refresh_workflows(self, di);
        refresh_collaboration_sessions(self, di).await;
        refresh_macros(self, di).await;
        refresh_headless_sessions(self, di).await;
        refresh_notes(self, di).await;
        refresh_snippets(self, di).await;
        Ok(())
    }
}

/// Record an audit event through the audit repository (best-effort).
pub(crate) async fn record_audit_event(di: &DIContainer, action: AuditAction) {
    let Some(repo) = di.audit_repo() else {
        return;
    };
    if let Err(e) = log_entry(
        repo,
        action,
        Actor {
            type_: ActorType::User,
            id: "tui".to_string(),
            name: "agent-tui".to_string(),
        },
        Resource {
            type_: "application".to_string(),
            id: "agent-tui".to_string(),
            path: None,
        },
    )
    .await
    {
        tracing::warn!(error = %e, "failed to record audit event");
    }
}

/// Agent tab: sessions from the session repository.
pub(crate) async fn refresh_agent_sessions(state: &mut AppState, di: &DIContainer) {
    if let Some(repo) = di.session_repo() {
        if let Ok(mut sessions) = repo.find_all().await {
            sessions.sort_by_key(|s| std::cmp::Reverse(s.updated_at));
            state.agent_tab_state.sessions = sessions;
        }
    }
}

/// Files tab: list `current_path` via the file scanner (shallow listing,
/// with a `../` entry to navigate to the parent directory).
pub(crate) async fn refresh_files(state: &mut AppState, di: &DIContainer) {
    if state.files_tab_state.current_path.is_empty() {
        let cwd = std::env::current_dir().unwrap_or_else(|_| Path::new(".").to_path_buf());
        state.files_tab_state.current_path = cwd.display().to_string();
    }

    let base = Path::new(&state.files_tab_state.current_path);
    if let Some(scanner) = di.file_scanner() {
        if let Ok(mut entries) = scanner.list_entries(base).await {
            if base.parent().is_some() {
                entries.insert(0, "../".to_string());
            }
            state.files_tab_state.files = entries;
        }
    }
}

/// Git tab: current branch and staged/unstaged files via the git adapter.
pub(crate) fn refresh_git_status(state: &mut AppState, di: &DIContainer) {
    let Some(git) = di.git_adapter() else {
        return;
    };

    state.git_tab_state.current_branch = git.current_branch_name().unwrap_or_default();
    if let Ok((staged, unstaged)) = git.status_entries() {
        state.git_tab_state.staged_files = staged;
        state.git_tab_state.unstaged_files = unstaged;
        let total = staged_count(state) + unstaged_count(state);
        if state.git_tab_state.selected_file_index >= total {
            state.git_tab_state.selected_file_index = total.saturating_sub(1);
        }
    }
}

fn staged_count(state: &AppState) -> usize {
    state.git_tab_state.staged_files.len()
}

fn unstaged_count(state: &AppState) -> usize {
    state.git_tab_state.unstaged_files.len()
}

/// Packages tab: dependencies parsed from the project manifest.
pub(crate) async fn refresh_packages(state: &mut AppState, di: &DIContainer) {
    let Some(parser) = di.dependency_parser() else {
        return;
    };
    let Ok(deps) = parser.parse_dependencies(Path::new(".")).await else {
        return;
    };

    state.packages_tab_state.package_manager = deps.package_manager;
    state.packages_tab_state.packages = deps
        .dependencies
        .iter()
        .map(|(name, info)| PackageItem {
            name: name.clone(),
            version: info.version.clone(),
            category: info.category.clone(),
            outdated: false,
        })
        .chain(
            deps.dev_dependencies
                .iter()
                .map(|(name, info)| PackageItem {
                    name: name.clone(),
                    version: info.version.clone(),
                    category: "dev".to_string(),
                    outdated: false,
                }),
        )
        .collect();
    state
        .packages_tab_state
        .packages
        .sort_by(|a, b| a.name.cmp(&b.name));
}

/// Logs tab: reload audit entries from the audit repository.
pub(crate) async fn refresh_audit_logs(state: &mut AppState, di: &DIContainer) {
    let Some(repo) = di.audit_repo() else {
        return;
    };

    let query = AuditQuery {
        category: state.logs_tab_state.log_level_filter.clone(),
        limit: Some(100),
        ..AuditQuery::default()
    };
    if let Ok(entries) = repo.query(query).await {
        state.logs_tab_state.entries = entries;
    }
}

/// System tab: real host metrics via the sysinfo-backed collector.
pub(crate) async fn refresh_system_metrics(state: &mut AppState, di: &DIContainer) {
    let Some(collector) = di.metrics_collector() else {
        return;
    };
    let Ok(metrics) = collector.collect_metrics().await else {
        return;
    };

    let mem_used_mb = metrics.memory_usage / 1_048_576;
    let mem_total_mb = metrics.memory_total / 1_048_576;
    let mem_pct = metrics.memory_usage_percentage();

    state.system_tab_state.metrics = vec![
        (
            "CPU Usage".to_string(),
            format!("{:.1}%", metrics.cpu_usage),
        ),
        (
            "Memory Used".to_string(),
            format!("{mem_used_mb} / {mem_total_mb} MB ({mem_pct:.1}%)"),
        ),
        (
            "Health".to_string(),
            if metrics.is_healthy() {
                "healthy".to_string()
            } else {
                "degraded".to_string()
            },
        ),
    ];

    let mut alerts = Vec::new();
    if metrics.cpu_usage > 80.0 {
        alerts.push(format!("High CPU usage: {:.1}%", metrics.cpu_usage));
    }
    if mem_pct > 85.0 {
        alerts.push(format!("High memory usage: {mem_pct:.1}%"));
    }
    state.system_tab_state.alerts = alerts;
}

/// Database tab: list tables in the backing SQLite database.
pub(crate) async fn refresh_database_tables(state: &mut AppState, di: &DIContainer) {
    let Some(repo) = di.share_link_repo() else {
        return;
    };

    if let Ok(rows) = sqlx::query(
        "SELECT name FROM sqlite_master WHERE type = 'table' AND name NOT LIKE 'sqlite_%' ORDER BY name",
    )
    .fetch_all(repo.pool())
    .await
    {
        state.database_tab_state.tables =
            rows.iter().map(|r| r.get::<String, _>(0)).collect();
    }
}

/// Skills tab: subagents registered in the subagent manager.
pub(crate) async fn refresh_skills(state: &mut AppState, di: &DIContainer) {
    let Some(manager) = di.subagent_manager() else {
        return;
    };

    if let Ok(subagents) = manager.list_subagents().await {
        let mut skills: Vec<SkillItem> = subagents
            .into_iter()
            .map(|s| SkillItem {
                name: s.name,
                description: s.description,
                status: format!("{:?}", s.status),
            })
            .collect();
        skills.sort_by(|a, b| a.name.cmp(&b.name));
        state.skills_tab_state.skills = skills;
    }
}

/// Workflows tab: the automation pipeline plan derived from the real
/// automation configuration and the current git branch.
pub(crate) fn refresh_workflows(state: &mut AppState, di: &DIContainer) {
    let config = load_automation_config();
    let branch = di
        .git_adapter()
        .and_then(|g| g.current_branch_name())
        .unwrap_or_else(|| "unknown".to_string());

    let step = |name: &str, enabled: bool| {
        if enabled {
            name.to_string()
        } else {
            format!("{name} (skipped by config)")
        }
    };

    if state.workflows_tab_state.workflows.is_empty() {
        state.workflows_tab_state.workflows = vec![WorkflowItem {
            name: format!("issue-to-pr @ {branch}"),
            status: "ready".to_string(),
            steps: vec![
                step("Create branch", config.auto_create_branch),
                step("Commit changes", config.auto_commit),
                step("Push to remote", config.auto_push),
                step("Create pull request", config.auto_create_pr),
            ],
        }];
    }
}

/// System tab: performance analysis (score + suggestions) and snapshot list
/// via the `AnalyzePerformanceUseCase`.
pub(crate) async fn refresh_performance(state: &mut AppState, di: &DIContainer) {
    let Some(use_case) = di.analyze_performance_use_case() else {
        return;
    };

    if let Ok(result) = use_case.analyze_current().await {
        state.system_tab_state.performance_score = Some(result.score);
        if !result.is_healthy {
            state
                .system_tab_state
                .alerts
                .push("Performance degraded".to_string());
        }
        state.system_tab_state.suggestions = result
            .suggestions
            .iter()
            .map(|s| {
                format!(
                    "[{:?}] {} (+{:.0}%)",
                    s.impact,
                    s.title,
                    s.estimated_improvement * 100.0
                )
            })
            .collect();
    }

    if let Ok(snapshots) = use_case.list_snapshots().await {
        state.system_tab_state.snapshots = snapshots
            .iter()
            .map(|s| format!("{} ({})", s.name, s.id))
            .collect();
    }
}

/// Collaboration tab: active sessions from the collaboration repository.
pub(crate) async fn refresh_collaboration_sessions(state: &mut AppState, di: &DIContainer) {
    let Some(repo) = di.collaboration_repo() else {
        return;
    };
    if let Ok(sessions) = repo.find_active().await {
        state.collaboration_tab_state.sessions = sessions;
        let max = state
            .collaboration_tab_state
            .sessions
            .len()
            .saturating_sub(1);
        state.collaboration_tab_state.selected_session_index = state
            .collaboration_tab_state
            .selected_session_index
            .min(max);
    }
}

/// Macros tab: all macros from the macro repository.
pub(crate) async fn refresh_macros(state: &mut AppState, di: &DIContainer) {
    let Some(repo) = di.macro_repo() else {
        return;
    };
    if let Ok(macros) = crate::modules::macros::application::usecases::list_macros(repo).await {
        state.macros_tab_state.macros = macros;
        let max = state.macros_tab_state.macros.len().saturating_sub(1);
        state.macros_tab_state.selected_index = state.macros_tab_state.selected_index.min(max);
    }
}

/// Terminal tab: headless session ids from the headless session manager.
pub(crate) async fn refresh_headless_sessions(state: &mut AppState, di: &DIContainer) {
    let Some(use_case) = di.execute_headless_use_case() else {
        return;
    };
    if let Ok(sessions) = use_case.list_sessions().await {
        state.terminal_tab_state.headless_sessions = sessions;
        let max = state
            .terminal_tab_state
            .headless_sessions
            .len()
            .saturating_sub(1);
        state.terminal_tab_state.selected_session_index =
            state.terminal_tab_state.selected_session_index.min(max);
    }
}

/// Notes tab: persisted notes from the UI content repository.
pub(crate) async fn refresh_notes(state: &mut AppState, di: &DIContainer) {
    let Some(repo) = di.ui_content_repo() else {
        return;
    };
    if let Ok(notes) = repo.list_notes().await {
        state.notes_tab_state.notes = notes;
        let max = state.notes_tab_state.notes.len().saturating_sub(1);
        state.notes_tab_state.selected_note_index =
            state.notes_tab_state.selected_note_index.min(max);
    }
}

/// Snippets tabs: persisted snippets from the UI content repository.
pub(crate) async fn refresh_snippets(state: &mut AppState, di: &DIContainer) {
    let Some(repo) = di.ui_content_repo() else {
        return;
    };
    if let Ok(snippets) = repo.list_snippets().await {
        state.snippet_tab_state.snippets = snippets;
        let max = state.snippet_tab_state.snippets.len().saturating_sub(1);
        state.snippet_tab_state.selected_snippet_index =
            state.snippet_tab_state.selected_snippet_index.min(max);
    }
}
