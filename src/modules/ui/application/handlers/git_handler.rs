use super::tab_action_types::TabAction;
use crate::modules::ui::domain::models::AppState;
use crate::shared::kernel::result::AppResult;
use git_tui::{GitCliRepository, GitRepository};
use std::sync::Arc;

/// Git tab action handler — delegates to git-app
pub(crate) fn handle_git_action(state: &mut AppState, action: TabAction) -> AppResult<()> {
    let repo = match std::env::current_dir() {
        Ok(path) => match GitCliRepository::new(path.as_path()) {
            Ok(repo) => Arc::new(repo) as Arc<dyn GitRepository>,
            Err(_) => return Ok(()),
        },
        Err(_) => return Ok(()),
    };

    match action {
        TabAction::Stage => {
            if let Some(idx) = state.git_tab_state.unstaged_files.pop() {
                let _ = repo.stage(&idx);
                state.git_tab_state.staged_files.push(idx);
            }
        }
        TabAction::Unstage => {
            if let Some(idx) = state.git_tab_state.staged_files.pop() {
                let _ = repo.unstage(&idx);
                state.git_tab_state.unstaged_files.push(idx);
            }
        }
        TabAction::Commit(msg) => {
            state.git_tab_state.staged_files.clear();
            let _ = repo.commit(&msg);
        }
        _ => {}
    }
    Ok(())
}
