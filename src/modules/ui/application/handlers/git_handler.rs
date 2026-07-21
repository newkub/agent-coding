use super::tab_action_types::TabAction;
use crate::modules::ui::domain::models::AppState;
use crate::shared::kernel::result::AppResult;

/// Git tab action handler
pub(crate) fn handle_git_action(state: &mut AppState, action: TabAction) -> AppResult<()> {
    match action {
        TabAction::Stage => {
            // Move selected file from unstaged to staged
            if let Some(idx) = state.git_tab_state.unstaged_files.pop() {
                state.git_tab_state.staged_files.push(idx);
            }
        }
        TabAction::Unstage => {
            // Move selected file from staged to unstaged
            if let Some(idx) = state.git_tab_state.staged_files.pop() {
                state.git_tab_state.unstaged_files.push(idx);
            }
        }
        TabAction::Commit(msg) => {
            // Clear staged files after commit
            state.git_tab_state.staged_files.clear();
            println!("Committed: {}", msg);
        }
        _ => {}
    }
    Ok(())
}
