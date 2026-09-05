//! Async side effects for tab actions, backed by the DI container.
//!
//! The synchronous handlers in `handlers/` update `AppState`; this module
//! performs the real I/O (repositories, git, filesystem, shell, sqlite,
//! metrics) and refreshes the affected state afterwards.

use super::handlers::TabAction;
use super::services;
use crate::adapters::config::loader::load_automation_config;
use crate::adapters::external::subagent_task_executor::DefaultSubagentTaskExecutor;
use crate::modules::audit::domain::models::AuditAction;
use crate::modules::automation::domain::models::issue_pr::AutomationWorkflow;
use crate::modules::automation::ports::{AutomationWorkflowExecutor, GitHubClient};
use crate::modules::collaboration::application::usecases as collaboration_usecases;
use crate::modules::collaboration::domain::models::{
    CollaborationId, Participant, ParticipantId, ParticipantRole, SharedMessageType,
};
use crate::modules::macros::application::usecases as macro_usecases;
use crate::modules::macros::domain::models::MacroId;
use crate::modules::session::domain::models::{Message, MessageRole, Session, SessionId};
use crate::modules::subagents::domain::models::subagent::{SubagentTask, TaskContext, TaskType};
use crate::modules::subagents::ports::SubagentTaskExecutor;
use crate::modules::ui::domain::models::{AppState, ToastKind};
use crate::modules::ui::ports::UiContentRepository;
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
        (Tab::Api, _) => api_effects(state, action).await,
        (Tab::Database, _) => database_effects(state, action, di).await,
        (Tab::Logs, _) => logs_effects(state, action, di).await,
        (Tab::System, _) => system_effects(state, action, di).await,
        (Tab::Skills, _) => skills_effects(state, action, di).await,
        (Tab::Collaboration, _) => collaboration_effects(state, action, di).await,
        (Tab::Macros, _) => macros_effects(state, action, di).await,
        (Tab::Workflows, _) => workflows_effects(state, action, di).await,
        (Tab::Terminal, TabAction::Execute) => {
            let command = state.terminal_tab_state.terminal_input.clone();
            run_shell_command(state, &command, ShellTarget::Terminal, di).await;
        }
        (Tab::Terminal, _) => headless_session_effects(state, action, di).await,
        (Tab::Cli, TabAction::RunCommand(command)) => {
            let command = command.clone();
            state.cli_tab_state.command_input.clear();
            run_shell_command(state, &command, ShellTarget::Cli, di).await;
        }
        (
            Tab::Tasks,
            TabAction::Add(_) | TabAction::Edit(_, _) | TabAction::Remove(_) | TabAction::Toggle(_),
        ) => {
            services::persist_tasks(state);
        }
        (Tab::Notes, TabAction::Add(_) | TabAction::Edit(_, _) | TabAction::Remove(_)) => {
            persist_notes(state, di).await;
        }
        (
            Tab::Snippets | Tab::Snippet,
            TabAction::SaveSnippet | TabAction::Edit(_, _) | TabAction::Remove(_),
        ) => {
            persist_snippets(state, di).await;
        }
        (Tab::Settings, TabAction::ApplySettings) => settings_effects(state),
        _ => {}
    }
    Ok(())
}

/// Persist the Notes tab state and reload it from SQLite.
async fn persist_notes(state: &mut AppState, di: &DIContainer) {
    let Some(repo) = di.ui_content_repo() else {
        return;
    };
    match repo.replace_notes(&state.notes_tab_state.notes).await {
        Ok(()) => services::refresh_notes(state, di).await,
        Err(e) => state.push_toast(ToastKind::Error, format!("Failed to save notes: {e}")),
    }
}

/// Persist the Snippet(s) tab state and reload it from SQLite.
async fn persist_snippets(state: &mut AppState, di: &DIContainer) {
    let Some(repo) = di.ui_content_repo() else {
        return;
    };
    match repo
        .replace_snippets(&state.snippet_tab_state.snippets)
        .await
    {
        Ok(()) => services::refresh_snippets(state, di).await,
        Err(e) => state.push_toast(ToastKind::Error, format!("Failed to save snippets: {e}")),
    }
}

