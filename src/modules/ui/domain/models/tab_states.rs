use crate::modules::audit::domain::models::AuditEntry;
use crate::modules::session::domain::models::Session;
use serde::{Deserialize, Serialize};
use task_tui::TaskManagerUseCase;

/// A package entry loaded from the project manifest (Cargo.toml/package.json/...)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PackageItem {
    pub name: String,
    pub version: String,
    pub category: String,
    pub outdated: bool,
}

/// A skill (subagent) entry listed in the Skills tab
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SkillItem {
    pub name: String,
    pub description: String,
    pub status: String,
}

/// A workflow entry listed in the Workflows tab
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkflowItem {
    pub name: String,
    pub status: String,
    pub steps: Vec<String>,
}

/// A note entry listed in the Notes tab
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NoteItem {
    pub title: String,
    pub content: String,
}

/// A snippet entry listed in the Snippet(s) tab
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SnippetItem {
    pub name: String,
    pub language: String,
    pub code: String,
}

/// Snippet tab state
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SnippetTabState {
    pub selected_category: String,
    pub selected_snippet_index: usize,
    pub is_editing: bool,
    pub edit_content: String,
    /// Snippets created during this session
    pub snippets: Vec<SnippetItem>,
}

/// Skills tab state
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SkillsTabState {
    pub selected_skill_index: usize,
    pub skill_source_filter: Option<String>, // "stakpak", "local", "custom"
    /// Available skills (subagents) loaded from the subagent manager
    pub skills: Vec<SkillItem>,
}

/// Workflows tab state
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkflowsTabState {
    pub selected_workflow_index: usize,
    pub is_editing: bool,
    pub execution_status: Option<String>,
    /// Automation workflows available for execution
    pub workflows: Vec<WorkflowItem>,
}

/// Files tab state
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FilesTabState {
    pub current_path: String,
    pub selected_file_index: usize,
    pub is_editing: bool,
    pub show_hidden: bool,
    /// Entries of `current_path` (dirs shown with a trailing '/')
    pub files: Vec<String>,
    /// Preview of the selected file
    pub preview: String,
}

/// Settings tab state
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SettingsTabState {
    pub selected_category_index: usize,
    pub theme: String,
    pub font_size: u16,
}

/// CLI tab state
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CliTabState {
    pub command_input: String,
    pub selected_history_index: Option<usize>,
    /// Commands executed during this session
    pub history: Vec<String>,
    /// Captured output lines of executed commands
    pub output: Vec<String>,
}

/// Git tab state
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GitTabState {
    pub selected_file_index: usize,
    pub staged_files: Vec<String>,
    pub unstaged_files: Vec<String>,
    /// Current branch name (empty when not a git repository)
    pub current_branch: String,
    /// Diff preview of the selected file
    pub diff: String,
}

/// Agent tab state
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AgentTabState {
    pub session_id: Option<String>,
    pub messages: Vec<AgentMessage>,
    /// Sessions loaded from the session repository
    pub sessions: Vec<Session>,
    pub selected_session_index: usize,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AgentMessage {
    pub role: String, // "user" or "agent"
    pub content: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// API tab state
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApiTabState {
    pub selected_endpoint_index: usize,
    pub is_editing: bool,
}

/// Database tab state
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DatabaseTabState {
    pub selected_table_index: usize,
    pub query_input: String,
    /// Tables found in the backing SQLite database
    pub tables: Vec<String>,
    /// Rows produced by the last executed query
    pub results: Vec<String>,
}

/// Logs tab state
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LogsTabState {
    pub log_level_filter: Option<String>,
    pub selected_log_index: usize,
    /// Audit log entries loaded from the audit repository
    pub entries: Vec<AuditEntry>,
}

/// Notes tab state
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NotesTabState {
    pub selected_note_index: usize,
    pub is_editing: bool,
    /// In-memory notes (no notes repository exists yet)
    pub notes: Vec<NoteItem>,
}

/// Packages tab state
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PackagesTabState {
    pub selected_package_index: usize,
    pub show_outdated: bool,
    /// Packages parsed from the project manifest
    pub packages: Vec<PackageItem>,
    /// Package manager detected for the project (cargo/npm/pip/...)
    pub package_manager: String,
}

/// System tab state
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SystemTabState {
    pub selected_metric_index: usize,
    /// Collected metrics as (label, value) pairs
    pub metrics: Vec<(String, String)>,
    /// Alerts raised by threshold checks
    pub alerts: Vec<String>,
}

/// Tab state enum
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TabState {
    Agent(AgentTabState),
    Cli(CliTabState),
    Files(FilesTabState),
    Git(GitTabState),
    Settings(SettingsTabState),
    Skills(SkillsTabState),
    Snippet(SnippetTabState),
    Workflows(WorkflowsTabState),
    Api(ApiTabState),
    Database(DatabaseTabState),
    Logs(LogsTabState),
    Notes(NotesTabState),
    Packages(PackagesTabState),
    System(SystemTabState),
}

/// Tasks tab state
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TasksTabState {
    pub selected_task_index: usize,
    pub show_completed: bool,
    pub task_manager: TaskManagerUseCase,
}

/// Terminal tab state
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TerminalTabState {
    pub terminal_input: String,
    pub selected_history_index: Option<usize>,
    /// Commands executed during this session
    pub history: Vec<String>,
    /// Captured output lines of executed commands
    pub output: Vec<String>,
}
