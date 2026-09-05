use super::tab_action_types::TabAction;
use crate::modules::ui::domain::models::{AppState, SnippetItem};
use crate::shared::kernel::result::AppResult;

/// Snippet tab action handler (single-snippet editor view)
pub(crate) fn handle_snippet_action(state: &mut AppState, action: TabAction) -> AppResult<()> {
    match action {
        TabAction::CreateSnippet => {
            state.snippet_tab_state.is_editing = true;
            state.snippet_tab_state.edit_content.clear();
        }
        TabAction::SaveSnippet => {
            // Persist the edited content as a real `Snippet` via the domain
            // service and append it to the session-local list.
            let count = state.snippet_tab_state.snippets.len() + 1;
            if let Ok(snippet) = crate::modules::snippet::application::services::create_snippet(
                format!("snippet-{count}"),
                String::new(),
                state.snippet_tab_state.edit_content.clone(),
                "text".to_string(),
            ) {
                state.snippet_tab_state.snippets.push(SnippetItem {
                    id: snippet.id.to_string(),
                    name: snippet.name,
                    language: snippet.language,
                    code: snippet.code,
                });
            }
            state.snippet_tab_state.is_editing = false;
        }
        TabAction::Edit(_, _) => {
            state.snippet_tab_state.is_editing = !state.snippet_tab_state.is_editing;
        }
        _ => {}
    }
    Ok(())
}
