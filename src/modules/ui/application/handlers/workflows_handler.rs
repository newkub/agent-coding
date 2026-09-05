use super::tab_action_types::TabAction;
use crate::modules::ui::domain::models::AppState;
use crate::shared::kernel::result::AppResult;

/// Workflows tab action handler
pub(crate) fn handle_workflows_action(state: &mut AppState, action: TabAction) -> AppResult<()> {
    match action {
        TabAction::RunWorkflow => {
            state.workflows_tab_state.execution_status = Some("running".to_string());
            if let Some(wf) = state
                .workflows_tab_state
                .workflows
                .get_mut(state.workflows_tab_state.selected_workflow_index)
            {
                wf.status = "running".to_string();
            }
        }
        TabAction::StopWorkflow => {
            state.workflows_tab_state.execution_status = Some("stopped".to_string());
            if let Some(wf) = state
                .workflows_tab_state
                .workflows
                .get_mut(state.workflows_tab_state.selected_workflow_index)
            {
                wf.status = "stopped".to_string();
            }
        }
        TabAction::Edit(_, _) => {
            state.workflows_tab_state.is_editing = !state.workflows_tab_state.is_editing;
        }
        _ => {}
    }
    Ok(())
}
