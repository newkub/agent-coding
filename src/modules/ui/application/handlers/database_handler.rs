use super::TabAction;
use crate::modules::ui::domain::models::AppState;
use crate::shared::kernel::result::AppResult;

pub(crate) fn handle_database_action(state: &mut AppState, action: TabAction) -> AppResult<()> {
    match action {
        TabAction::Execute => {
            // Execute query - placeholder for future implementation
            state.database_tab_state.query_input.clear();
        }
        TabAction::Clear => {
            state.database_tab_state.query_input.clear();
        }
        TabAction::Refresh => {
            state.database_tab_state.selected_table_index = 0;
        }
        _ => {}
    }
    Ok(())
}
