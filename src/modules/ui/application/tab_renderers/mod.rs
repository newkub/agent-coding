use crate::modules::ui::domain::models::AppState;
use crate::shared::types::Tab;

pub(crate) mod core_tabs;
pub(crate) mod data_tabs;
pub(crate) mod development_tabs;
pub(crate) mod system_tabs;

/// Per-column content for the 3-column layout
#[derive(Debug, Clone, Default)]
pub(crate) struct TabColumns {
    pub left: String,
    pub center: String,
    pub right: String,
}

impl TabColumns {
    pub(crate) fn new(
        left: impl Into<String>,
        center: impl Into<String>,
        right: impl Into<String>,
    ) -> Self {
        Self {
            left: left.into(),
            center: center.into(),
            right: right.into(),
        }
    }
}

/// Build per-column content for the current tab from live app state
pub(crate) fn render_tab_columns(state: &AppState) -> TabColumns {
    match state.ui_state.current_tab {
        Tab::Agent => core_tabs::render_agent_tab(state),
        Tab::Git => core_tabs::render_git_tab(state),
        Tab::Files => core_tabs::render_files_tab(state),
        Tab::Terminal => core_tabs::render_terminal_tab(state),
        Tab::Cli => core_tabs::render_cli_tab(state),
        Tab::Api => data_tabs::render_api_tab(state),
        Tab::Database => data_tabs::render_database_tab(state),
        Tab::Tasks => data_tabs::render_tasks_tab(state),
        Tab::Notes => data_tabs::render_notes_tab(state),
        Tab::Packages => development_tabs::render_packages_tab(state),
        Tab::Snippet | Tab::Snippets => development_tabs::render_snippets_tab(state),
        Tab::Skills => development_tabs::render_skills_tab(state),
        Tab::Workflows => development_tabs::render_workflows_tab(state),
        Tab::Settings => development_tabs::render_settings_tab(state),
        Tab::Logs => system_tabs::render_logs_tab(state),
        Tab::System => system_tabs::render_system_tab(state),
        Tab::Collaboration => development_tabs::render_collaboration_tab(state),
        Tab::Macros => development_tabs::render_macros_tab(state),
    }
}
