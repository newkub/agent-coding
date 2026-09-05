use crate::modules::ui::application::tab_navigation::NavigationDirection;
use crate::modules::ui::application::tab_navigators::{
    navigate_agent_tab, navigate_api_tab, navigate_cli_tab, navigate_collaboration_tab,
    navigate_database_tab, navigate_files_tab, navigate_git_tab, navigate_logs_tab,
    navigate_macros_tab, navigate_notes_tab, navigate_packages_tab, navigate_skills_tab,
    navigate_snippets_tab, navigate_system_tab, navigate_tasks_tab, navigate_terminal_tab,
    navigate_workflows_tab,
};
use crate::modules::ui::domain::models::AppState;
use crate::shared::types::{Column, Tab};

/// Navigate to specific tab
pub(crate) fn navigate_to_tab(state: &mut AppState, tab: Tab) {
    state.ui_state.switch_tab(tab);
}

/// Navigate to next tab (wraps around)
pub(crate) fn navigate_next_tab(state: &mut AppState) {
    state.ui_state.next_tab();
}

/// Navigate to previous tab (wraps around)
pub(crate) fn navigate_prev_tab(state: &mut AppState) {
    state.ui_state.prev_tab();
}

/// Navigate to next column (wraps around)
pub(crate) fn navigate_next_column(state: &mut AppState) {
    state.ui_state.next_column();
}

/// Navigate to previous column (wraps around)
pub(crate) fn navigate_prev_column(state: &mut AppState) {
    state.ui_state.prev_column();
}

/// Navigate to first column (Left)
pub(crate) fn navigate_to_left(state: &mut AppState) {
    state.ui_state.current_column = Column::Left;
}

/// Navigate to center column
pub(crate) fn navigate_to_center(state: &mut AppState) {
    state.ui_state.current_column = Column::Center;
}

/// Navigate to right column
pub(crate) fn navigate_to_right(state: &mut AppState) {
    state.ui_state.current_column = Column::Right;
}

/// Toggle focus mode
pub(crate) fn toggle_focus_mode(state: &mut AppState) {
    state.ui_state.toggle_focus();
}

/// Navigate within a tab (specific to tab type)
pub(crate) fn navigate_tab_item(state: &mut AppState, direction: NavigationDirection) {
    match state.ui_state.current_tab {
        Tab::Agent => navigate_agent_tab(state, direction),
        Tab::Packages => navigate_packages_tab(state, direction),
        Tab::Files => navigate_files_tab(state, direction),
        Tab::Git => navigate_git_tab(state, direction),
        Tab::Terminal => navigate_terminal_tab(state, direction),
        Tab::Snippets => navigate_snippets_tab(state, direction),
        Tab::Api => navigate_api_tab(state, direction),
        Tab::Database => navigate_database_tab(state, direction),
        Tab::Tasks => navigate_tasks_tab(state, direction),
        Tab::Notes => navigate_notes_tab(state, direction),
        Tab::Logs => navigate_logs_tab(state, direction),
        Tab::System => navigate_system_tab(state, direction),
        Tab::Snippet => navigate_snippets_tab(state, direction),
        Tab::Skills => navigate_skills_tab(state, direction),
        Tab::Workflows => navigate_workflows_tab(state, direction),
        Tab::Settings => {}
        Tab::Cli => navigate_cli_tab(state, direction),
        Tab::Collaboration => navigate_collaboration_tab(state, direction),
        Tab::Macros => navigate_macros_tab(state, direction),
    }
}
