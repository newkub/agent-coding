use super::tab_action_types::TabAction;
use crate::modules::ui::domain::models::AppState;
use crate::shared::kernel::result::AppResult;
use git_tui::GitUseCase;

/// Git tab action handler — delegates to git-app
pub(crate) fn handle_git_action(state: &mut AppState, action: TabAction) -> AppResult<()> {
    let mut uc = GitUseCase::new();
    match action {
        TabAction::Stage => {
            if let Some(idx) = state.git_tab_state.unstaged_files.pop() {
                uc.stage(&idx);
                state.git_tab_state.staged_files.push(idx);
            }
        }
        TabAction::Unstage => {
            if let Some(idx) = state.git_tab_state.staged_files.pop() {
                uc.unstage(&idx);
                state.git_tab_state.unstaged_files.push(idx);
            }
        }
        TabAction::Commit(msg) => {
            state.git_tab_state.staged_files.clear();
            let _ = uc.commit(&msg);
        }
        _ => {}
    }
    Ok(())
}
