// Re-export all domain models
pub mod app_commands;
mod app_state;
mod audit;
mod autocomplete;
mod command;
mod custom_commands;
mod keybindings;
mod legacy_states;
mod mouse;
mod tab_content;
mod tab_states;
mod toast;

pub use app_state::AppState;
pub use audit::AuditLog;
pub use autocomplete::{AutocompleteState, Suggestion, SuggestionKind};
pub use command::Command;
pub use custom_commands::{CommandVariable, CustomCommand, CustomCommands};
pub use keybindings::{KeyBinding, KeyBindingMode, KeyBindings, KeyContext};
pub use legacy_states::{
    CollaborationState, DiffReviewState, MacroState, MetricsState, SandboxState, SnippetState,
    TimelineState,
};
pub use mouse::{DragItemType, DragState, MousePosition, MouseState};
pub use tab_content::TabContent;
pub use tab_states::{
    AgentMessage, AgentTabState, ApiTabState, CliTabState, CollaborationTabState, DatabaseTabState,
    FilesTabState, GitTabState, LogsTabState, MacroTabState, NoteItem, NotesTabState, PackageItem,
    PackagesTabState, SettingsTabState, SkillItem, SkillsTabState, SnippetItem, SnippetTabState,
    SystemTabState, TabState, TasksTabState, TerminalTabState, WorkflowItem, WorkflowsTabState,
};
pub use toast::{ToastKind, ToastNotification};
