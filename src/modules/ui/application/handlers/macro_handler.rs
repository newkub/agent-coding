use super::tab_action_types::TabAction;
use crate::modules::ui::domain::models::AppState;
use crate::shared::kernel::result::AppResult;

/// Macros tab action handler.
///
/// The synchronous part only mutates `AppState`; repository/executor calls
/// run in `tab_effects::macros_effects`.
pub(crate) fn handle_macros_action(state: &mut AppState, action: TabAction) -> AppResult<()> {
    match action {
        TabAction::Input(text) => {
            state.macros_tab_state.input = text;
        }
        TabAction::StartRecording => {
            state.macros_tab_state.recording = true;
        }
        TabAction::StopRecording => {
            // `recording_id` stays set so the effect layer can finish the
            // recording in the repository; it clears the id afterwards.
            state.macros_tab_state.recording = false;
        }
        TabAction::Delete => {
            // Deletion runs in the effect layer (repository); here we only
            // keep the selection within bounds once the list is refreshed.
            state.macros_tab_state.selected_index = state
                .macros_tab_state
                .selected_index
                .min(state.macros_tab_state.macros.len().saturating_sub(1));
        }
        _ => {}
    }
    Ok(())
}
