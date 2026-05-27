use crate::modules::ui::domain::models::AppState;
use super::TabRenderResult;
use ratatui::layout::Rect;
use ratatui::widgets::Paragraph;

/// Render API tab content
pub(crate) fn render_api_tab(state: &AppState) -> TabRenderResult<'_> {
    let tab_state = &state.api_tab_state;
    
    let content = Paragraph::new(format!(
        "API\n\nSelected Endpoint: {}",
        tab_state.selected_endpoint_index
    ));
    TabRenderResult::new(content, Rect::new(0, 0, 0, 0))
}

/// Render Database tab content
pub(crate) fn render_database_tab(state: &AppState) -> TabRenderResult<'_> {
    let tab_state = &state.database_tab_state;
    
    let content = Paragraph::new(format!(
        "Database\n\nSelected Table: {}",
        tab_state.selected_table_index
    ));
    TabRenderResult::new(content, Rect::new(0, 0, 0, 0))
}

/// Render Tasks tab content
pub(crate) fn render_tasks_tab(state: &AppState) -> TabRenderResult<'_> {
    let tab_state = &state.tasks_tab_state;
    
    let content = Paragraph::new(format!(
        "Tasks\n\nSelected Task: {}",
        tab_state.selected_task_index
    ));
    TabRenderResult::new(content, Rect::new(0, 0, 0, 0))
}

/// Render Notes tab content
pub(crate) fn render_notes_tab(state: &AppState) -> TabRenderResult<'_> {
    let tab_state = &state.notes_tab_state;
    
    let content = Paragraph::new(format!(
        "Notes\n\nSelected Note: {}",
        tab_state.selected_note_index
    ));
    TabRenderResult::new(content, Rect::new(0, 0, 0, 0))
}
