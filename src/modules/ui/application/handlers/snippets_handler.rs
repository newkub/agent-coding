use crate::modules::ui::domain::models::AppState;
use crate::shared::kernel::result::AppResult;
use super::TabAction;

pub(crate) fn handle_snippets_action(state: &mut AppState, action: TabAction) -> AppResult<()> {
    match action {
        TabAction::Add(_snippet) => {
            // Snippets are managed through snippet_tab_state - placeholder for future implementation
            state.snippet_tab_state.selected_snippet_index = 0;
        }
        TabAction::Remove(index)
            if index > 0 => {
                state.snippet_tab_state.selected_snippet_index = index - 1;
            }
        TabAction::Edit(_index, content) => {
            state.snippet_tab_state.edit_content = content;
            state.snippet_tab_state.is_editing = true;
        }
        _ => {}
    }
    Ok(())
}
