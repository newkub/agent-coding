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
            let max = state.logs_tab_state.entries.len().saturating_sub(1);
            if state.logs_tab_state.selected_log_index < max {
                state.logs_tab_state.selected_log_index += 1;
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

pub(crate) fn navigate_system_tab(state: &mut AppState, direction: NavigationDirection) {
    match direction {
        NavigationDirection::Up => {
            if state.system_tab_state.selected_metric_index > 0 {
                state.system_tab_state.selected_metric_index -= 1;
            }
        }
        NavigationDirection::Down => {
            let max = state.system_tab_state.metrics.len().saturating_sub(1);
            if state.system_tab_state.selected_metric_index < max {
                state.system_tab_state.selected_metric_index += 1;
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
