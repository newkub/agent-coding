use crate::modules::ui::application::tab_navigation::NavigationDirection;
use crate::modules::ui::domain::models::AppState;
use crate::shared::types::Column;

pub(crate) fn navigate_agent_tab(state: &mut AppState, direction: NavigationDirection) {
    match direction {
        NavigationDirection::Up => {
            if state.agent_tab_state.selected_session_index > 0 {
                state.agent_tab_state.selected_session_index -= 1;
            }
        }
        NavigationDirection::Down => {
            let max = state.agent_tab_state.sessions.len().saturating_sub(1);
            if state.agent_tab_state.selected_session_index < max {
                state.agent_tab_state.selected_session_index += 1;
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

pub(crate) fn navigate_files_tab(state: &mut AppState, direction: NavigationDirection) {
    match direction {
        NavigationDirection::Up => {
            if state.files_tab_state.selected_file_index > 0 {
                state.files_tab_state.selected_file_index -= 1;
            }
        }
        NavigationDirection::Down => {
            let max = state.files_tab_state.files.len().saturating_sub(1);
            if state.files_tab_state.selected_file_index < max {
                state.files_tab_state.selected_file_index += 1;
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

pub(crate) fn navigate_git_tab(state: &mut AppState, direction: NavigationDirection) {
    match direction {
        NavigationDirection::Up => {
            if state.git_tab_state.selected_file_index > 0 {
                state.git_tab_state.selected_file_index -= 1;
            }
        }
        NavigationDirection::Down => {
            let max = (state.git_tab_state.staged_files.len()
                + state.git_tab_state.unstaged_files.len())
            .saturating_sub(1);
            if state.git_tab_state.selected_file_index < max {
                state.git_tab_state.selected_file_index += 1;
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

pub(crate) fn navigate_terminal_tab(state: &mut AppState, direction: NavigationDirection) {
    match direction {
        NavigationDirection::Up => {
            if state.terminal_tab_state.selected_history_index.unwrap_or(0) > 0 {
                state.terminal_tab_state.selected_history_index =
                    Some(state.terminal_tab_state.selected_history_index.unwrap_or(0) - 1);
            }
        }
        NavigationDirection::Down => {
            let max = state.terminal_tab_state.history.len().saturating_sub(1);
            let next = state.terminal_tab_state.selected_history_index.unwrap_or(0) + 1;
            state.terminal_tab_state.selected_history_index = Some(next.min(max));
        }
        NavigationDirection::Left => {
            state.ui_state.current_column = Column::Left;
        }
        NavigationDirection::Right => {
            state.ui_state.current_column = Column::Right;
        }
    }
}

pub(crate) fn navigate_cli_tab(state: &mut AppState, direction: NavigationDirection) {
    match direction {
        NavigationDirection::Up => {
            if state.cli_tab_state.selected_history_index.unwrap_or(0) > 0 {
                state.cli_tab_state.selected_history_index =
                    Some(state.cli_tab_state.selected_history_index.unwrap_or(0) - 1);
            }
        }
        NavigationDirection::Down => {
            let max = state.cli_tab_state.history.len().saturating_sub(1);
            let next = state.cli_tab_state.selected_history_index.unwrap_or(0) + 1;
            state.cli_tab_state.selected_history_index = Some(next.min(max));
        }
        NavigationDirection::Left => {
            state.ui_state.current_column = Column::Left;
        }
        NavigationDirection::Right => {
            state.ui_state.current_column = Column::Right;
        }
    }
}
