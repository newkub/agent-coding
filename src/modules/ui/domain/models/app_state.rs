use crate::shared::kernel::types::{Tab, UIState};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

use super::{
    custom_commands::CustomCommands,
    legacy_states::{
        CollaborationState, DiffReviewState, MacroState, MetricsState, SandboxState, SnippetState,
        TimelineState,
    },
    tab_content::TabContent,
    tab_states::{
        AgentTabState, ApiTabState, CliTabState, DatabaseTabState, FilesTabState, GitTabState,
        LogsTabState, NotesTabState, PackagesTabState, SettingsTabState, SkillsTabState,
        SnippetTabState, SystemTabState, TasksTabState, TerminalTabState, WorkflowsTabState,
    },
    toast::ToastNotification,
};

/// Complete application state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    pub ui_state: UIState,
    // Tab contents for 3-column layout
    pub agent_tab: TabContent,
    pub packages_tab: TabContent,
    pub files_tab: TabContent,
    pub git_tab: TabContent,
    pub terminal_tab: TabContent,
    pub snippets_tab: TabContent,
    pub api_tab: TabContent,
    pub database_tab: TabContent,
    pub tasks_tab: TabContent,
    pub notes_tab: TabContent,
    pub logs_tab: TabContent,
    pub system_tab: TabContent,
    pub skills_tab: TabContent,
    pub workflows_tab: TabContent,
    pub settings_tab: TabContent,
    pub cli_tab: TabContent,
    // Tab-specific states
    pub agent_tab_state: AgentTabState,
    pub packages_tab_state: PackagesTabState,
    pub files_tab_state: FilesTabState,
    pub git_tab_state: GitTabState,
    pub terminal_tab_state: TerminalTabState,
    pub snippet_tab_state: SnippetTabState,
    pub api_tab_state: ApiTabState,
    pub database_tab_state: DatabaseTabState,
    pub tasks_tab_state: TasksTabState,
    pub notes_tab_state: NotesTabState,
    pub logs_tab_state: LogsTabState,
    pub system_tab_state: SystemTabState,
    pub cli_tab_state: CliTabState,
    pub skills_tab_state: SkillsTabState,
    pub workflows_tab_state: WorkflowsTabState,
    pub settings_tab_state: SettingsTabState,
    // Legacy states (kept for compatibility)
    pub diff_state: DiffReviewState,
    pub snippet_state: SnippetState,
    pub macro_state: MacroState,
    pub sandbox_state: SandboxState,
    pub metrics_state: MetricsState,
    pub collaboration_state: CollaborationState,
    pub timeline_state: TimelineState,
    // Toast notifications
    pub toasts: VecDeque<ToastNotification>,
    // Command palette state
    pub show_command_palette: bool,
    pub command_input: String,
    pub command_palette_selected: usize,
    // Help modal state
    pub show_help: bool,
    // Custom commands
    pub custom_commands: CustomCommands,
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

impl AppState {
    pub fn new() -> Self {
        Self {
            ui_state: UIState::new(),
            // Initialize tab contents
            agent_tab: TabContent::with_content(Tab::Agent, "Context", "Chat", "Actions"),
            packages_tab: TabContent::with_content(Tab::Packages, "Sources", "Details", "Actions"),
            files_tab: TabContent::with_content(Tab::Files, "Explorer", "Content", "Actions"),
            git_tab: TabContent::with_content(Tab::Git, "Status", "Diff", "History"),
            terminal_tab: TabContent::with_content(Tab::Terminal, "Sessions", "Output", "Commands"),
            snippets_tab: TabContent::with_content(Tab::Snippets, "Library", "Editor", "Tags"),
            api_tab: TabContent::with_content(Tab::Api, "Collections", "Request", "Response"),
            database_tab: TabContent::with_content(
                Tab::Database,
                "Connections",
                "Query",
                "Results",
            ),
            tasks_tab: TabContent::with_content(Tab::Tasks, "Lists", "Details", "Filters"),
            notes_tab: TabContent::with_content(Tab::Notes, "Folders", "Editor", "Tags"),
            logs_tab: TabContent::with_content(Tab::Logs, "Sources", "Viewer", "Filters"),
            system_tab: TabContent::with_content(Tab::System, "Overview", "Details", "Alerts"),
            skills_tab: TabContent::with_content(Tab::Skills, "Library", "Editor", "Tags"),
            workflows_tab: TabContent::with_content(Tab::Workflows, "Library", "Editor", "Tags"),
            settings_tab: TabContent::with_content(
                Tab::Settings,
                "General",
                "Appearance",
                "Advanced",
            ),
            cli_tab: TabContent::with_content(Tab::Cli, "Input", "Output", "History"),
            // Initialize tab-specific states
            agent_tab_state: AgentTabState::default(),
            packages_tab_state: PackagesTabState::default(),
            files_tab_state: FilesTabState::default(),
            git_tab_state: GitTabState::default(),
            terminal_tab_state: TerminalTabState::default(),
            snippet_tab_state: SnippetTabState::default(),
            api_tab_state: ApiTabState::default(),
            database_tab_state: DatabaseTabState::default(),
            tasks_tab_state: TasksTabState {
                task_manager: task_tui::TaskManagerUseCase::new(),
                ..TasksTabState::default()
            },
            notes_tab_state: NotesTabState::default(),
            logs_tab_state: LogsTabState::default(),
            system_tab_state: SystemTabState::default(),
            cli_tab_state: CliTabState::default(),
            skills_tab_state: SkillsTabState::default(),
            workflows_tab_state: WorkflowsTabState::default(),
            settings_tab_state: SettingsTabState::default(),
            // Legacy states
            diff_state: DiffReviewState::default(),
            snippet_state: SnippetState::default(),
            macro_state: MacroState::default(),
            sandbox_state: SandboxState::default(),
            metrics_state: MetricsState::default(),
            collaboration_state: CollaborationState::default(),
            timeline_state: TimelineState::default(),
            // Toast notifications
            toasts: VecDeque::new(),
            // Command palette state
            show_command_palette: false,
            command_input: String::new(),
            command_palette_selected: 0,
            // Help modal state
            show_help: false,
            // Custom commands
            custom_commands: CustomCommands::get_default_commands(),
        }
    }

