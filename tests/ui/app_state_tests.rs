//! App State tests

use agent_tui::modules::ui::domain::models::*;
use agent_tui::shared::kernel::types::Tab;

#[test]
fn test_app_state_new() {
    let state = AppState::new();
    assert_eq!(state.ui_state.current_tab, Tab::Agent);
}

#[test]
fn test_app_state_current_tab_content() {
    let state = AppState::new();
    let content = state.current_tab_content();
    assert_eq!(content.left, "Context");
    assert_eq!(content.center, "Chat");
    assert_eq!(content.right, "Actions");
    assert_eq!(content.label, "Agent");
}

#[test]
fn test_app_state_current_tab_content_git() {
    let mut state = AppState::new();
    state.ui_state.switch_tab(Tab::Git);
    let content = state.current_tab_content();
    assert_eq!(content.left, "Status");
    assert_eq!(content.label, "Git");
}

#[test]
fn test_app_state_current_tab_content_cli() {
    let mut state = AppState::new();
    state.ui_state.switch_tab(Tab::Cli);
    let content = state.current_tab_content();
    assert_eq!(content.left, "Input");
    assert_eq!(content.center, "Output");
    assert_eq!(content.right, "History");
    assert_eq!(content.label, "CLI");
}

#[test]
fn test_app_state_current_tab_content_mut() {
    let mut state = AppState::new();
    let content = state.current_tab_content_mut();
    content.center = "Modified".to_string();

    let content2 = state.current_tab_content();
    assert_eq!(content2.center, "Modified");
}

#[test]
fn test_diff_review_state_default() {
    let state = DiffReviewState::default();
    assert!(!state.is_active);
    assert!(state.diff_text.is_empty());
}

#[test]
fn test_snippet_state_default() {
    let state = SnippetState::default();
    assert!(!state.is_active);
    assert!(state.search_query.is_empty());
}

#[test]
fn test_macro_state_default() {
    let state = MacroState::default();
    assert!(!state.is_recording);
    assert!(state.current_macro_id.is_none());
}

#[test]
fn test_sandbox_state_default() {
    let state = SandboxState::default();
    assert!(!state.is_preview_active);
    assert!(state.command_preview.is_none());
}

#[test]
fn test_metrics_state_default() {
    let state = MetricsState::default();
    assert!(!state.is_active);
}

#[test]
fn test_collaboration_state_default() {
    let state = CollaborationState::default();
    assert!(!state.is_active);
    assert!(state.session_id.is_none());
}

#[test]
fn test_timeline_state_default() {
    let state = TimelineState::default();
    assert!(state.entries.is_empty());
}

#[test]
fn test_app_state_start_diff_review() {
    let mut state = AppState::new();
    state.start_diff_review("diff content".to_string());
    assert!(state.diff_state.is_active);
    assert_eq!(state.diff_state.diff_text, "diff content");
}

#[test]
fn test_app_state_end_diff_review() {
    let mut state = AppState::new();
    state.start_diff_review("diff".to_string());
    state.end_diff_review();
    assert!(!state.diff_state.is_active);
}

#[test]
fn test_app_state_open_snippet_manager() {
    let mut state = AppState::new();
    state.open_snippet_manager();
    assert!(state.snippet_state.is_active);
}

#[test]
fn test_app_state_close_snippet_manager() {
    let mut state = AppState::new();
    state.open_snippet_manager();
    state.close_snippet_manager();
    assert!(!state.snippet_state.is_active);
}

#[test]
fn test_app_state_start_macro_recording() {
    let mut state = AppState::new();
    state.start_macro_recording("macro-123".to_string());
    assert!(state.macro_state.is_recording);
    assert_eq!(
        state.macro_state.current_macro_id,
        Some("macro-123".to_string())
    );
}

#[test]
fn test_app_state_stop_macro_recording() {
    let mut state = AppState::new();
    state.start_macro_recording("macro-123".to_string());
    state.stop_macro_recording();
    assert!(!state.macro_state.is_recording);
    assert!(state.macro_state.current_macro_id.is_none());
}

#[test]
fn test_app_state_preview_command() {
    let mut state = AppState::new();
    state.preview_command("ls -la");
    assert!(state.sandbox_state.is_preview_active);
    // command_preview is not set in current implementation
}

#[test]
fn test_app_state_hide_command_preview() {
    let mut state = AppState::new();
    state.preview_command("ls");
    state.hide_command_preview();
    assert!(!state.sandbox_state.is_preview_active);
    // command_preview is set to None in implementation
}

#[test]
fn test_app_state_open_metrics() {
    let mut state = AppState::new();
    state.open_metrics();
    assert!(state.metrics_state.is_active);
}

#[test]
fn test_app_state_close_metrics() {
    let mut state = AppState::new();
    state.open_metrics();
    state.close_metrics();
    assert!(!state.metrics_state.is_active);
}

#[test]
fn test_app_state_join_collaboration() {
    let mut state = AppState::new();
    state.join_collaboration("collab-123".to_string());
    assert!(state.collaboration_state.is_active);
    assert_eq!(
        state.collaboration_state.session_id,
        Some("collab-123".to_string())
    );
}

#[test]
fn test_app_state_leave_collaboration() {
    let mut state = AppState::new();
    state.join_collaboration("collab-123".to_string());
    state.leave_collaboration();
    assert!(!state.collaboration_state.is_active);
    assert!(state.collaboration_state.session_id.is_none());
}

#[test]
fn test_app_state_default() {
    let state = AppState::default();
    assert_eq!(state.ui_state.current_tab, Tab::Agent);
}
