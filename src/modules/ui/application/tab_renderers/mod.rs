use crate::modules::ui::domain::models::AppState;
use ratatui::layout::Rect;
use ratatui::widgets::Paragraph;

pub mod core_tabs;
pub mod data_tabs;
pub mod development_tabs;
pub mod system_tabs;

/// Result of rendering a tab
#[derive(Debug, Clone)]
pub struct TabRenderResult<'a> {
    pub content: Paragraph<'a>,
    pub area: Rect,
}

impl<'a> TabRenderResult<'a> {
    pub(crate) const fn new(content: Paragraph<'a>, area: Rect) -> Self {
        Self { content, area }
    }
}

/// Render current tab based on app state
pub fn render_current_tab(state: &AppState, area: Rect) -> TabRenderResult<'_> {
    use crate::shared::types::Tab;

    match state.ui_state.current_tab {
        Tab::Agent => core_tabs::render_agent_tab(state),
        Tab::Git => core_tabs::render_git_tab(state),
        Tab::Files => core_tabs::render_files_tab(state),
        Tab::Terminal => core_tabs::render_terminal_tab(state),
        Tab::Api => data_tabs::render_api_tab(state),
        Tab::Database => data_tabs::render_database_tab(state),
        Tab::Tasks => data_tabs::render_tasks_tab(state),
        Tab::Notes => data_tabs::render_notes_tab(state),
        Tab::Packages => development_tabs::render_packages_tab(state),
        Tab::Snippets => development_tabs::render_snippets_tab(state),
        Tab::Logs => system_tabs::render_logs_tab(state),
        Tab::System => system_tabs::render_system_tab(state),
        _ => TabRenderResult::new(Paragraph::new("Tab not implemented"), area),
    }
}
