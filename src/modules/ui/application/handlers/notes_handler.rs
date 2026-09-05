use super::tab_action_types::TabAction;
use crate::modules::ui::domain::models::{AppState, NoteItem};
use crate::shared::kernel::result::AppResult;

/// Notes tab action handler
pub(crate) fn handle_notes_action(state: &mut AppState, action: TabAction) -> AppResult<()> {
    match action {
        TabAction::Add(title) => {
            state.notes_tab_state.notes.push(NoteItem {
                title,
                content: String::new(),
            });
            state.notes_tab_state.selected_note_index =
                state.notes_tab_state.notes.len().saturating_sub(1);
        }
        TabAction::Edit(index, content) => {
            if let Some(note) = state.notes_tab_state.notes.get_mut(index) {
                note.content = content;
            }
            state.notes_tab_state.is_editing = false;
        }
        TabAction::Remove(index) => {
            if index < state.notes_tab_state.notes.len() {
                state.notes_tab_state.notes.remove(index);
            }
            state.notes_tab_state.selected_note_index = state
                .notes_tab_state
                .selected_note_index
                .min(state.notes_tab_state.notes.len().saturating_sub(1));
        }
        TabAction::Select => {
            state.notes_tab_state.is_editing = !state.notes_tab_state.is_editing;
        }
        _ => {}
    }
    Ok(())
}
