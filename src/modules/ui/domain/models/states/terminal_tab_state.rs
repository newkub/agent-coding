use serde::{Deserialize, Serialize};
use crate::shared::kernel::types::Tab;
use super::TabState;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub(crate) struct TerminalTabState {
    pub sessions: Vec<TerminalSession>,
    pub active_session_index: usize,
    pub command_input: String,
    pub output: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalSession {
    pub id: String,
    pub name: String,
    pub command: String,
    pub is_active: bool,
}

impl TabState for TerminalTabState {
    fn tab(&self) -> Tab {
        Tab::Terminal
    }
}
