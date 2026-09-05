use super::super::domain::models::{AppState, NoteItem, PackageItem, SkillItem, WorkflowItem};
use crate::modules::audit::application::usecases::{log_entry, AuditQuery};
use crate::modules::audit::domain::models::{Actor, ActorType, AuditAction, Resource};
use crate::modules::automation::domain::models::issue_pr::AutomationConfig;
use crate::modules::onboarding::ports::DependencyParser;
use crate::modules::performance::ports::MetricsCollector;
use crate::modules::subagents::ports::SubagentManager;
use crate::presentation::tui::di::DIContainer;
use crate::shared::kernel::result::AppResult;
use sqlx::Row;
use std::path::Path;

/// Service: Initialize app state
pub(crate) fn initialize_app_state() -> AppState {
    AppState::new()
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
        refresh_database_tables(self, di).await;
        refresh_skills(self, di).await;
        refresh_workflows(self, di);
        seed_notes(self);
        Ok(())
    }
}

/// Record an audit event through the audit repository (best-effort).
pub(crate) async fn record_audit_event(di: &DIContainer, action: AuditAction) {
    let Some(repo) = di.audit_repo() else {
        return;
    };
    let _ = log_entry(
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
    .await;
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
    let config = AutomationConfig::default();
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

/// Notes tab: no notes repository exists yet; seed a static welcome note.
fn seed_notes(state: &mut AppState) {
    state.notes_tab_state.notes = vec![NoteItem {
        title: "Welcome".to_string(),
        content: "Notes are kept in-memory for now. Press Enter to edit the selected note."
            .to_string(),
    }];
}
