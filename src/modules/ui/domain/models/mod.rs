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
pub use custom_commands::{CustomCommand, CustomCommands, CommandVariable};
pub use keybindings::{KeyBindings, KeyBinding, KeyBindingMode, KeyContext};
pub use legacy_states::{CollaborationState, DiffReviewState, MacroState, MetricsState, SandboxState, SnippetState, TimelineState};
pub use mouse::{MouseState, MousePosition, DragState, DragItemType};
pub use tab_content::TabContent;
pub use tab_states::{AgentMessage, AgentTabState, ApiTabState, CliTabState, DatabaseTabState, FilesTabState, GitTabState, LogsTabState, NotesTabState, PackagesTabState, SettingsTabState, SkillsTabState, SnippetTabState, SystemTabState, TabState, TasksTabState, TerminalTabState, WorkflowsTabState};
pub use toast::{ToastKind, ToastNotification};
