//! Async side effects for tab actions, backed by the DI container.
//!
//! The synchronous handlers in `handlers/` update `AppState`; this module
//! performs the real I/O (repositories, git, filesystem, shell, sqlite,
//! metrics) and refreshes the affected state afterwards.

use super::handlers::TabAction;
use super::services;
use crate::modules::audit::domain::models::AuditAction;
use crate::modules::session::domain::models::{Message, MessageRole, Session, SessionId};
use crate::modules::ui::domain::models::AppState;
use crate::presentation::tui::di::DIContainer;
use crate::shared::kernel::result::AppResult;
use crate::shared::kernel::types::Tab;
use sqlx::Row;
use std::path::PathBuf;

/// Run the side effects of a tab action against the DI container.
pub(crate) async fn apply_tab_effects(
    state: &mut AppState,
    action: &TabAction,
    di: &DIContainer,
) -> AppResult<()> {
    match (state.ui_state.current_tab, action) {
        (Tab::Agent, _) => agent_effects(state, action, di).await,
        (Tab::Files, _) => files_effects(state, action, di).await,
        (Tab::Git, _) => git_effects(state, action, di).await,
        (Tab::Packages, TabAction::Refresh) => services::refresh_packages(state, di).await,
        (Tab::Database, _) => database_effects(state, action, di).await,
        (Tab::Logs, _) => logs_effects(state, action, di).await,
        (Tab::System, TabAction::Refresh) | (Tab::System, TabAction::Select) => {
            services::refresh_system_metrics(state, di).await
        }
        (Tab::Skills, _) => skills_effects(state, action, di).await,
        (Tab::Terminal, TabAction::Execute) => {
            let command = state.terminal_tab_state.terminal_input.clone();
            run_shell_command(state, &command, ShellTarget::Terminal, di).await;
        }
        (Tab::Cli, TabAction::RunCommand(command)) => {
            let command = command.clone();
            state.cli_tab_state.command_input.clear();
            run_shell_command(state, &command, ShellTarget::Cli, di).await;
        }
        (Tab::Settings, TabAction::ApplySettings) => settings_effects(state),
        _ => {}
    }
    Ok(())
}

/// Agent tab: persist sessions/messages via the session repository.
async fn agent_effects(state: &mut AppState, action: &TabAction, di: &DIContainer) {
    let Some(repo) = di.session_repo() else {
        return;
    };

    match action {
        TabAction::StartSession => {
            let Some(id) = state.agent_tab_state.session_id.clone() else {
                return;
            };
            let now = chrono::Utc::now();
            let session = Session::create(
                SessionId::from_string(id),
                format!("tui-{}", now.format("%H:%M:%S")),
                now,
                now,
            );
            if repo.save(&session).await.is_ok() {
                services::record_audit_event(
                    di,
                    AuditAction::SessionCreate {
                        name: session.name.clone(),
                    },
                )
                .await;
            }
            services::refresh_agent_sessions(state, di).await;
        }
        TabAction::EndSession => {
            services::refresh_agent_sessions(state, di).await;
        }
        TabAction::SendMessage(content) => {
            let Some(id) = state.agent_tab_state.session_id.clone() else {
                return;
            };
            let session_id = SessionId::from_string(id);
            if let Ok(Some(mut session)) = repo.find_by_id(&session_id).await {
                session.add_message(Message::create(
                    uuid::Uuid::new_v4().to_string(),
                    MessageRole::User,
                    content.clone(),
                    chrono::Utc::now(),
                ));
                let _ = repo.save(&session).await;
            }
            services::record_audit_event(
                di,
                AuditAction::MessageSend {
                    session_id: session_id.as_str().to_string(),
                },
            )
            .await;
        }
        TabAction::Select => {
            let idx = state.agent_tab_state.selected_session_index;
            if let Some(session) = state.agent_tab_state.sessions.get(idx).cloned() {
                state.agent_tab_state.session_id = Some(session.id.as_str().to_string());
                state.agent_tab_state.messages = session
                    .messages
                    .iter()
                    .map(|m| crate::modules::ui::domain::models::AgentMessage {
                        role: m.role.as_str().to_string(),
                        content: m.content.clone(),
                        timestamp: m.created_at,
                    })
                    .collect();
            }
        }
        _ => {}
    }
}

