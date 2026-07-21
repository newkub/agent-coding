use super::TabAction;
use crate::modules::ui::domain::models::AppState;
use crate::shared::kernel::result::AppResult;

pub(crate) fn handle_logs_action(state: &mut AppState, action: TabAction) -> AppResult<()> {
    match action {
        TabAction::Refresh => {
            state.logs_tab_state.selected_log_index = 0;
        }
        TabAction::Clear => {
            state.logs_tab_state.log_level_filter = None;
        }
        TabAction::Filter(filter) => {
            state.logs_tab_state.log_level_filter = Some(filter);
        }
        _ => {}
    }
    Ok(())
}
