use super::TabAction;
use crate::modules::ui::domain::models::AppState;
use crate::shared::kernel::result::AppResult;

pub(crate) fn handle_terminal_action(state: &mut AppState, action: TabAction) -> AppResult<()> {
    match action {
        TabAction::Input(text) => {
            state.terminal_tab_state.terminal_input = text;
        }
        TabAction::Execute => {
            // Execute command - for now just store in history
            state.terminal_tab_state.selected_history_index = None;
        }
        TabAction::Clear => {
            state.terminal_tab_state.terminal_input.clear();
        }
        _ => {}
    }
    Ok(())
}
