pub(crate) mod core_tabs;
pub(crate) mod data_tabs;
pub(crate) mod development_tabs;
pub(crate) mod system_tabs;

// Re-exports
pub(crate) use core_tabs::{
    navigate_agent_tab, navigate_files_tab, navigate_git_tab, navigate_terminal_tab,
};
pub(crate) use data_tabs::{
    navigate_api_tab, navigate_database_tab, navigate_notes_tab, navigate_tasks_tab,
};
pub(crate) use development_tabs::{navigate_packages_tab, navigate_snippets_tab};
pub(crate) use system_tabs::{navigate_logs_tab, navigate_system_tab};
