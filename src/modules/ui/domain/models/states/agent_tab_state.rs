use serde::{Deserialize, Serialize};
use crate::shared::kernel::types::Tab;
use super::{AutocompleteState, TabState};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub(crate) struct AgentTabState {
    pub session_id: Option<String>,
    pub messages: Vec<AgentMessage>,
    pub input: String,
    pub autocomplete: AutocompleteState,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AgentMessage {
    pub role: String,
    pub content: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl TabState for AgentTabState {
    fn tab(&self) -> Tab {
        Tab::Agent
    }
}