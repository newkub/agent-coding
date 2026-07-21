use super::TabRenderResult;
use crate::modules::ui::domain::models::AppState;
use ratatui::layout::Rect;
use ratatui::widgets::Paragraph;

/// Render Packages tab content
pub(crate) fn render_packages_tab(state: &AppState) -> TabRenderResult<'_> {
    let tab_state = &state.packages_tab_state;

    let content = Paragraph::new(format!(
        "Packages\n\nSelected Package: {}",
        tab_state.selected_package_index
    ));
    TabRenderResult::new(content, Rect::new(0, 0, 0, 0))
}

/// Render Snippets tab content
pub(crate) fn render_snippets_tab(state: &AppState) -> TabRenderResult<'_> {
    let tab_state = &state.snippet_tab_state;

    let content = Paragraph::new(format!(
        "Snippets\n\nSelected Snippet: {}",
        tab_state.selected_snippet_index
    ));
    TabRenderResult::new(content, Rect::new(0, 0, 0, 0))
}
