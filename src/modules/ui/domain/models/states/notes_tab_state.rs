use serde::{Deserialize, Serialize};
use crate::shared::kernel::types::Tab;
use super::TabState;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub(crate) struct NotesTabState {
    pub notes: Vec<Note>,
    pub selected_note_index: usize,
    pub folders: Vec<String>,
    pub selected_folder: Option<String>,
    pub tags: Vec<String>,
    pub selected_tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub content: String,
    pub folder: Option<String>,
    pub tags: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl TabState for NotesTabState {
    fn tab(&self) -> Tab {
        Tab::Notes
    }
}
