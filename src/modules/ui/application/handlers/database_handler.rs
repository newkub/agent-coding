use super::TabAction;
use crate::modules::ui::domain::models::AppState;
use crate::shared::kernel::result::AppResult;
use database_tui::{ConnectionConfig, DatabaseUseCase, SqliteDbPort};

/// Database tab action handler — delegates to database-app
#[allow(clippy::let_underscore_future)]
pub(crate) fn handle_database_action(state: &mut AppState, action: TabAction) -> AppResult<()> {
    let mut uc = DatabaseUseCase::new(
        ConnectionConfig {
            url: String::new(),
            database: String::new(),
        },
        Box::new(SqliteDbPort::new()),
    );
    match action {
        TabAction::Execute => {
            uc.set_query_input(state.database_tab_state.query_input.clone());
            let _ = uc.execute_query();
            state.database_tab_state.query_input.clear();
        }
        TabAction::Clear => {
            state.database_tab_state.query_input.clear();
            uc.clear_query();
        }
        TabAction::Refresh => {
            state.database_tab_state.selected_table_index = 0;
            uc.select_prev_table();
        }
        _ => {}
    }
    Ok(())
}
