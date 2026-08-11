use super::tab_action_types::TabAction;
use crate::modules::ui::domain::models::AppState;
use crate::shared::kernel::result::AppResult;
use files_tui::FilesUseCase;

/// Files tab action handler — delegates to files-app
pub(crate) fn handle_files_action(state: &mut AppState, action: TabAction) -> AppResult<()> {
    let mut uc = FilesUseCase::new();
    match action {
        TabAction::OpenFile => {
            // Open file via files-app adapter
        }
        TabAction::CreateFile => {
            state.files_tab_state.is_editing = true;
        }
        TabAction::Refresh => {
            let _ = uc.refresh();
        }
        _ => {}
    }
    Ok(())
}
