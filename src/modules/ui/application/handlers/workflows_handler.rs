use crate::modules::ui::domain::models::AppState;
use crate::shared::kernel::result::AppResult;
use super::tab_action_types::TabAction;

/// Workflows tab action handler
pub(super) fn handle_workflows_action(state: &mut AppState, action: TabAction) -> AppResult<()> {
    match action {
        TabAction::RunWorkflow => {
            state.workflows_tab_state.execution_status = Some("running".to_string());
            // Run workflow
        }
        TabAction::StopWorkflow => {
            state.workflows_tab_state.execution_status = Some("stopped".to_string());
        }
        TabAction::Edit(_, _) => {
            state.workflows_tab_state.is_editing = !state.workflows_tab_state.is_editing;
        }
        _ => {}
    }
    Ok(())
}
