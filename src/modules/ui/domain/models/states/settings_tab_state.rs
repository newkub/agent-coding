use serde::{Deserialize, Serialize};
use crate::shared::kernel::types::Tab;
use super::TabState;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub(crate) struct SettingsTabState {
    pub selected_category_index: usize,
    pub theme: String,
    pub font_size: u16,
}

impl TabState for SettingsTabState {
    fn tab(&self) -> Tab {
        Tab::Settings
    }
}