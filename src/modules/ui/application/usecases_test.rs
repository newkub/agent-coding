use crate::shared::types::{Column, Tab};
use crate::modules::ui::application::usecases::{
    switch_next_column, switch_next_tab, switch_prev_tab, toggle_focus,
};
use crate::modules::ui::domain::events::UIEvent;
use crate::modules::ui::domain::models::AppState;

#[test]
fn test_switch_next_tab() {
    let mut state = AppState::new();
    assert_eq!(state.ui_state.current_tab, Tab::Agent);
    
    let event = switch_next_tab(&mut state);
    assert_eq!(state.ui_state.current_tab, Tab::Git);
    assert_eq!(event, UIEvent::tab_changed(Tab::Agent, Tab::Git));
}

#[test]
fn test_switch_prev_tab() {
    let mut state = AppState::new();
    let event = switch_prev_tab(&mut state);
    assert_eq!(state.ui_state.current_tab, Tab::Cli);
    assert_eq!(event, UIEvent::tab_changed(Tab::Agent, Tab::Cli));
}

#[test]
fn test_switch_tab() {
    let mut state = AppState::new();
    let event = switch_tab(&mut state, Tab::Cli);
    assert_eq!(state.ui_state.current_tab, Tab::Cli);
    assert_eq!(event, UIEvent::tab_changed(Tab::Agent, Tab::Cli));
}

#[test]
fn test_switch_next_column() {
    let mut state = AppState::new();
    let event = switch_next_column(&mut state);
    assert_eq!(state.ui_state.current_column, Column::Right);
    assert_eq!(event, UIEvent::column_changed(Column::Center, Column::Right));
}

#[test]
fn test_toggle_focus() {
    let mut state = AppState::new();
    assert!(state.ui_state.is_focused);
    
    let event = toggle_focus(&mut state);
    assert!(!state.ui_state.is_focused);
    assert_eq!(event, UIEvent::focus_toggled(false));
}
