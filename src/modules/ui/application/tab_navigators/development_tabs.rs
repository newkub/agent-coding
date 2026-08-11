use crate::modules::ui::application::tab_navigation::NavigationDirection;
use crate::modules::ui::domain::models::AppState;
use crate::shared::types::Column;

pub(crate) fn navigate_files_tab(state: &mut AppState, direction: NavigationDirection) {
    match direction {
        NavigationDirection::Up => {
            if state.files_tab_state.selected_file_index > 0 {
                state.files_tab_state.selected_file_index -= 1;
            }
        }
        NavigationDirection::Down => {
            state.files_tab_state.selected_file_index += 1;
        }
        NavigationDirection::Left => {
            state.ui_state.current_column = Column::Left;
        }
        NavigationDirection::Right => {
            state.ui_state.current_column = Column::Right;
        }
    }
}

pub(crate) fn navigate_packages_tab(state: &mut AppState, direction: NavigationDirection) {
    match direction {
        NavigationDirection::Up => {
            if state.packages_tab_state.selected_package_index > 0 {
                state.packages_tab_state.selected_package_index -= 1;
            }
        }
        NavigationDirection::Down => {
            state.packages_tab_state.selected_package_index += 1;
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
            state.snippet_tab_state.selected_snippet_index += 1;
        }
        NavigationDirection::Left => {
            state.ui_state.current_column = Column::Left;
        }
        NavigationDirection::Right => {
            state.ui_state.current_column = Column::Right;
        }
    }
}