/// Files tab: open directories, preview files, rescan on refresh.
async fn files_effects(state: &mut AppState, action: &TabAction, di: &DIContainer) {
    match action {
        TabAction::OpenFile | TabAction::Select => {
            let idx = state.files_tab_state.selected_file_index;
            let Some(name) = state.files_tab_state.files.get(idx).cloned() else {
                return;
            };
            let base = PathBuf::from(&state.files_tab_state.current_path);

            if let Some(dir) = name.strip_suffix('/') {
                // Descend into directory ("../" goes to the parent).
                let next = if name == "../" {
                    base.parent()
                        .map(|p| p.to_path_buf())
                        .unwrap_or_else(|| base.clone())
                } else {
                    base.join(dir)
                };
                state.files_tab_state.current_path = next.display().to_string();
                state.files_tab_state.selected_file_index = 0;
                state.files_tab_state.preview.clear();
                services::refresh_files(state, di).await;
            } else {
                let path = base.join(&name);
                state.files_tab_state.preview = match tokio::fs::read_to_string(&path).await {
                    Ok(content) => content.lines().take(200).collect::<Vec<_>>().join("\n"),
                    Err(e) => format!("Cannot preview {name}: {e}"),
                };
            }
        }
        TabAction::Refresh => services::refresh_files(state, di).await,
        _ => {}
    }
}

/// Git tab: stage/unstage/commit/push via the git2-backed adapter.
async fn git_effects(state: &mut AppState, action: &TabAction, di: &DIContainer) {
    let Some(git) = di.git_adapter() else {
        return;
    };

    let selected = state.git_tab_state.selected_file_index;
    let unstaged_len = state.git_tab_state.unstaged_files.len();

    match action {
        TabAction::Stage => {
            if let Some(path) = state.git_tab_state.unstaged_files.get(selected).cloned() {
                let _ = git.stage_file(&path);
            }
            services::refresh_git_status(state, di);
        }
        TabAction::Unstage => {
            if let Some(path) = state
                .git_tab_state
                .staged_files
                .get(selected.saturating_sub(unstaged_len))
                .cloned()
            {
                let _ = git.unstage_file(&path);
            }
            services::refresh_git_status(state, di);
        }
        TabAction::Commit(message) => {
            use crate::modules::automation::ports::GitOperations;
            if git.commit(message).await.is_ok() {
                services::record_audit_event(
                    di,
                    AuditAction::GitCommit {
                        message: message.clone(),
                        files: state.git_tab_state.staged_files.clone(),
                    },
                )
                .await;
            }
            services::refresh_git_status(state, di);
        }
        TabAction::Push => {
            use crate::modules::automation::ports::GitOperations;
            if let Some(branch) = git.current_branch_name() {
                let _ = git.push(&branch).await;
            }
            services::refresh_git_status(state, di);
        }
        TabAction::Select => {
            // Load a diff preview for the selected file.
            let (path, staged) = if selected < unstaged_len {
                (
                    state.git_tab_state.unstaged_files.get(selected).cloned(),
                    false,
                )
            } else {
                (
                    state
                        .git_tab_state
                        .staged_files
                        .get(selected - unstaged_len)
                        .cloned(),
                    true,
                )
            };
            state.git_tab_state.diff = match path {
                Some(p) => git
                    .diff_for_file(&p, staged)
                    .unwrap_or_else(|e| format!("(diff unavailable: {e})")),
                None => String::new(),
            };
        }
        TabAction::Refresh => services::refresh_git_status(state, di),
        _ => {}
    }
}

/// Database tab: execute SQL against the share-links SQLite pool.
async fn database_effects(state: &mut AppState, action: &TabAction, di: &DIContainer) {
    let Some(repo) = di.share_link_repo() else {
        return;
    };

    match action {
        TabAction::Execute => {
            let query = state.database_tab_state.query_input.trim().to_string();
            if query.is_empty() {
                return;
            }
            match sqlx::query(sqlx::AssertSqlSafe(query.as_str()))
                .fetch_all(repo.pool())
                .await
            {
                Ok(rows) => {
                    state.database_tab_state.results = format_rows(&rows);
                    services::refresh_database_tables(state, di).await;
                }
                Err(e) => {
                    state.database_tab_state.results = vec![format!("error: {e}")];
                }
            }
            state.database_tab_state.query_input.clear();
            services::record_audit_event(di, AuditAction::CommandExecute { command: query }).await;
        }
        TabAction::Select => {
            // Preview the selected table.
            let idx = state.database_tab_state.selected_table_index;
            if let Some(table) = state.database_tab_state.tables.get(idx).cloned() {
                let sql = format!("SELECT * FROM \"{table}\" LIMIT 20");
                // Table names come from `sqlite_master`, not user input.
                if let Ok(rows) = sqlx::query(sqlx::AssertSqlSafe(sql.as_str()))
                    .fetch_all(repo.pool())
                    .await
                {
                    state.database_tab_state.results = format_rows(&rows);
                }
            }
        }
        TabAction::Refresh => services::refresh_database_tables(state, di).await,
        _ => {}
    }
}

