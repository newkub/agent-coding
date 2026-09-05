use super::tab_action_types::TabAction;
use crate::modules::ui::domain::models::AppState;
use crate::shared::kernel::result::AppResult;

/// Workflows tab action handler — updates state; real execution runs in `tab_effects`.
pub(crate) fn handle_workflows_action(state: &mut AppState, action: TabAction) -> AppResult<()> {
    match action {
        TabAction::Input(content) => {
            state.workflows_tab_state.automation_input = content;
        }
        TabAction::RunWorkflow => {
            state.workflows_tab_state.execution_status = Some("running".to_string());
        }
        TabAction::StopWorkflow => {
            state.workflows_tab_state.execution_status = Some("stopping".to_string());
        }
        TabAction::Edit(_, _) => {
            state.workflows_tab_state.is_editing = !state.workflows_tab_state.is_editing;
        }
        _ => {}
    }
    Ok(())
}
