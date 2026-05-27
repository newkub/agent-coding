use async_trait::async_trait;
use crossterm::event::{self, KeyEvent};
use crate::shared::constants::INPUT_POLL_TIMEOUT_MS;
use crate::shared::kernel::result::AppResult;
use crate::modules::ui::ports::InputHandler;

pub(crate) struct CrosstermInputHandler;

impl CrosstermInputHandler {
    pub(crate) const fn new() -> Self {
        Self
    }
}

impl Default for CrosstermInputHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl InputHandler for CrosstermInputHandler {
    async fn read_key(&self) -> AppResult<Option<KeyEvent>> {
        if event::poll(std::time::Duration::from_millis(INPUT_POLL_TIMEOUT_MS))? {
            let key = event::read()?;
            if let event::Event::Key(key) = key {
                return Ok(Some(key));
            }
        }
        Ok(None)
    }
}