/// Logs tab: reload audit entries (filtering by category when set).
async fn logs_effects(state: &mut AppState, action: &TabAction, di: &DIContainer) {
    match action {
        TabAction::Refresh | TabAction::Filter(_) | TabAction::Clear | TabAction::Select => {
            services::refresh_audit_logs(state, di).await;
        }
        _ => {}
    }
}

/// Skills tab: audit skill usage.
async fn skills_effects(state: &mut AppState, action: &TabAction, di: &DIContainer) {
    let idx = state.skills_tab_state.selected_skill_index;
    let Some(name) = state
        .skills_tab_state
        .skills
        .get(idx)
        .map(|s| s.name.clone())
    else {
        return;
    };

    match action {
        TabAction::LoadSkill | TabAction::RunSkill => {
            services::record_audit_event(di, AuditAction::PluginLoad { name }).await;
        }
        _ => {}
    }
}

/// Settings tab: persist theme/font size via the config loader.
fn settings_effects(state: &mut AppState) {
    let loader = crate::adapters::config::loader::ConfigLoader::new();
    if let Ok(mut settings) = loader.load() {
        if !state.settings_tab_state.theme.is_empty() {
            settings
                .ui
                .theme
                .clone_from(&state.settings_tab_state.theme);
        }
        settings.ui.font_size = state.settings_tab_state.font_size.min(255) as u8;
        let _ = loader.save(&settings);
    }
}

/// Which input/history buffers a shell command belongs to.
enum ShellTarget {
    Terminal,
    Cli,
}

/// Execute a real shell command and capture its output into the tab state.
async fn run_shell_command(
    state: &mut AppState,
    command: &str,
    target: ShellTarget,
    di: &DIContainer,
) {
    if command.trim().is_empty() {
        return;
    }

    let result = if cfg!(windows) {
        tokio::process::Command::new("cmd")
            .args(["/C", command])
            .output()
            .await
    } else {
        tokio::process::Command::new("sh")
            .args(["-c", command])
            .output()
            .await
    };

    let mut lines = vec![format!("$ {command}")];
    match result {
        Ok(out) => {
            lines.extend(
                String::from_utf8_lossy(&out.stdout)
                    .lines()
                    .map(str::to_string),
            );
            lines.extend(
                String::from_utf8_lossy(&out.stderr)
                    .lines()
                    .map(|l| format!("! {l}")),
            );
            if let Some(code) = out.status.code() {
                lines.push(format!("[exit {code}]"));
            }
        }
        Err(e) => lines.push(format!("error: {e}")),
    }

    const MAX_OUTPUT_LINES: usize = 500;
    match target {
        ShellTarget::Terminal => {
            state.terminal_tab_state.history.push(command.to_string());
            state.terminal_tab_state.terminal_input.clear();
            state.terminal_tab_state.output.extend(lines);
            state.terminal_tab_state.output.truncate(MAX_OUTPUT_LINES);
        }
        ShellTarget::Cli => {
            state.cli_tab_state.history.push(command.to_string());
            state.cli_tab_state.output.extend(lines);
            state.cli_tab_state.output.truncate(MAX_OUTPUT_LINES);
        }
    }

    services::record_audit_event(
        di,
        AuditAction::CommandExecute {
            command: command.to_string(),
        },
    )
    .await;
}

/// Format sqlite rows into displayable lines.
fn format_rows(rows: &[sqlx::sqlite::SqliteRow]) -> Vec<String> {
    if rows.is_empty() {
        return vec!["(0 rows)".to_string()];
    }
    rows.iter()
        .take(50)
        .map(|row| {
            (0..row.columns().len())
                .map(|i| cell_to_string(row, i))
                .collect::<Vec<_>>()
                .join(" | ")
        })
        .collect()
}

/// Decode a sqlite cell into a display string (dynamic typing fallbacks).
fn cell_to_string(row: &sqlx::sqlite::SqliteRow, i: usize) -> String {
    if let Ok(v) = row.try_get::<Option<String>, _>(i) {
        return v.unwrap_or_else(|| "NULL".to_string());
    }
    if let Ok(v) = row.try_get::<Option<i64>, _>(i) {
        return v.map_or_else(|| "NULL".to_string(), |v| v.to_string());
    }
    if let Ok(v) = row.try_get::<Option<f64>, _>(i) {
        return v.map_or_else(|| "NULL".to_string(), |v| v.to_string());
    }
    if let Ok(v) = row.try_get::<Option<bool>, _>(i) {
        return v.map_or_else(|| "NULL".to_string(), |v| v.to_string());
    }
    if let Ok(v) = row.try_get::<Option<Vec<u8>>, _>(i) {
        return v.map_or_else(|| "NULL".to_string(), |v| format!("<{} bytes>", v.len()));
    }
    "?".to_string()
}
