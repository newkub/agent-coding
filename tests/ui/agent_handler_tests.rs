use agent_tui::modules::ui::application::handlers::agent_handler;
use agent_tui::modules::ui::application::handlers::tab_action_types::TabAction;
use agent_tui::modules::ui::domain::models::{AgentMessage, AppState};

#[test]
fn test_handle_start_session() {
    let mut state = AppState::default();
    let action = TabAction::StartSession;

    let result = agent_handler::handle_agent_action(&mut state, action);

    assert!(result.is_ok());
    assert!(state.agent_tab_state.session_id.is_some());
}

#[test]
fn test_handle_end_session() {
    let mut state = AppState::default();
    state.agent_tab_state.session_id = Some("test-session".to_string());
    state.agent_tab_state.messages.push(AgentMessage {
        role: "user".to_string(),
        content: "test".to_string(),
        timestamp: chrono::Utc::now(),
    });

    let action = TabAction::EndSession;
    let result = agent_handler::handle_agent_action(&mut state, action);

    assert!(result.is_ok());
    assert!(state.agent_tab_state.session_id.is_none());
    assert!(state.agent_tab_state.messages.is_empty());
}

#[test]
fn test_handle_send_message() {
    let mut state = AppState::default();
    let action = TabAction::SendMessage("Hello, World!".to_string());

    let result = agent_handler::handle_agent_action(&mut state, action);

    assert!(result.is_ok());
    assert_eq!(state.agent_tab_state.messages.len(), 1);
    assert_eq!(state.agent_tab_state.messages[0].role, "user");
    assert_eq!(state.agent_tab_state.messages[0].content, "Hello, World!");
}
