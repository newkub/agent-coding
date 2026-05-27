pub mod agent_handler;
mod cli_handler;
mod files_handler;
mod git_handler;
mod quick_action;
mod settings_handler;
mod skills_handler;
mod snippet_handler;
pub mod tab_action_types;
mod workflows_handler;

// New tab handlers
mod packages_handler;
mod terminal_handler;
mod snippets_handler;
mod api_handler;
mod database_handler;
mod tasks_handler;
mod notes_handler;
mod logs_handler;
mod system_handler;

pub use agent_handler::handle_agent_action;
pub(crate) use files_handler::handle_files_action;
pub(crate) use git_handler::handle_git_action;
pub use tab_action_types::TabAction;

// New tab handler exports
pub(crate) use packages_handler::handle_packages_action;
pub(crate) use terminal_handler::handle_terminal_action;
pub(crate) use snippets_handler::handle_snippets_action;
pub(crate) use api_handler::handle_api_action;
pub(crate) use database_handler::handle_database_action;
pub(crate) use tasks_handler::handle_tasks_action;
pub(crate) use notes_handler::handle_notes_action;
pub(crate) use logs_handler::handle_logs_action;
pub(crate) use system_handler::handle_system_action;

