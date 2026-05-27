use crate::modules::ui::domain::models::AppState;
use crate::shared::kernel::result::AppResult;
use super::tab_action_types::TabAction;

/// Snippet tab action handler
pub(super) fn handle_snippet_action(state: &mut AppState, action: TabAction) -> AppResult<()> {
    match action {
        TabAction::CreateSnippet => {
            state.snippet_tab_state.is_editing = true;
            state.snippet_tab_state.edit_content.clear();
        }
        TabAction::SaveSnippet => {
            state.snippet_tab_state.is_editing = false;
            // Save would persist the content
        }
        TabAction::Edit(_, _) => {
            state.snippet_tab_state.is_editing = !state.snippet_tab_state.is_editing;
        }
        _ => {}
    }
    Ok(())
}
