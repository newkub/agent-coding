use serde::{Deserialize, Serialize};
use crate::shared::kernel::types::Tab;
use super::TabState;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub(crate) struct WorkflowsTabState {
    pub selected_workflow_index: usize,
    pub is_editing: bool,
    pub execution_status: Option<String>,
}

impl TabState for WorkflowsTabState {
    fn tab(&self) -> Tab {
        Tab::Workflows
    }
}