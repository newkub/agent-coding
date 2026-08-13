//! Session domain tests - 100% coverage target

mod message_tests;
mod metadata_tests;
mod session_id_tests;
mod session_model_tests;
mod validation_tests;

use agent_tui::modules::session::domain::models::*;
use agent_tui::modules::session::domain::operations::*;
use agent_tui::modules::session::types::*;
use chrono::Utc;

fn create_test_message(role: MessageRole, content: String) -> Message {
    Message::create(uuid::Uuid::new_v4().to_string(), role, content, Utc::now())
}

fn create_test_session(name: String) -> Session {
    Session::create(
        SessionId::from_string(uuid::Uuid::new_v4().to_string()),
        name,
        Utc::now(),
        Utc::now(),
    )
}
