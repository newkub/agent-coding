use crate::modules::ui::domain::models::AppState;
use crate::shared::kernel::types::Tab;
use crate::shared::kernel::result::AppResult;
use super::handlers::TabAction;

/// Handle tab-specific actions
pub(crate) fn handle_tab_action(state: &mut AppState, action: TabAction) -> AppResult<()> {
    match state.ui_state.current_tab {
        Tab::Agent => super::handlers::handle_agent_action(state, action),
        Tab::Packages => super::handlers::handle_packages_action(state, action),
        Tab::Files => super::handlers::handle_files_action(state, action),
        Tab::Git => super::handlers::handle_git_action(state, action),
        Tab::Terminal => super::handlers::handle_terminal_action(state, action),
        Tab::Snippets => super::handlers::handle_snippets_action(state, action),
        Tab::Api => super::handlers::handle_api_action(state, action),
        Tab::Database => super::handlers::handle_database_action(state, action),
        Tab::Tasks => super::handlers::handle_tasks_action(state, action),
        Tab::Notes => super::handlers::handle_notes_action(state, action),
        Tab::Logs => super::handlers::handle_logs_action(state, action),
        Tab::System => super::handlers::handle_system_action(state, action),
        Tab::Snippet => super::handlers::handle_snippets_action(state, action),
        Tab::Skills => Ok(()),
        Tab::Workflows => Ok(()),
        Tab::Settings => Ok(()),
        Tab::Cli => Ok(()),
    }
}