use serde::{Deserialize, Serialize};
use crate::shared::kernel::types::Tab;
use super::TabState;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub(crate) struct CliTabState {
    pub command_input: String,
    pub selected_history_index: Option<usize>,
}

impl TabState for CliTabState {
    fn tab(&self) -> Tab {
        Tab::Cli
    }
}