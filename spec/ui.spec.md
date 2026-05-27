# UI Domain Tests

## UIState
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_ui_state_new | Creates state with defaults | `UIState::new()` |
| ✅ | test_ui_state_default | Default has Agent tab | `UIState::default()` |
| ✅ | test_ui_state_switch_tab | Can switch tabs | `state.switch_tab(Tab::Git)` |
| ✅ | test_ui_state_next_tab | Can navigate to next tab | `state.next_tab()` |
| ✅ | test_ui_state_prev_tab | Can navigate to previous tab | `state.prev_tab()` |
| ✅ | test_ui_state_next_column | Can navigate to next column | `state.next_column()` |
| ✅ | test_ui_state_toggle_focus | Can toggle focus | `state.toggle_focus()` |

## TabContent
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_tab_content_new | Creates empty content | `TabContent::new()` |
| ✅ | test_tab_content_with_content | Creates content with values | `TabContent::with_content("content")` |

## AppState
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_app_state_new | Creates app state | `AppState::new()` |
| ✅ | test_app_state_current_tab_content | Gets current tab content | `state.current_tab_content()` |
| ✅ | test_app_state_current_tab_content_git | Gets Git tab content | `state.current_tab_content_git()` |
| ✅ | test_app_state_current_tab_content_cli | Gets CLI tab content | `state.current_tab_content_cli()` |
| ✅ | test_app_state_current_tab_content_mut | Can modify current tab content | `state.current_tab_content_mut()` |
| ✅ | test_app_state_default | Default has Agent tab | `AppState::default()` |
| ✅ | test_app_state_start_diff_review | Starts diff review | `state.start_diff_review(diff)` |
| ✅ | test_app_state_end_diff_review | Ends diff review | `state.end_diff_review()` |
| ✅ | test_app_state_open_snippet_manager | Opens snippet manager | `state.open_snippet_manager()` |
| ✅ | test_app_state_close_snippet_manager | Closes snippet manager | `state.close_snippet_manager()` |
| ✅ | test_app_state_start_macro_recording | Starts macro recording | `state.start_macro_recording()` |
| ✅ | test_app_state_stop_macro_recording | Stops macro recording | `state.stop_macro_recording()` |
| ✅ | test_app_state_preview_command | Previews command | `state.preview_command("command")` |
| ✅ | test_app_state_hide_command_preview | Hides command preview | `state.hide_command_preview()` |
| ✅ | test_app_state_open_metrics | Opens metrics | `state.open_metrics()` |
| ✅ | test_app_state_close_metrics | Closes metrics | `state.close_metrics()` |
| ✅ | test_app_state_join_collaboration | Joins collaboration | `state.join_collaboration(collab)` |
| ✅ | test_app_state_leave_collaboration | Leaves collaboration | `state.leave_collaboration()` |

## Sub States
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_diff_review_state_default | Default diff state | `DiffReviewState::default()` |
| ✅ | test_snippet_state_default | Default snippet state | `SnippetState::default()` |
| ✅ | test_macro_state_default | Default macro state | `MacroState::default()` |
| ✅ | test_sandbox_state_default | Default sandbox state | `SandboxState::default()` |
| ✅ | test_metrics_state_default | Default metrics state | `MetricsState::default()` |
| ✅ | test_collaboration_state_default | Default collaboration state | `CollaborationState::default()` |
| ✅ | test_timeline_state_default | Default timeline state | `TimelineState::default()` |

## Tab
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_tab_all | Returns all tabs | `Tab::all()` |
| ✅ | test_tab_next | Gets next tab | `Tab::next(Tab::Agent)` |
| ✅ | test_tab_prev | Gets previous tab | `Tab::prev(Tab::Agent)` |
| ✅ | test_tab_label | Gets tab label | `Tab::Agent.label()` |

## Column
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_column_all | Returns all columns | `Column::all()` |
| ✅ | test_column_next | Gets next column | `Column::next(Column::Left)` |
| ✅ | test_column_prev | Gets previous column | `Column::prev(Column::Left)` |
| ✅ | test_column_label | Gets column label | `Column::Left.label()` |
