use crate::modules::ui::domain::models::AppState;
use super::TabRenderResult;
use ratatui::layout::Rect;
use ratatui::widgets::Paragraph;

/// Render Logs tab content
pub(crate) fn render_logs_tab(state: &AppState) -> TabRenderResult<'_> {
    let tab_state = &state.logs_tab_state;
    
    let content = Paragraph::new(format!(
        "Logs\n\nSelected Log: {}",
        tab_state.selected_log_index
    ));
    TabRenderResult::new(content, Rect::new(0, 0, 0, 0))
}

/// Render System tab content
pub(crate) fn render_system_tab(_state: &AppState) -> TabRenderResult<'_> {
    let content = Paragraph::new("System\n\nMonitoring system resources");
    TabRenderResult::new(content, Rect::new(0, 0, 0, 0))
}
