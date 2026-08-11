use super::TabAction;
use crate::modules::ui::domain::models::AppState;
use crate::shared::kernel::result::AppResult;
use api_tui::ApiUseCase;

/// API tab action handler — delegates to api-app
pub(crate) fn handle_api_action(state: &mut AppState, action: TabAction) -> AppResult<()> {
    let mut uc = ApiUseCase::new();
    match action {
        TabAction::Execute => {
            state.api_tab_state.is_editing = false;
            // In a real integration, would call uc.set_request + async execute
        }
        TabAction::Clear => {
            state.api_tab_state.is_editing = false;
        }
        TabAction::Refresh => {
            state.api_tab_state.selected_endpoint_index = 0;
            uc.select_prev();
        }
        _ => {}
    }
    Ok(())
}
