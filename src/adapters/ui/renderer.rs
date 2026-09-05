#![allow(dead_code)]
use crate::adapters::ui::ratatui_adapter::{render_app_state, RATerminal};
use crate::modules::ui::domain::models::AppState;
use crate::shared::kernel::result::AppResult;

pub(crate) fn render(terminal: &mut RATerminal, state: &AppState) -> AppResult<()> {
    render_app_state(terminal, state)
}
