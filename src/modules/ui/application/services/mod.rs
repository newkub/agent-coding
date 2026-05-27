use super::super::domain::models::AppState;

/// Service: Initialize app state
pub(crate) fn initialize_app_state() -> AppState {
    AppState::new()
}

/// Service: Get current tab content
pub(crate) fn get_current_tab_content(state: &AppState) -> String {
    let content = state.current_tab_content();
    format!(
        "Left: {}\nCenter: {}\nRight: {}",
        content.left, content.center, content.right
    )
}
