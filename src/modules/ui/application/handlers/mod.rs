pub mod agent_handler;
mod cli_handler;
mod collaboration_handler;
mod files_handler;
mod git_handler;
mod macro_handler;
mod quick_action;
mod settings_handler;
mod skills_handler;
mod snippet_handler;
pub mod tab_action_types;
mod workflows_handler;

// New tab handlers
mod api_handler;
mod database_handler;
mod logs_handler;
mod notes_handler;
mod packages_handler;
mod snippets_handler;
mod system_handler;
mod tasks_handler;
mod terminal_handler;

pub use agent_handler::handle_agent_action;
pub(crate) use cli_handler::handle_cli_action;
pub(crate) use collaboration_handler::handle_collaboration_action;
pub(crate) use files_handler::handle_files_action;
pub(crate) use git_handler::handle_git_action;
pub(crate) use macro_handler::handle_macros_action;
pub(crate) use settings_handler::handle_settings_action;
pub(crate) use skills_handler::handle_skills_action;
pub(crate) use snippet_handler::handle_snippet_action;
pub use tab_action_types::TabAction;
pub(crate) use workflows_handler::handle_workflows_action;

// New tab handler exports
pub(crate) use api_handler::handle_api_action;
pub(crate) use database_handler::handle_database_action;
pub(crate) use logs_handler::handle_logs_action;
pub(crate) use notes_handler::handle_notes_action;
pub(crate) use packages_handler::handle_packages_action;
pub(crate) use snippets_handler::handle_snippets_action;
pub(crate) use system_handler::handle_system_action;
pub(crate) use tasks_handler::handle_tasks_action;
pub(crate) use terminal_handler::handle_terminal_action;
