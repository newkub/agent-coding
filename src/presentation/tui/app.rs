use crate::adapters::input::crossterm_handler::CrosstermInputHandler;
use crate::adapters::ui::ratatui_adapter::RatatuiAdapter;
use crate::modules::ui::application::{
    services::initialize_app_state,
    usecases::{switch_next_column, switch_next_tab, switch_prev_tab, toggle_focus},
};
use crate::modules::ui::domain::models::AppState;
use crate::modules::ui::ports::{InputHandler, UIRenderer};
use crate::presentation::tui::components::styles::set_theme;
use crate::presentation::tui::components::theme::Theme;
use crate::shared::kernel::result::AppResult;
use crossterm::event::KeyCode;

pub(crate) struct TUIApp {
    state: AppState,
    renderer: RatatuiAdapter,
    input_handler: CrosstermInputHandler,
}

impl TUIApp {
    pub(crate) fn new() -> AppResult<Self> {
        // Initialize theme with modern dark theme (default)
        let theme = Theme::modern_dark();
        set_theme(theme);

        let mut renderer = RatatuiAdapter::new();
        renderer.initialize()?;

        Ok(Self {
            state: initialize_app_state(),
            renderer,
            input_handler: CrosstermInputHandler::new(),
        })
    }

    pub(crate) async fn run(&mut self) -> AppResult<()> {
        loop {
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

        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                return Ok(true); // Exit
            }
            KeyCode::Char('?') => {
                self.state.show_help = true;
            }
            KeyCode::Char('h')
                if key
                    .modifiers
                    .contains(crossterm::event::KeyModifiers::CONTROL) =>
            {
                self.state.show_help = true;
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
                // Could implement prev_column
            }
            KeyCode::Char('f') => {
                toggle_focus(&mut self.state);
            }
            _ => {}
        }
        Ok(false)
    }
}

impl Drop for TUIApp {
    fn drop(&mut self) {
        let _ = self.renderer.cleanup();
    }
}
