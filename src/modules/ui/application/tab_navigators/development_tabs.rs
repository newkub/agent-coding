use crate::modules::ui::application::tab_navigation::NavigationDirection;
use crate::modules::ui::domain::models::AppState;
use crate::shared::types::Column;

pub(crate) fn navigate_packages_tab(state: &mut AppState, direction: NavigationDirection) {
    match direction {
        NavigationDirection::Up => {
            if state.packages_tab_state.selected_package_index > 0 {
                state.packages_tab_state.selected_package_index -= 1;
            }
        }
        NavigationDirection::Down => {
            let max = state
                .packages_tab_state
                .packages
                .iter()
                .filter(|p| !state.packages_tab_state.show_outdated || p.outdated)
                .count()
                .saturating_sub(1);
            if state.packages_tab_state.selected_package_index < max {
                state.packages_tab_state.selected_package_index += 1;
            }
        }
        NavigationDirection::Left => {
            state.ui_state.current_column = Column::Left;
        }
        NavigationDirection::Right => {
            state.ui_state.current_column = Column::Right;
        }
    }
}

pub(crate) fn navigate_snippets_tab(state: &mut AppState, direction: NavigationDirection) {
    match direction {
        NavigationDirection::Up => {
            if state.snippet_tab_state.selected_snippet_index > 0 {
                state.snippet_tab_state.selected_snippet_index -= 1;
            }
        }
        NavigationDirection::Down => {
            let max = state.snippet_tab_state.snippets.len().saturating_sub(1);
            if state.snippet_tab_state.selected_snippet_index < max {
                state.snippet_tab_state.selected_snippet_index += 1;
            }
        }
        NavigationDirection::Left => {
            state.ui_state.current_column = Column::Left;
        }
        NavigationDirection::Right => {
            state.ui_state.current_column = Column::Right;
        }
    }
}

pub(crate) fn navigate_skills_tab(state: &mut AppState, direction: NavigationDirection) {
    match direction {
        NavigationDirection::Up => {
            if state.skills_tab_state.selected_skill_index > 0 {
                state.skills_tab_state.selected_skill_index -= 1;
            }
        }
        NavigationDirection::Down => {
            let max = state.skills_tab_state.skills.len().saturating_sub(1);
            if state.skills_tab_state.selected_skill_index < max {
                state.skills_tab_state.selected_skill_index += 1;
            }
        }
        NavigationDirection::Left => {
            state.ui_state.current_column = Column::Left;
        }
        NavigationDirection::Right => {
            state.ui_state.current_column = Column::Right;
        }
    }
}

pub(crate) fn navigate_workflows_tab(state: &mut AppState, direction: NavigationDirection) {
    match direction {
        NavigationDirection::Up => {
            if state.workflows_tab_state.selected_workflow_index > 0 {
                state.workflows_tab_state.selected_workflow_index -= 1;
            }
        }
        NavigationDirection::Down => {
            let max = state.workflows_tab_state.workflows.len().saturating_sub(1);
            if state.workflows_tab_state.selected_workflow_index < max {
                state.workflows_tab_state.selected_workflow_index += 1;
            }
        }
        NavigationDirection::Left => {
            state.ui_state.current_column = Column::Left;
        }
        NavigationDirection::Right => {
            state.ui_state.current_column = Column::Right;
        }
    }
}
