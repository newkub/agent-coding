use super::tab_action_types::TabAction;
use crate::modules::ui::domain::models::AppState;
use crate::shared::kernel::result::AppResult;

/// Collaboration tab action handler.
///
/// The synchronous part only mutates `AppState`; repository calls run in
/// `tab_effects::collaboration_effects`.
pub(crate) fn handle_collaboration_action(
    state: &mut AppState,
    action: TabAction,
) -> AppResult<()> {
    match action {
        TabAction::Input(text) => {
            state.collaboration_tab_state.input = text;
        }
        TabAction::Select => {
            // Reflect the selected session into the legacy collaboration state
            // so the rest of the app can observe the joined session.
            let idx = state.collaboration_tab_state.selected_session_index;
            if let Some(session) = state.collaboration_tab_state.sessions.get(idx) {
                state.collaboration_state.session_id = Some(session.id.as_str().to_string());
                state.collaboration_state.participants = session.participants.clone();
            }
        }
        TabAction::Join => {
            state.collaboration_state.is_active = true;
        }
        TabAction::Leave => {
            // Session/participant ids stay set so the effect layer can call
            // `leave_session` on the repository; it clears them afterwards.
        }
        TabAction::SendMessage(_) => {
            state.collaboration_tab_state.input.clear();
        }
        _ => {}
    }
    Ok(())
}
