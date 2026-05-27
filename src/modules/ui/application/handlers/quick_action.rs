use crate::modules::ui::domain::models::AppState;
use crate::shared::kernel::types::Tab;
use crate::shared::kernel::result::AppResult;
use super::{
    agent_handler::handle_agent_action,
    cli_handler::handle_cli_action,
    files_handler::handle_files_action,
    git_handler::handle_git_action,
    settings_handler::handle_settings_action,
    skills_handler::handle_skills_action,
    snippet_handler::handle_snippet_action,
    tab_action_types::TabAction,
    workflows_handler::handle_workflows_action,
};

/// Quick action mapping (keys 1-9)
pub(super) fn handle_quick_action(state: &mut AppState, number: u8) -> AppResult<()> {
    match state.ui_state.current_tab {
        Tab::Agent => {
            match number {
                1 => handle_agent_action(state, TabAction::Execute)?,
                2 => handle_agent_action(state, TabAction::Refresh)?,
                3 => handle_agent_action(state, TabAction::SendMessage("Review request".to_string()))?,
                4 => handle_agent_action(state, TabAction::EndSession)?,
                _ => {}
            }
        }
        Tab::Git => {
            match number {
                1 => handle_git_action(state, TabAction::Stage)?,
                2 => handle_git_action(state, TabAction::Unstage)?,
                3 => handle_git_action(state, TabAction::Commit("Update".to_string()))?,
                4 => handle_git_action(state, TabAction::Push)?,
                _ => {}
            }
        }
        Tab::Cli => {
            let commands = ["cargo build", "cargo test", "cargo fmt", "cargo clippy", "cargo run"];
            if let Some(cmd) = commands.get((number - 1) as usize) {
                handle_cli_action(state, TabAction::RunCommand(cmd.to_string()))?;
            }
        }
        Tab::Snippet => {
            match number {
                1 => handle_snippet_action(state, TabAction::CreateSnippet)?,
                2 => handle_snippet_action(state, TabAction::Edit(0, String::new()))?,
                3 => handle_snippet_action(state, TabAction::SaveSnippet)?,
                4 => handle_snippet_action(state, TabAction::RunSnippet)?,
                _ => {}
            }
        }
        Tab::Skills => {
            match number {
                1 => handle_skills_action(state, TabAction::LoadSkill)?,
                2 => handle_skills_action(state, TabAction::RunSkill)?,
                _ => {}
            }
        }
        Tab::Workflows => {
            match number {
                1 => handle_workflows_action(state, TabAction::RunWorkflow)?,
                2 => handle_workflows_action(state, TabAction::Edit(0, String::new()))?,
                3 => handle_workflows_action(state, TabAction::StopWorkflow)?,
                _ => {}
            }
        }
        Tab::Files => {
            match number {
                1 => handle_files_action(state, TabAction::OpenFile)?,
                2 => handle_files_action(state, TabAction::CreateFile)?,
                3 => handle_files_action(state, TabAction::Refresh)?,
                _ => {}
            }
        }
        Tab::Settings => {
            match number {
                1 => handle_settings_action(state, TabAction::ApplySettings)?,
                2 => handle_settings_action(state, TabAction::ResetDefaults)?,
                _ => {}
            }
        }
        _ => {}
    }
    Ok(())
}
