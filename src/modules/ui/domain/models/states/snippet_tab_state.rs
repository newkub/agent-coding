use serde::{Deserialize, Serialize};
use crate::shared::kernel::types::Tab;
use super::TabState;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub(crate) struct SnippetTabState {
    pub selected_category: String,
    pub selected_snippet_index: usize,
    pub is_editing: bool,
    pub edit_content: String,
}

impl TabState for SnippetTabState {
    fn tab(&self) -> Tab {
        Tab::Snippet
    }
}