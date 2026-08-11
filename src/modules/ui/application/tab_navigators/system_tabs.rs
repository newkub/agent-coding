use crate::modules::ui::application::tab_navigation::NavigationDirection;
use crate::modules::ui::domain::models::AppState;
use crate::shared::types::Column;

pub(crate) fn navigate_logs_tab(state: &mut AppState, direction: NavigationDirection) {
    match direction {
        NavigationDirection::Up => {
            if state.logs_tab_state.selected_log_index > 0 {
                state.logs_tab_state.selected_log_index -= 1;
            }
        }
        NavigationDirection::Down => {
            state.logs_tab_state.selected_log_index += 1;
        }
        NavigationDirection::Left => {
            state.ui_state.current_column = Column::Left;
        }
        NavigationDirection::Right => {
            state.ui_state.current_column = Column::Right;
        }
    }
}

pub(crate) fn navigate_system_tab(state: &mut AppState, direction: NavigationDirection) {
    match direction {
        NavigationDirection::Up | NavigationDirection::Down => {
            // Navigate through processes
        }
        NavigationDirection::Left => {
            state.ui_state.current_column = Column::Left;
        }
        NavigationDirection::Right => {
            state.ui_state.current_column = Column::Right;
        }
    }
}
