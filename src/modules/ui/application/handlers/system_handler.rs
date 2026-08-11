use super::TabAction;
use crate::modules::ui::domain::models::AppState;
use crate::shared::kernel::result::AppResult;
use system_tui::SystemUseCase;

/// System tab action handler — delegates to system-app
pub(crate) fn handle_system_action(state: &mut AppState, action: TabAction) -> AppResult<()> {
    let mut uc = SystemUseCase::new();
    match action {
        TabAction::Refresh => {
            state.system_tab_state.selected_metric_index = 0;
            uc.select_prev();
        }
        TabAction::Clear => {
            state.system_tab_state.selected_metric_index = 0;
        }
        _ => {}
    }
    Ok(())
}