/// API tab: execute the configured request through the `api-tui` HTTP client.
async fn api_effects(state: &mut AppState, action: &TabAction) {
    if !matches!(action, TabAction::Execute) || !state.api_tab_state.is_executing {
        return;
    }

    let url = state.api_tab_state.request_url.clone();
    if url.is_empty() {
        state.api_tab_state.response = "Error: request URL is empty".to_string();
        state.api_tab_state.is_executing = false;
        state.push_toast(ToastKind::Error, "API request URL is empty".to_string());
        return;
    }

    let method = match state.api_tab_state.request_method.as_str() {
        "POST" => api_tui::HttpMethod::Post,
        "PUT" => api_tui::HttpMethod::Put,
        "DELETE" => api_tui::HttpMethod::Delete,
        "PATCH" => api_tui::HttpMethod::Patch,
        _ => api_tui::HttpMethod::Get,
    };
    let body = if state.api_tab_state.request_body.is_empty() {
        None
    } else {
        Some(state.api_tab_state.request_body.clone())
    };
    let request = api_tui::ApiRequest {
        url,
        method,
        headers: vec![],
        body,
    };

    match api_tui::adapters::http_client::HttpClient::new()
        .execute(&request)
        .await
    {
        Ok(resp) => {
            state.api_tab_state.response = format!("Status: {}\n\n{}", resp.status, resp.body);
        }
        Err(e) => {
            state.api_tab_state.response = format!("Error: {e}");
            state.push_toast(ToastKind::Error, format!("API request failed: {e}"));
        }
    }
    state.api_tab_state.is_executing = false;
}

