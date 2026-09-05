use crate::adapters::config::loader::ConfigLoader;
use crate::adapters::input::crossterm_handler::CrosstermInputHandler;
use crate::adapters::ui::ratatui_adapter::RatatuiAdapter;
use crate::modules::ui::application::handlers::TabAction;
use crate::modules::ui::application::navigation::navigate_tab_item;
use crate::modules::ui::application::tab_actions::handle_tab_action;
use crate::modules::ui::application::tab_navigation::NavigationDirection;
use crate::modules::ui::application::{
    services::initialize_app_state,
    usecases::{
        switch_next_column, switch_next_tab, switch_prev_column, switch_prev_tab, toggle_focus,
    },
};
use crate::modules::ui::domain::models::app_commands::get_tab_specific_commands;
use crate::modules::ui::domain::models::{AppState, Command, ToastKind};
use crate::modules::ui::ports::{InputHandler, UIRenderer};
use crate::presentation::tui::components::styles::set_theme;
use crate::presentation::tui::components::theme::Theme;
use crate::presentation::tui::di::DIContainer;
use crate::shared::kernel::result::AppResult;
use crate::shared::types::Tab;
use crossterm::event::{KeyCode, KeyModifiers};

pub(crate) struct TUIApp {
    state: AppState,
    renderer: RatatuiAdapter,
    input_handler: CrosstermInputHandler,
    di: DIContainer,
}

impl TUIApp {
    /// Create a new TUI application.
    ///
    /// Builds the DI container, connects the backing SQLite database, and
    /// preloads every tab with real data before opening the UI.
    pub(crate) async fn new() -> AppResult<Self> {
        // Load user settings and initialize theme from config (fallback to modern dark)
        let loader = ConfigLoader::new();
        let settings = loader.load().unwrap_or_default();
        let _available = Theme::available_themes();
        let theme = Theme::from_name(&settings.ui.theme);
        set_theme(theme);

        // Build the DI container and connect the database
        let mut di = DIContainer::new().build().await?;
        di.init_db().await?;

        // Initialize state and preload real data into every tab
        let mut state = initialize_app_state();
        state.settings_tab_state.theme = settings.ui.theme.clone();
        state.settings_tab_state.font_size = u16::from(settings.ui.font_size);
        state.load_from_di(&di).await?;

        let mut renderer = RatatuiAdapter::new();
        renderer.initialize()?;

        Ok(Self {
            state,
            renderer,
            input_handler: CrosstermInputHandler::new(),
            di,
        })
    }

    pub(crate) async fn run(&mut self) -> AppResult<()> {
        loop {
            // Drop expired toast notifications
            self.state.prune_expired_toasts();

            // Render current state
            self.renderer.render(&self.state).await?;

            // Read input
            if let Some(key) = self.input_handler.read_key().await? {
                if self.handle_key(key).await? {
                    break; // Exit requested
                }
            }
        }

        self.renderer.cleanup()?;
        Ok(())
    }

