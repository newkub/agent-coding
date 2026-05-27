use serde::{Deserialize, Serialize};
use crate::shared::kernel::types::Tab;
use super::TabState;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub(crate) struct SkillsTabState {
    pub selected_skill_index: usize,
    pub skill_source_filter: Option<String>,
}

impl TabState for SkillsTabState {
    fn tab(&self) -> Tab {
        Tab::Skills
    }
}