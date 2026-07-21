use super::tab_action_types::TabAction;
use crate::modules::ui::domain::models::{AgentMessage, AppState};
use crate::shared::kernel::result::AppResult;

/// Agent tab action handler
pub fn handle_agent_action(state: &mut AppState, action: TabAction) -> AppResult<()> {
    match action {
        TabAction::StartSession => {
            state.agent_tab_state.session_id = Some(uuid::Uuid::new_v4().to_string());
        }
        TabAction::EndSession => {
            state.agent_tab_state.session_id = None;
            state.agent_tab_state.messages.clear();
        }
        TabAction::SendMessage(content) => {
            let msg = AgentMessage {
                role: "user".to_string(),
                content,
                timestamp: chrono::Utc::now(),
            };
            state.agent_tab_state.messages.push(msg);
        }
        _ => {}
    }
    Ok(())
}
