//! UI State tests

use agent_tui::shared::kernel::types::{UIState, Tab, Column};

#[test]
fn test_ui_state_new() {
    let state = UIState::new();
    assert_eq!(state.current_tab, Tab::Agent);
    assert_eq!(state.current_column, Column::Center);
    assert!(state.is_focused);
}

#[test]
fn test_ui_state_default() {
    let state = UIState::default();
    assert_eq!(state.current_tab, Tab::Agent);
}

#[test]
fn test_ui_state_switch_tab() {
    let mut state = UIState::new();
    state.switch_tab(Tab::Git);
    assert_eq!(state.current_tab, Tab::Git);
}

#[test]
fn test_ui_state_next_tab() {
    let mut state = UIState::new();
    state.next_tab();
    assert_eq!(state.current_tab, Tab::Packages);
    state.next_tab();
    assert_eq!(state.current_tab, Tab::Files);
    state.next_tab();
    assert_eq!(state.current_tab, Tab::Git);
    state.next_tab();
    assert_eq!(state.current_tab, Tab::Terminal);
    state.next_tab();
    assert_eq!(state.current_tab, Tab::Snippet);
    state.next_tab();
    assert_eq!(state.current_tab, Tab::Snippets);
    state.next_tab();
    assert_eq!(state.current_tab, Tab::Api);
    state.next_tab();
    assert_eq!(state.current_tab, Tab::Database);
    state.next_tab();
    assert_eq!(state.current_tab, Tab::Tasks);
    state.next_tab();
    assert_eq!(state.current_tab, Tab::Notes);
    state.next_tab();
    assert_eq!(state.current_tab, Tab::Logs);
    state.next_tab();
    assert_eq!(state.current_tab, Tab::System);
    state.next_tab();
    assert_eq!(state.current_tab, Tab::Skills);
    state.next_tab();
    assert_eq!(state.current_tab, Tab::Workflows);
    state.next_tab();
    assert_eq!(state.current_tab, Tab::Settings);
    state.next_tab();
    assert_eq!(state.current_tab, Tab::Cli);
    state.next_tab();
    assert_eq!(state.current_tab, Tab::Agent);
}

#[test]
fn test_ui_state_prev_tab() {
    let mut state = UIState::new();
    state.prev_tab();
    assert_eq!(state.current_tab, Tab::Cli);
    state.prev_tab();
    assert_eq!(state.current_tab, Tab::Settings);
    state.prev_tab();
    assert_eq!(state.current_tab, Tab::Workflows);
    state.prev_tab();
    assert_eq!(state.current_tab, Tab::Skills);
    state.prev_tab();
    assert_eq!(state.current_tab, Tab::System);
    state.prev_tab();
    assert_eq!(state.current_tab, Tab::Logs);
    state.prev_tab();
    assert_eq!(state.current_tab, Tab::Notes);
    state.prev_tab();
    assert_eq!(state.current_tab, Tab::Tasks);
    state.prev_tab();
    assert_eq!(state.current_tab, Tab::Database);
    state.prev_tab();
    assert_eq!(state.current_tab, Tab::Api);
    state.prev_tab();
    assert_eq!(state.current_tab, Tab::Snippets);
    state.prev_tab();
    assert_eq!(state.current_tab, Tab::Snippet);
    state.prev_tab();
    assert_eq!(state.current_tab, Tab::Terminal);
    state.prev_tab();
    assert_eq!(state.current_tab, Tab::Git);
    state.prev_tab();
    assert_eq!(state.current_tab, Tab::Files);
    state.prev_tab();
    assert_eq!(state.current_tab, Tab::Packages);
    state.prev_tab();
    assert_eq!(state.current_tab, Tab::Agent);
}

#[test]
fn test_ui_state_next_column() {
    let mut state = UIState::new();
    state.next_column();
    assert_eq!(state.current_column, Column::Right);
    state.next_column();
    assert_eq!(state.current_column, Column::Left);
}

#[test]
fn test_ui_state_toggle_focus() {
    let mut state = UIState::new();
    assert!(state.is_focused);
    state.toggle_focus();
    assert!(!state.is_focused);
    state.toggle_focus();
    assert!(state.is_focused);
}