    async fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> AppResult<bool> {
        // Handle help modal close first
        if self.state.show_help {
            match key.code {
                KeyCode::Char('?') | KeyCode::Esc => {
                    self.state.show_help = false;
                    return Ok(false);
                }
                _ => return Ok(false),
            }
        }

        // Command palette intercepts all input while open
        if self.state.show_command_palette {
            self.handle_palette_key(key).await?;
            return Ok(false);
        }

        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                return Ok(true); // Exit
            }
            KeyCode::Char('?') => {
                self.state.show_help = true;
            }
            KeyCode::Char('h') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.state.show_help = true;
            }
            KeyCode::Char('k') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.state.show_command_palette = true;
                self.state.command_input.clear();
                self.state.command_palette_selected = 0;
            }
            KeyCode::Char('n') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.handle_tab_shortcut(Tab::Agent, TabAction::StartSession)
                    .await?;
            }
            KeyCode::Char('l') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.handle_tab_shortcut(Tab::Terminal, TabAction::Clear)
                    .await?;
            }
            KeyCode::Tab => {
                switch_next_tab(&mut self.state);
            }
            KeyCode::BackTab => {
                switch_prev_tab(&mut self.state);
            }
            KeyCode::Right => {
                switch_next_column(&mut self.state);
            }
            KeyCode::Left => {
                switch_prev_column(&mut self.state);
            }
            KeyCode::Up => {
                navigate_tab_item(&mut self.state, NavigationDirection::Up);
            }
            KeyCode::Down => {
                navigate_tab_item(&mut self.state, NavigationDirection::Down);
            }
            KeyCode::Enter => {
                self.handle_enter_key().await?;
            }
            KeyCode::Backspace => match self.state.ui_state.current_tab {
                Tab::Terminal => {
                    self.state.terminal_tab_state.terminal_input.pop();
                }
                Tab::Database => {
                    self.state.database_tab_state.query_input.pop();
                }
                Tab::Cli => {
                    self.state.cli_tab_state.command_input.pop();
                }
                _ => {}
            },
            KeyCode::Char('f') if key.modifiers.is_empty() => {
                toggle_focus(&mut self.state);
            }
            KeyCode::Char(c) if key.modifiers.is_empty() => {
                self.handle_tab_char(c).await?;
            }
            _ => {}
        }
        Ok(false)
    }

    /// Enter dispatches the primary action of the current tab
    async fn handle_enter_key(&mut self) -> AppResult<()> {
        let action = match self.state.ui_state.current_tab {
            Tab::Agent => {
                if self.state.agent_tab_state.session_id.is_none() {
                    Some(TabAction::StartSession)
                } else {
                    // Load the selected session from the repository list
                    Some(TabAction::Select)
                }
            }
            Tab::Terminal => Some(TabAction::Execute),
            Tab::Cli => {
                let cmd = self.state.cli_tab_state.command_input.clone();
                Some(TabAction::RunCommand(cmd))
            }
            Tab::Files => Some(TabAction::OpenFile),
            Tab::Database => {
                if self.state.database_tab_state.query_input.trim().is_empty() {
                    // Preview the selected table
                    Some(TabAction::Select)
                } else {
                    Some(TabAction::Execute)
                }
            }
            _ => Some(TabAction::Select),
        };
        if let Some(action) = action {
            handle_tab_action(&mut self.state, action, &self.di).await?;
        }
        Ok(())
    }

    /// Plain character keys: input fields first, then per-tab shortcuts
    async fn handle_tab_char(&mut self, c: char) -> AppResult<()> {
        match self.state.ui_state.current_tab {
            Tab::Terminal => {
                self.state.terminal_tab_state.terminal_input.push(c);
            }
            Tab::Database => {
                self.state.database_tab_state.query_input.push(c);
            }
            Tab::Cli => {
                self.state.cli_tab_state.command_input.push(c);
            }
            Tab::Git => match c {
                's' => {
                    handle_tab_action(&mut self.state, TabAction::Stage, &self.di).await?;
                    self.state
                        .push_toast(ToastKind::Success, "Staged selected file");
                }
                'u' => {
                    handle_tab_action(&mut self.state, TabAction::Unstage, &self.di).await?;
                    self.state
                        .push_toast(ToastKind::Success, "Unstaged selected file");
                }
                'p' => {
                    handle_tab_action(&mut self.state, TabAction::Push, &self.di).await?;
                    self.state
                        .push_toast(ToastKind::Info, "Pushed current branch");
                }
                'r' => {
                    handle_tab_action(&mut self.state, TabAction::Refresh, &self.di).await?;
                }
                _ => {}
            },
            Tab::Files | Tab::Logs | Tab::System | Tab::Packages if c == 'r' => {
                handle_tab_action(&mut self.state, TabAction::Refresh, &self.di).await?;
            }
            _ => {}
        }
        Ok(())
    }

    /// Run a tab action only when the current tab matches
    async fn handle_tab_shortcut(&mut self, tab: Tab, action: TabAction) -> AppResult<()> {
        if self.state.ui_state.current_tab == tab {
            handle_tab_action(&mut self.state, action, &self.di).await?;
        }
        Ok(())
    }

    /// Keys while the command palette is open
    async fn handle_palette_key(&mut self, key: crossterm::event::KeyEvent) -> AppResult<()> {
        match key.code {
            KeyCode::Esc => self.close_command_palette(),
            KeyCode::Char('k') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.close_command_palette();
            }
            KeyCode::Enter => {
                self.execute_palette_command().await?;
                self.close_command_palette();
            }
            KeyCode::Up => {
                self.state.command_palette_selected =
                    self.state.command_palette_selected.saturating_sub(1);
            }
            KeyCode::Down => {
                let max = self.filtered_palette_commands().len().saturating_sub(1);
                if self.state.command_palette_selected < max {
                    self.state.command_palette_selected += 1;
                }
            }
            KeyCode::Backspace => {
                self.state.command_input.pop();
                self.state.command_palette_selected = 0;
            }
            KeyCode::Char(c) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.state.command_input.push(c);
                self.state.command_palette_selected = 0;
            }
            _ => {}
        }
        Ok(())
    }

    fn close_command_palette(&mut self) {
        self.state.show_command_palette = false;
        self.state.command_input.clear();
        self.state.command_palette_selected = 0;
    }

    /// Commands filtered by the current query, matching the palette widget
    fn filtered_palette_commands(&self) -> Vec<Command> {
        let query = &self.state.command_input;
        get_tab_specific_commands(self.state.ui_state.current_tab)
            .into_iter()
            .filter(|c| query.is_empty() || ratatui_ui::fuzzy_score(query, &c.name).is_some())
            .collect()
    }

    /// Execute the selected palette command against the current tab
    async fn execute_palette_command(&mut self) -> AppResult<()> {
        let Some(cmd) = self
            .filtered_palette_commands()
            .get(self.state.command_palette_selected)
            .cloned()
        else {
            return Ok(());
        };

        let executed = match (self.state.ui_state.current_tab, cmd.name.as_str()) {
            (Tab::Agent, "New Session") => {
                handle_tab_action(&mut self.state, TabAction::StartSession, &self.di).await?;
                true
            }
            (Tab::Terminal, "Clear") => {
                handle_tab_action(&mut self.state, TabAction::Clear, &self.di).await?;
                true
            }
            (Tab::Files, "New File") => {
                handle_tab_action(&mut self.state, TabAction::CreateFile, &self.di).await?;
                true
            }
            (Tab::Files, "Refresh") | (Tab::Packages, "Refresh") | (Tab::Git, "Refresh") => {
                handle_tab_action(&mut self.state, TabAction::Refresh, &self.di).await?;
                true
            }
            (Tab::Git, "Commit") => {
                handle_tab_action(
                    &mut self.state,
                    TabAction::Commit("Update from TUI".to_string()),
                    &self.di,
                )
                .await?;
                true
            }
            (Tab::Git, "Push") => {
                handle_tab_action(&mut self.state, TabAction::Push, &self.di).await?;
                true
            }
            (Tab::Database, "Run Query") => {
                handle_tab_action(&mut self.state, TabAction::Execute, &self.di).await?;
                true
            }
            (Tab::Logs, "Refresh") | (Tab::System, "Refresh") => {
                handle_tab_action(&mut self.state, TabAction::Refresh, &self.di).await?;
                true
            }
            (Tab::Snippets, "New Snippet") | (Tab::Snippet, "New Snippet") => {
                handle_tab_action(&mut self.state, TabAction::CreateSnippet, &self.di).await?;
                true
            }
            (Tab::Skills, "Run") => {
                handle_tab_action(&mut self.state, TabAction::RunSkill, &self.di).await?;
                true
            }
            (Tab::Workflows, "Run") => {
                handle_tab_action(&mut self.state, TabAction::RunWorkflow, &self.di).await?;
                true
            }
            (Tab::Notes, "New Note") => {
                handle_tab_action(
                    &mut self.state,
                    TabAction::Add("New note".to_string()),
                    &self.di,
                )
                .await?;
                true
            }
            _ => false,
        };

        if executed {
            self.state
                .push_toast(ToastKind::Success, format!("Ran: {}", cmd.name));
        } else {
            self.state.push_toast(
                ToastKind::Info,
                format!("'{}' is not available yet", cmd.name),
            );
        }
        Ok(())
    }
}

impl Drop for TUIApp {
    fn drop(&mut self) {
        let _ = self.renderer.cleanup();
    }
}
