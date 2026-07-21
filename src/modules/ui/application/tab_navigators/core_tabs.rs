use crate::modules::ui::application::tab_navigation::NavigationDirection;
use crate::modules::ui::domain::models::AppState;
use crate::shared::types::Column;

pub fn navigate_agent_tab(state: &mut AppState, direction: NavigationDirection) {
    match direction {
        NavigationDirection::Up | NavigationDirection::Down => {
            // Navigate through messages
        }
        NavigationDirection::Left => {
            state.ui_state.current_column = Column::Left;
        }
        NavigationDirection::Right => {
            state.ui_state.current_column = Column::Right;
        }
    }
}

pub fn navigate_files_tab(state: &mut AppState, direction: NavigationDirection) {
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

pub fn navigate_git_tab(state: &mut AppState, direction: NavigationDirection) {
    match direction {
        NavigationDirection::Up => {
            if state.git_tab_state.selected_file_index > 0 {
                state.git_tab_state.selected_file_index -= 1;
            }
        }
        NavigationDirection::Down => {
            state.git_tab_state.selected_file_index += 1;
        }
        NavigationDirection::Left => {
            state.ui_state.current_column = Column::Left;
        }
        NavigationDirection::Right => {
            state.ui_state.current_column = Column::Right;
        }
    }
}

pub fn navigate_terminal_tab(state: &mut AppState, direction: NavigationDirection) {
    match direction {
        NavigationDirection::Up => {
            if state.terminal_tab_state.selected_history_index.unwrap_or(0) > 0 {
                state.terminal_tab_state.selected_history_index =
                    Some(state.terminal_tab_state.selected_history_index.unwrap() - 1);
            }
        }
        NavigationDirection::Down => {
            state.terminal_tab_state.selected_history_index =
                Some(state.terminal_tab_state.selected_history_index.unwrap_or(0) + 1);
        }
        NavigationDirection::Left => {
            state.ui_state.current_column = Column::Left;
        }
        NavigationDirection::Right => {
            state.ui_state.current_column = Column::Right;
        }
    }
}