    pub const fn current_tab_content(&self) -> &TabContent {
        match self.ui_state.current_tab {
            Tab::Agent => &self.agent_tab,
            Tab::Packages => &self.packages_tab,
            Tab::Files => &self.files_tab,
            Tab::Git => &self.git_tab,
            Tab::Terminal => &self.terminal_tab,
            Tab::Snippet => &self.snippets_tab,
            Tab::Snippets => &self.snippets_tab,
            Tab::Api => &self.api_tab,
            Tab::Database => &self.database_tab,
            Tab::Tasks => &self.tasks_tab,
            Tab::Notes => &self.notes_tab,
            Tab::Logs => &self.logs_tab,
            Tab::System => &self.system_tab,
            Tab::Skills => &self.skills_tab,
            Tab::Workflows => &self.workflows_tab,
            Tab::Settings => &self.settings_tab,
            Tab::Cli => &self.cli_tab,
        }
    }

    pub fn current_tab_content_mut(&mut self) -> &mut TabContent {
        match self.ui_state.current_tab {
            Tab::Agent => &mut self.agent_tab,
            Tab::Packages => &mut self.packages_tab,
            Tab::Files => &mut self.files_tab,
            Tab::Git => &mut self.git_tab,
            Tab::Terminal => &mut self.terminal_tab,
            Tab::Snippet => &mut self.snippets_tab,
            Tab::Snippets => &mut self.snippets_tab,
            Tab::Api => &mut self.api_tab,
            Tab::Database => &mut self.database_tab,
            Tab::Tasks => &mut self.tasks_tab,
            Tab::Notes => &mut self.notes_tab,
            Tab::Logs => &mut self.logs_tab,
            Tab::System => &mut self.system_tab,
            Tab::Skills => &mut self.skills_tab,
            Tab::Workflows => &mut self.workflows_tab,
            Tab::Settings => &mut self.settings_tab,
            Tab::Cli => &mut self.cli_tab,
        }
    }

    // Legacy state methods for backward compatibility

    pub fn start_diff_review(&mut self, diff_text: String) {
        self.diff_state.is_active = true;
        self.diff_state.diff_text = diff_text;
    }

    pub fn end_diff_review(&mut self) {
        self.diff_state.is_active = false;
        self.diff_state.diff_text = String::new();
    }

    pub fn open_snippet_manager(&mut self) {
        self.snippet_state.is_active = true;
    }

    pub fn close_snippet_manager(&mut self) {
        self.snippet_state.is_active = false;
    }

    pub fn start_macro_recording(&mut self, macro_id: String) {
        self.macro_state.is_recording = true;
        self.macro_state.current_macro_id = Some(macro_id);
    }

    pub fn stop_macro_recording(&mut self) {
        self.macro_state.is_recording = false;
        self.macro_state.current_macro_id = None;
    }

    pub fn preview_command(&mut self, _command: &str) {
        self.sandbox_state.is_preview_active = true;
        // command_preview is of type CommandPreview, not String
        // For now, just set the preview flag
    }

    pub fn hide_command_preview(&mut self) {
        self.sandbox_state.is_preview_active = false;
        self.sandbox_state.command_preview = None;
    }

    pub fn open_metrics(&mut self) {
        self.metrics_state.is_active = true;
    }

    pub fn close_metrics(&mut self) {
        self.metrics_state.is_active = false;
    }

    pub fn join_collaboration(&mut self, session_id: String) {
        self.collaboration_state.session_id = Some(session_id);
        self.collaboration_state.is_active = true;
    }

    pub fn leave_collaboration(&mut self) {
        self.collaboration_state.session_id = None;
        self.collaboration_state.is_active = false;
    }
}