/// Workflows tab: run or cancel issue-to-PR automation against the real
/// GitHub/Git adapters and the configured `AutomationConfig`.
async fn workflows_effects(state: &mut AppState, action: &TabAction, di: &DIContainer) {
    match action {
        TabAction::RunWorkflow => {
            let input = state
                .workflows_tab_state
                .automation_input
                .trim()
                .to_string();
            let Some((repository, number_str)) = input.split_once('#') else {
                state.workflows_tab_state.execution_status = Some("invalid input".to_string());
                state.push_toast(
                    ToastKind::Error,
                    "Use owner/repo#issue_number to run automation".to_string(),
                );
                return;
            };
            let Ok(number) = number_str.trim().parse::<u32>() else {
                state.workflows_tab_state.execution_status =
                    Some("invalid issue number".to_string());
                state.push_toast(ToastKind::Error, "Invalid issue number".to_string());
                return;
            };
            let Some(github) = di.github_client() else {
                state.workflows_tab_state.execution_status = Some("github unavailable".to_string());
                state.push_toast(ToastKind::Error, "GitHub client not configured".to_string());
                return;
            };
            let Some(use_case) = di.execute_automation_use_case() else {
                state.workflows_tab_state.execution_status =
                    Some("automation unavailable".to_string());
                state.push_toast(
                    ToastKind::Error,
                    "Automation use case not configured".to_string(),
                );
                return;
            };
            let issue = match github.get_issue(repository, number).await {
                Ok(issue) => issue,
                Err(e) => {
                    state.workflows_tab_state.execution_status = Some(format!("failed: {e}"));
                    state.push_toast(ToastKind::Error, format!("Failed to load issue: {e}"));
                    return;
                }
            };
            let mut workflow = AutomationWorkflow::new(issue);
            let config = load_automation_config();
            match use_case.execute(&mut workflow, &config).await {
                Ok(()) => {
                    state.workflows_tab_state.execution_status = Some("completed".to_string());
                    state.workflows_tab_state.current_workflow_id = Some(workflow.id.clone());
                    state.workflows_tab_state.workflows.push(
                        crate::modules::ui::domain::models::WorkflowItem {
                            name: format!(
                                "{}#{}",
                                workflow.issue.repository, workflow.issue.number
                            ),
                            status: format!("{:?}", workflow.status),
                            steps: workflow
                                .steps
                                .iter()
                                .map(|s| format!("{}: {:?}", s.name, s.status))
                                .collect(),
                        },
                    );
                }
                Err(e) => {
                    state.workflows_tab_state.execution_status = Some(format!("failed: {e}"));
                    state.push_toast(ToastKind::Error, format!("Automation failed: {e}"));
                }
            }
        }
        TabAction::StopWorkflow => {
            let Some(workflow_id) = state.workflows_tab_state.current_workflow_id.clone() else {
                state.workflows_tab_state.execution_status =
                    Some("no workflow running".to_string());
                return;
            };
            let Some(use_case) = di.execute_automation_use_case() else {
                state.workflows_tab_state.execution_status =
                    Some("automation unavailable".to_string());
                return;
            };
            match use_case.cancel(&workflow_id).await {
                Ok(()) => {
                    state.workflows_tab_state.execution_status = Some("cancelled".to_string());
                    state.workflows_tab_state.current_workflow_id = None;
                }
                Err(e) => {
                    state.workflows_tab_state.execution_status =
                        Some(format!("cancel failed: {e}"));
                    state.push_toast(ToastKind::Error, format!("Cancel failed: {e}"));
                }
            }
        }
        _ => {}
    }
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

            let executor = DefaultSubagentTaskExecutor::new();
            let mut task = SubagentTask::new(
                "assistant".to_string(),
                TaskType::Custom("chat".to_string()),
                content.clone(),
                TaskContext::new().with_session(session_id.as_str().to_string()),
            );
            match executor.execute_task(&mut task).await {
                Ok(()) => {
                    if let Some(output) = task.output {
                        state.agent_tab_state.messages.push(
                            crate::modules::ui::domain::models::AgentMessage {
                                role: "agent".to_string(),
                                content: output.clone(),
                                timestamp: chrono::Utc::now(),
                            },
                        );
                        if let Ok(Some(mut session)) = repo.find_by_id(&session_id).await {
                            session.add_message(Message::create(
                                uuid::Uuid::new_v4().to_string(),
                                MessageRole::Assistant,
                                output,
                                chrono::Utc::now(),
                            ));
                            let _ = repo.save(&session).await;
                        }
                        services::record_audit_event(
                            di,
                            AuditAction::AiRequest {
                                model: std::env::var("SUBAGENT_MODEL")
                                    .unwrap_or_else(|_| "gpt-4o-mini".to_string()),
                                tokens: 0,
                            },
                        )
                        .await;
                    }
                }
                Err(e) => {
                    state.push_toast(ToastKind::Error, format!("Assistant response failed: {e}"));
                }
            }
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

/// Collaboration tab: sessions and shared messages via the collaboration
/// repository use cases.
async fn collaboration_effects(state: &mut AppState, action: &TabAction, di: &DIContainer) {
    let Some(repo) = di.collaboration_repo() else {
        return;
    };

    match action {
        TabAction::Refresh => services::refresh_collaboration_sessions(state, di).await,
        TabAction::Select => {
            // Load the message history of the selected session.
            let idx = state.collaboration_tab_state.selected_session_index;
            if let Some(session) = state.collaboration_tab_state.sessions.get(idx).cloned() {
                if let Ok(messages) = repo.get_messages(&session.id).await {
                    state.collaboration_tab_state.messages = messages;
                }
            }
        }
        TabAction::Create => {
            let name = state.collaboration_tab_state.input.trim().to_string();
            if name.is_empty() {
                return;
            }
            let ai_session = state
                .agent_tab_state
                .session_id
                .clone()
                .unwrap_or_else(|| "tui".to_string());
            match collaboration_usecases::create_session(
                repo,
                name,
                "local-user".to_string(),
                ai_session,
            )
            .await
            {
                Ok(session) => {
                    state.collaboration_tab_state.input.clear();
                    state.collaboration_tab_state.is_host = true;
                    state.collaboration_tab_state.local_participant_id = session
                        .participants
                        .first()
                        .map(|p| p.id.as_str().to_string());
                    state.join_collaboration(session.id.as_str().to_string());
                    state.collaboration_state.participants = session.participants.clone();
                }
                Err(e) => {
                    state.push_toast(ToastKind::Error, format!("Create session failed: {e}"));
                }
            }
            services::refresh_collaboration_sessions(state, di).await;
        }
        TabAction::Join => {
            let idx = state.collaboration_tab_state.selected_session_index;
            let Some(session) = state.collaboration_tab_state.sessions.get(idx).cloned() else {
                return;
            };
            let participant = Participant {
                id: ParticipantId::from_string(uuid::Uuid::new_v4().to_string()),
                name: "local-user".to_string(),
                role: ParticipantRole::Editor,
                joined_at: chrono::Utc::now(),
                is_online: true,
                cursor_position: None,
            };
            match collaboration_usecases::join_session(repo, &session.id, participant.clone()).await
            {
                Ok(updated) => {
                    state.collaboration_tab_state.is_host = false;
                    state.collaboration_tab_state.local_participant_id =
                        Some(participant.id.as_str().to_string());
                    state.join_collaboration(updated.id.as_str().to_string());
                    state.collaboration_state.participants = updated.participants.clone();
                    if let Ok(messages) = repo.get_messages(&updated.id).await {
                        state.collaboration_tab_state.messages = messages;
                    }
                }
                Err(e) => {
                    state.push_toast(ToastKind::Error, format!("Join session failed: {e}"));
                }
            }
            services::refresh_collaboration_sessions(state, di).await;
        }
        TabAction::Leave => {
            let session_id = state.collaboration_state.session_id.clone();
            let participant_id = state.collaboration_tab_state.local_participant_id.clone();
            if let (Some(sid), Some(pid)) = (session_id, participant_id) {
                let cid = CollaborationId::from_string(sid);
                let pid = ParticipantId::from_string(pid);
                if let Err(e) = collaboration_usecases::leave_session(repo, &cid, &pid).await {
                    state.push_toast(ToastKind::Error, format!("Leave session failed: {e}"));
                }
            }
            state.leave_collaboration();
            state.collaboration_tab_state.is_host = false;
            state.collaboration_tab_state.local_participant_id = None;
            state.collaboration_tab_state.messages.clear();
            services::refresh_collaboration_sessions(state, di).await;
        }
        TabAction::SendMessage(content) => {
            let session_id = state.collaboration_state.session_id.clone();
            let sender_id = state.collaboration_tab_state.local_participant_id.clone();
            let (Some(sid), Some(pid)) = (session_id, sender_id) else {
                return;
            };
            let cid = CollaborationId::from_string(sid);
            match collaboration_usecases::send_message(
                repo,
                &cid,
                ParticipantId::from_string(pid),
                content.clone(),
                SharedMessageType::Chat,
            )
            .await
            {
                Ok(_) => {
                    if let Ok(messages) = repo.get_messages(&cid).await {
                        state.collaboration_tab_state.messages = messages;
                    }
                }
                Err(e) => {
                    state.push_toast(ToastKind::Error, format!("Send message failed: {e}"));
                }
            }
        }
        _ => {}
    }
}

/// Macros tab: recording/playback/deletion via the macro repository and
/// executor use cases.
async fn macros_effects(state: &mut AppState, action: &TabAction, di: &DIContainer) {
    match action {
        TabAction::Refresh => services::refresh_macros(state, di).await,
        TabAction::StartRecording => {
            let Some(repo) = di.macro_repo() else {
                return;
            };
            let name = {
                let input = state.macros_tab_state.input.trim();
                if input.is_empty() {
                    format!("macro-{}", state.macros_tab_state.macros.len() + 1)
                } else {
                    input.to_string()
                }
            };
            match macro_usecases::start_recording(repo, name, String::new()).await {
                Ok(id) => {
                    state.macros_tab_state.input.clear();
                    state.macros_tab_state.recording_id = Some(id.as_str().to_string());
                    state.macros_tab_state.status = Some(format!("recording {id}"));
                    // Keep the legacy macro state in sync.
                    state.start_macro_recording(id.as_str().to_string());
                }
                Err(e) => {
                    state.macros_tab_state.status = Some(format!("record failed: {e}"));
                }
            }
            services::refresh_macros(state, di).await;
        }
        TabAction::StopRecording => {
            let Some(repo) = di.macro_repo() else {
                return;
            };
            let Some(id_str) = state.macros_tab_state.recording_id.take() else {
                return;
            };
            let id = MacroId::from_string(id_str);
            state.macros_tab_state.status = match macro_usecases::stop_recording(repo, &id).await {
                Ok(Some(m)) => Some(format!("saved {} ({} steps)", m.name, m.step_count())),
                Ok(None) => Some("no active recording".to_string()),
                Err(e) => Some(format!("stop failed: {e}")),
            };
            state.stop_macro_recording();
            services::refresh_macros(state, di).await;
        }
        TabAction::Playback => {
            let (Some(repo), Some(executor)) = (di.macro_repo(), di.macro_executor()) else {
                return;
            };
            let Some(mut macro_def) = state
                .macros_tab_state
                .macros
                .get(state.macros_tab_state.selected_index)
                .cloned()
            else {
                return;
            };
            macro_def.increment_usage();
            let _ = repo.save(&macro_def).await;
            state.macros_tab_state.status =
                match macro_usecases::playback_macro(executor, &macro_def, None).await {
                    Ok(result) => Some(format!(
                        "playback {}: {} step(s), success={}",
                        macro_def.name,
                        result.step_results.len(),
                        result.success
                    )),
                    Err(e) => Some(format!("playback failed: {e}")),
                };
            services::refresh_macros(state, di).await;
        }
        TabAction::Delete => {
            let Some(repo) = di.macro_repo() else {
                return;
            };
            let idx = state.macros_tab_state.selected_index;
            let Some(macro_def) = state.macros_tab_state.macros.get(idx).cloned() else {
                return;
            };
            state.macros_tab_state.status =
                match macro_usecases::delete_macro(repo, &macro_def.id).await {
                    Ok(()) => Some(format!("deleted {}", macro_def.name)),
                    Err(e) => Some(format!("delete failed: {e}")),
                };
            services::refresh_macros(state, di).await;
        }
        _ => {}
    }
}

/// Terminal tab: headless session management via the headless use case.
async fn headless_session_effects(state: &mut AppState, action: &TabAction, di: &DIContainer) {
    let Some(use_case) = di.execute_headless_use_case() else {
        return;
    };

    let selected_session = |state: &AppState| {
        state
            .terminal_tab_state
            .headless_sessions
            .get(state.terminal_tab_state.selected_session_index)
            .cloned()
    };

    let result = match action {
        TabAction::ListSessions => None,
        TabAction::CreateSession => Some(match use_case.create_session().await {
            Ok(id) => format!("created headless session {id}"),
            Err(e) => e.to_string(),
        }),
        TabAction::DeleteSession => match selected_session(state) {
            Some(id) => Some(match use_case.delete_session(&id).await {
                Ok(()) => format!("deleted headless session {id}"),
                Err(e) => e.to_string(),
            }),
            None => Some("no headless session selected".to_string()),
        },
        TabAction::LoadSession => match selected_session(state) {
            Some(id) => Some(match use_case.load_session(&id).await {
                Ok(()) => format!("loaded headless session {id}"),
                Err(e) => e.to_string(),
            }),
            None => Some("no headless session selected".to_string()),
        },
        TabAction::SaveSession => match selected_session(state) {
            Some(id) => Some(match use_case.save_session(&id).await {
                Ok(()) => format!("saved headless session {id}"),
                Err(e) => e.to_string(),
            }),
            None => Some("no headless session selected".to_string()),
        },
        _ => return,
    };

    if let Some(line) = result {
        state.terminal_tab_state.output.push(line);
    }
    services::refresh_headless_sessions(state, di).await;
}

/// System tab: host metrics plus performance analysis, snapshots and
/// optimization suggestions via the performance use case.
async fn system_effects(state: &mut AppState, action: &TabAction, di: &DIContainer) {
    match action {
        TabAction::Refresh | TabAction::Select => {
            services::refresh_system_metrics(state, di).await;
            services::refresh_performance(state, di).await;
        }
        TabAction::Snapshot => {
            if let Some(use_case) = di.analyze_performance_use_case() {
                let name = format!("tui-{}", chrono::Utc::now().format("%H:%M:%S"));
                match use_case.create_snapshot(name).await {
                    Ok(snapshot) => state.push_toast(
                        ToastKind::Success,
                        format!("Snapshot saved: {}", snapshot.name),
                    ),
                    Err(e) => state.push_toast(ToastKind::Error, format!("Snapshot failed: {e}")),
                }
            }
            services::refresh_performance(state, di).await;
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
