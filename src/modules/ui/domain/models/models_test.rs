use crate::shared::types::{Column, Tab};
use crate::modules::ui::domain::models::{AppState, TabContent, UIState};

#[test]
fn test_ui_state_default() {
    let state = UIState::default();
    assert_eq!(state.current_tab, Tab::Agent);
    assert_eq!(state.current_column, Column::Center);
    assert!(state.is_focused);
}

#[test]
fn test_ui_state_switch_tab() {
    let mut state = UIState::default();
    state.switch_tab(Tab::Git);
    assert_eq!(state.current_tab, Tab::Git);
    assert_eq!(state.current_column, Column::Center);
}

#[test]
fn test_ui_state_next_tab() {
    let mut state = UIState::default();
    state.next_tab();
    assert_eq!(state.current_tab, Tab::Git);
    state.next_tab();
    assert_eq!(state.current_tab, Tab::Cli);
    state.next_tab();
    assert_eq!(state.current_tab, Tab::Agent);
}

#[test]
fn test_ui_state_prev_tab() {
    let mut state = UIState::default();
    state.prev_tab();
    assert_eq!(state.current_tab, Tab::Cli);
    state.prev_tab();
    assert_eq!(state.current_tab, Tab::Git);
}

#[test]
fn test_ui_state_next_column() {
    let mut state = UIState::default();
    state.next_column();
    assert_eq!(state.current_column, Column::Right);
    state.next_column();
    assert_eq!(state.current_column, Column::Left);
}

#[test]
fn test_ui_state_toggle_focus() {
    let mut state = UIState::default();
    assert!(state.is_focused);
    state.toggle_focus();
    assert!(!state.is_focused);
    state.toggle_focus();
    assert!(state.is_focused);
}

#[test]
fn test_tab_content_default() {
    let content = TabContent::default();
    assert!(content.left.is_empty());
    assert!(content.center.is_empty());
    assert!(content.right.is_empty());
}

#[test]
fn test_tab_content_with_content() {
    let content = TabContent::with_content("Left", "Center", "Right");
    assert_eq!(content.left, "Left");
    assert_eq!(content.center, "Center");
    assert_eq!(content.right, "Right");
}

#[test]
fn test_app_state_default() {
    let state = AppState::new();
    assert_eq!(state.ui_state.current_tab, Tab::Agent);
    assert_eq!(state.agent_tab.left, "Agent Context");
    assert_eq!(state.git_tab.left, "Git Status");
    assert_eq!(state.cli_tab.left, "Command Input");
}

#[test]
fn test_app_state_current_tab_content() {
    let state = AppState::new();
    let content = state.current_tab_content();
    assert_eq!(content.left, "Agent Context");
    
    let mut state = AppState::new();
    state.ui_state.switch_tab(Tab::Git);
    let content = state.current_tab_content();
    assert_eq!(content.left, "Git Status");
}

#[test]
fn test_tab_all() {
    let tabs = Tab::all();
    assert_eq!(tabs.len(), 3);
    assert!(tabs.contains(&Tab::Agent));
    assert!(tabs.contains(&Tab::Git));
    assert!(tabs.contains(&Tab::Cli));
}

#[test]
fn test_column_all() {
    let columns = Column::all();
    assert_eq!(columns.len(), 3);
    assert!(columns.contains(&Column::Left));
    assert!(columns.contains(&Column::Center));
    assert!(columns.contains(&Column::Right));
}
