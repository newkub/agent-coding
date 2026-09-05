use super::handlers::TabAction;
use crate::modules::ui::domain::models::AppState;
use crate::presentation::tui::di::DIContainer;
use crate::shared::kernel::result::AppResult;
use crate::shared::kernel::types::Tab;

/// Handle tab-specific actions.
///
/// First the synchronous per-tab handler updates `AppState`, then the async
/// side effects run against the DI container (repositories, git, filesystem,
/// shell, sqlite, metrics) and refresh the affected state.
pub(crate) async fn handle_tab_action(
    state: &mut AppState,
    action: TabAction,
    di: &DIContainer,
) -> AppResult<()> {
    match state.ui_state.current_tab {
        Tab::Agent => super::handlers::handle_agent_action(state, action.clone()),
        Tab::Packages => super::handlers::handle_packages_action(state, action.clone()),
        Tab::Files => super::handlers::handle_files_action(state, action.clone()),
        Tab::Git => super::handlers::handle_git_action(state, action.clone()),
        Tab::Terminal => super::handlers::handle_terminal_action(state, action.clone()),
        Tab::Snippets => super::handlers::handle_snippets_action(state, action.clone()),
        Tab::Api => super::handlers::handle_api_action(state, action.clone()),
        Tab::Database => super::handlers::handle_database_action(state, action.clone()),
        Tab::Tasks => super::handlers::handle_tasks_action(state, action.clone()),
        Tab::Notes => super::handlers::handle_notes_action(state, action.clone()),
        Tab::Logs => super::handlers::handle_logs_action(state, action.clone()),
        Tab::System => super::handlers::handle_system_action(state, action.clone()),
        Tab::Snippet => super::handlers::handle_snippet_action(state, action.clone()),
        Tab::Skills => super::handlers::handle_skills_action(state, action.clone()),
        Tab::Workflows => super::handlers::handle_workflows_action(state, action.clone()),
        Tab::Settings => super::handlers::handle_settings_action(state, action.clone()),
        Tab::Cli => super::handlers::handle_cli_action(state, action.clone()),
    }?;

    super::tab_effects::apply_tab_effects(state, &action, di).await
}
