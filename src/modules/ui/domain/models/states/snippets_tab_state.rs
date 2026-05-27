use serde::{Deserialize, Serialize};
use crate::shared::kernel::types::Tab;
use super::TabState;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub(crate) struct SnippetsTabState {
    pub categories: Vec<String>,
    pub selected_category: Option<String>,
    pub snippets: Vec<Snippet>,
    pub selected_snippet_index: usize,
    pub edit_content: String,
    pub is_editing: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snippet {
    pub id: String,
    pub title: String,
    pub language: String,
    pub content: String,
    pub tags: Vec<String>,
}

impl TabState for SnippetsTabState {
    fn tab(&self) -> Tab {
        Tab::Snippets
    }
}
