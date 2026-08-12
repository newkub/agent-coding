use crate::modules::ui::application::tab_navigation::NavigationDirection;
use crate::modules::ui::domain::models::AppState;
use crate::shared::types::Column;

pub(crate) fn navigate_api_tab(state: &mut AppState, direction: NavigationDirection) {
    match direction {
        NavigationDirection::Up => {
            if state.api_tab_state.selected_endpoint_index > 0 {
                state.api_tab_state.selected_endpoint_index -= 1;
            }
        }
        NavigationDirection::Down => {
            state.api_tab_state.selected_endpoint_index += 1;
        }
        NavigationDirection::Left => {
            state.ui_state.current_column = Column::Left;
        }
        NavigationDirection::Right => {
            state.ui_state.current_column = Column::Right;
        }
    }
}

pub(crate) fn navigate_database_tab(state: &mut AppState, direction: NavigationDirection) {
    match direction {
        NavigationDirection::Up => {
            if state.database_tab_state.selected_table_index > 0 {
                state.database_tab_state.selected_table_index -= 1;
            }
        }
        NavigationDirection::Down => {
            state.database_tab_state.selected_table_index += 1;
        }
        NavigationDirection::Left => {
            state.ui_state.current_column = Column::Left;
        }
        NavigationDirection::Right => {
            state.ui_state.current_column = Column::Right;
        }
    }
}

pub(crate) fn navigate_tasks_tab(state: &mut AppState, direction: NavigationDirection) {
    match direction {
        NavigationDirection::Up => {
            if state.tasks_tab_state.selected_task_index > 0 {
                state.tasks_tab_state.selected_task_index -= 1;
            }
        }
        NavigationDirection::Down => {
            state.tasks_tab_state.selected_task_index += 1;
        }
        NavigationDirection::Left => {
            state.ui_state.current_column = Column::Left;
        }
        NavigationDirection::Right => {
            state.ui_state.current_column = Column::Right;
        }
    }
    let idx = state.tasks_tab_state.selected_task_index;
    state.tasks_tab_state.task_manager.navigate(idx);
}

pub(crate) fn navigate_notes_tab(state: &mut AppState, direction: NavigationDirection) {
    match direction {
        NavigationDirection::Up => {
            if state.notes_tab_state.selected_note_index > 0 {
                state.notes_tab_state.selected_note_index -= 1;
            }
        }
        NavigationDirection::Down => {
            state.notes_tab_state.selected_note_index += 1;
        }
        NavigationDirection::Left => {
            state.ui_state.current_column = Column::Left;
        }
        NavigationDirection::Right => {
            state.ui_state.current_column = Column::Right;
        }
    }
}
