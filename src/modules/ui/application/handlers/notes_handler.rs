use super::TabAction;
use crate::modules::ui::domain::models::AppState;
use crate::shared::kernel::result::AppResult;

pub(crate) fn handle_notes_action(state: &mut AppState, action: TabAction) -> AppResult<()> {
    match action {
        TabAction::Add(_note) => {
            // Notes are managed through notes_tab_state - placeholder for future implementation
            state.notes_tab_state.selected_note_index = 0;
        }
        TabAction::Remove(index) if index > 0 => {
            state.notes_tab_state.selected_note_index = index - 1;
        }
        TabAction::Edit(_index, _content) => {
            state.notes_tab_state.is_editing = true;
        }
        _ => {}
    }
    Ok(())
}
