use serde::{Deserialize, Serialize};

/// Snippet tab state
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SnippetTabState {
    pub selected_category: String,
    pub selected_snippet_index: usize,
    pub is_editing: bool,
    pub edit_content: String,
}

/// Skills tab state
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SkillsTabState {
    pub selected_skill_index: usize,
    pub skill_source_filter: Option<String>, // "stakpak", "local", "custom"
}

/// Workflows tab state
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkflowsTabState {
    pub selected_workflow_index: usize,
    pub is_editing: bool,
    pub execution_status: Option<String>,
}

/// Files tab state
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FilesTabState {
    pub current_path: String,
    pub selected_file_index: usize,
    pub is_editing: bool,
    pub show_hidden: bool,
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
}

/// Git tab state
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GitTabState {
    pub selected_file_index: usize,
    pub staged_files: Vec<String>,
    pub unstaged_files: Vec<String>,
}

/// Agent tab state
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AgentTabState {
    pub session_id: Option<String>,
    pub messages: Vec<AgentMessage>,
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
}

/// Logs tab state
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LogsTabState {
    pub log_level_filter: Option<String>,
    pub selected_log_index: usize,
}

/// Notes tab state
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NotesTabState {
    pub selected_note_index: usize,
    pub is_editing: bool,
}

/// Packages tab state
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PackagesTabState {
    pub selected_package_index: usize,
    pub show_outdated: bool,
}

/// System tab state
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SystemTabState {
    pub selected_metric_index: usize,
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
}

/// Terminal tab state
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TerminalTabState {
    pub terminal_input: String,
    pub selected_history_index: Option<usize>,
}
