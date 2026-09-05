// Re-exports for layout types if needed

use serde::{Deserialize, Serialize};

/// Main application tabs - 19 tabs total
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Tab {
    #[default]
    Agent,
    Packages,
    Files,
    Git,
    Terminal,
    Snippet,
    Snippets,
    Api,
    Database,
    Tasks,
    Notes,
    Logs,
    System,
    Skills,
    Workflows,
    Settings,
    Cli,
    Collaboration,
    Macros,
}

impl Tab {
    pub const fn all() -> [Self; 19] {
        [
            Self::Agent,
            Self::Packages,
            Self::Files,
            Self::Git,
            Self::Terminal,
            Self::Snippet,
            Self::Snippets,
            Self::Api,
            Self::Database,
            Self::Tasks,
            Self::Notes,
            Self::Logs,
            Self::System,
            Self::Skills,
            Self::Workflows,
            Self::Settings,
            Self::Cli,
            Self::Collaboration,
            Self::Macros,
        ]
    }

    pub const fn next(self) -> Self {
        match self {
            Self::Agent => Self::Packages,
            Self::Packages => Self::Files,
            Self::Files => Self::Git,
            Self::Git => Self::Terminal,
            Self::Terminal => Self::Snippet,
            Self::Snippet => Self::Snippets,
            Self::Snippets => Self::Api,
            Self::Api => Self::Database,
            Self::Database => Self::Tasks,
            Self::Tasks => Self::Notes,
            Self::Notes => Self::Logs,
            Self::Logs => Self::System,
            Self::System => Self::Skills,
            Self::Skills => Self::Workflows,
            Self::Workflows => Self::Settings,
            Self::Settings => Self::Cli,
            Self::Cli => Self::Collaboration,
            Self::Collaboration => Self::Macros,
            Self::Macros => Self::Agent,
        }
    }

    pub const fn prev(self) -> Self {
        match self {
            Self::Agent => Self::Macros,
            Self::Packages => Self::Agent,
            Self::Files => Self::Packages,
            Self::Git => Self::Files,
            Self::Terminal => Self::Git,
            Self::Snippet => Self::Terminal,
            Self::Snippets => Self::Snippet,
            Self::Api => Self::Snippets,
            Self::Database => Self::Api,
            Self::Tasks => Self::Database,
            Self::Notes => Self::Tasks,
            Self::Logs => Self::Notes,
            Self::System => Self::Logs,
            Self::Skills => Self::System,
            Self::Workflows => Self::Skills,
            Self::Settings => Self::Workflows,
            Self::Cli => Self::Settings,
            Self::Collaboration => Self::Cli,
            Self::Macros => Self::Collaboration,
        }
    }

    pub const fn label(&self) -> &'static str {
        match self {
            Self::Agent => "Agent",
            Self::Packages => "Packages",
            Self::Files => "Files",
            Self::Git => "Git",
            Self::Terminal => "Terminal",
            Self::Snippet => "Snippet",
            Self::Snippets => "Snippets",
            Self::Api => "API",
            Self::Database => "Database",
            Self::Tasks => "Tasks",
            Self::Notes => "Notes",
            Self::Logs => "Logs",
            Self::System => "System",
            Self::Skills => "Skills",
            Self::Workflows => "Workflows",
            Self::Settings => "Settings",
            Self::Cli => "CLI",
            Self::Collaboration => "Collab",
            Self::Macros => "Macros",
        }
    }

    /// Get tab count
    pub const fn count() -> usize {
        19
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Column {
    #[default]
    Left,
    Center,
    Right,
}

impl Column {
    pub const fn all() -> [Self; 3] {
        [Self::Left, Self::Center, Self::Right]
    }

    pub const fn next(self) -> Self {
        match self {
            Self::Left => Self::Center,
            Self::Center => Self::Right,
            Self::Right => Self::Left,
        }
    }

    pub const fn prev(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Center => Self::Left,
            Self::Right => Self::Center,
        }
    }

    /// Get column count
    pub const fn count() -> usize {
        3
    }

    /// Get column label
    pub const fn label(&self) -> &'static str {
        match self {
            Self::Left => "Left",
            Self::Center => "Center",
            Self::Right => "Right",
        }
    }
}

/// UI state including current tab and column selection
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UIState {
    pub current_tab: Tab,
    pub current_column: Column,
    pub is_focused: bool,
}

impl UIState {
    pub const fn new() -> Self {
        Self {
            current_tab: Tab::Agent,
            current_column: Column::Center,
            is_focused: true,
        }
    }

    pub fn switch_tab(&mut self, tab: Tab) {
        self.current_tab = tab;
        self.current_column = Column::Center;
    }

    pub fn next_tab(&mut self) {
        self.current_tab = self.current_tab.next();
        self.current_column = Column::Center;
    }

    pub fn prev_tab(&mut self) {
        self.current_tab = self.current_tab.prev();
        self.current_column = Column::Center;
    }

    pub fn next_column(&mut self) {
        self.current_column = self.current_column.next();
    }

    pub fn prev_column(&mut self) {
        self.current_column = self.current_column.prev();
    }

    pub fn toggle_focus(&mut self) {
        self.is_focused = !self.is_focused;
    }
}
