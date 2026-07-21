use super::super::domain::{
    events::UIEvent,
    models::AppState,
    operations::{calculate_next_column, calculate_next_tab, calculate_prev_tab},
};
use crate::shared::types::Tab;

/// Use case: Switch to next tab
pub(crate) fn switch_next_tab(state: &mut AppState) -> UIEvent {
    let from = state.ui_state.current_tab;
    let to = calculate_next_tab(from);
    state.ui_state.switch_tab(to);
    UIEvent::tab_changed(from, to)
}

/// Use case: Switch to previous tab
pub(crate) fn switch_prev_tab(state: &mut AppState) -> UIEvent {
    let from = state.ui_state.current_tab;
    let to = calculate_prev_tab(from);
    state.ui_state.switch_tab(to);
    UIEvent::tab_changed(from, to)
}

/// Use case: Switch to specific tab
pub(crate) fn switch_tab(state: &mut AppState, tab: Tab) -> UIEvent {
    let from = state.ui_state.current_tab;
    state.ui_state.switch_tab(tab);
    UIEvent::tab_changed(from, tab)
}

/// Use case: Switch to next column
pub(crate) fn switch_next_column(state: &mut AppState) -> UIEvent {
    let from = state.ui_state.current_column;
    let to = calculate_next_column(from);
    state.ui_state.next_column();
    UIEvent::column_changed(from, to)
}

/// Use case: Toggle focus
pub(crate) fn toggle_focus(state: &mut AppState) -> UIEvent {
    state.ui_state.toggle_focus();
    UIEvent::focus_toggled(state.ui_state.is_focused)
}
