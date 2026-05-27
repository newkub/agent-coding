use serde::{Deserialize, Serialize};
use crate::shared::kernel::types::Tab;
use super::TabState;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub(crate) struct FilesTabState {
    pub current_path: String,
    pub selected_file_index: usize,
    pub is_editing: bool,
    pub show_hidden: bool,
}

impl TabState for FilesTabState {
    fn tab(&self) -> Tab {
        Tab::Files
    }
}