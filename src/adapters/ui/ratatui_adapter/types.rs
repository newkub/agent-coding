//! Ratatui adapter - Types

use crate::shared::kernel::result::AppResult;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io;

pub(crate) type RATerminal = Terminal<CrosstermBackend<std::io::Stdout>>;

pub(crate) struct RatatuiAdapter {
    pub terminal: Option<RATerminal>,
}

impl RatatuiAdapter {
    pub(crate) const fn new() -> Self {
        Self { terminal: None }
    }

    pub(crate) fn initialize(&mut self) -> AppResult<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        self.terminal = Some(Terminal::new(backend)?);
        Ok(())
    }

    pub(crate) fn cleanup(&self) -> AppResult<()> {
        disable_raw_mode()?;
        execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
        Ok(())
    }
}

impl Default for RatatuiAdapter {
    fn default() -> Self {
        Self::new()
    }
}
