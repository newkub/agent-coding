use super::TabAction;
use crate::modules::ui::domain::models::AppState;
use crate::shared::kernel::result::AppResult;

pub(crate) fn handle_system_action(state: &mut AppState, action: TabAction) -> AppResult<()> {
    match action {
        TabAction::Refresh => {
            state.system_tab_state.selected_metric_index = 0;
        }
        TabAction::Clear => {
            state.system_tab_state.selected_metric_index = 0;
        }
        _ => {}
    }
    Ok(())
}
