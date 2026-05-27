use serde::{Deserialize, Serialize};
use crate::shared::kernel::types::Tab;

/// Tab state trait - all tab-specific states implement this
pub(crate) trait TabState: Send + Sync {
    fn tab(&self) -> Tab;
}

/// Event emitted when tab state changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct TabStateChanged {
    pub tab: Tab,
    pub event_type: String,
}

pub(crate) mod tab_registry;
pub(crate) mod agent_tab_state;
pub(crate) mod git_tab_state;
pub(crate) mod packages_tab_state;
pub(crate) mod terminal_tab_state;
pub(crate) mod snippets_tab_state;
pub(crate) mod api_tab_state;
pub(crate) mod database_tab_state;
pub(crate) mod tasks_tab_state;
pub(crate) mod notes_tab_state;
pub(crate) mod logs_tab_state;
pub(crate) mod system_tab_state;
pub(crate) mod files_tab_state;

// Old states - kept for compatibility but not used in new tab structure
pub(crate) mod cli_tab_state;
pub(crate) mod skills_tab_state;
pub(crate) mod workflows_tab_state;
pub(crate) mod settings_tab_state;

pub(crate) use tab_registry::TabStateRegistry;