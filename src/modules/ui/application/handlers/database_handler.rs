use super::tab_action_types::TabAction;
use crate::modules::ui::domain::models::AppState;
use crate::shared::kernel::result::AppResult;

/// Database tab action handler.
///
/// State-only transitions; the real SQL execution runs asynchronously in
/// `tab_effects` against the DI-provided SQLite pool.
pub(crate) fn handle_database_action(state: &mut AppState, action: TabAction) -> AppResult<()> {
    match action {
        TabAction::Clear => {
            state.database_tab_state.query_input.clear();
            state.database_tab_state.results.clear();
        }
        TabAction::Refresh => {
            state.database_tab_state.selected_table_index = 0;
        }
        _ => {}
    }
    Ok(())
}
