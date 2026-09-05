use super::TabAction;
use crate::modules::ui::domain::models::AppState;
use crate::shared::kernel::result::AppResult;

/// API tab action handler — updates request state; execution runs in `tab_effects`.
pub(crate) fn handle_api_action(state: &mut AppState, action: TabAction) -> AppResult<()> {
    match action {
        TabAction::Input(content) => {
            state.api_tab_state.request_url = content;
        }
        TabAction::Edit(_, content) => {
            state.api_tab_state.request_body = content;
        }
        TabAction::Execute => {
            state.api_tab_state.is_executing = true;
            state.api_tab_state.is_editing = false;
        }
        TabAction::Clear => {
            state.api_tab_state.response.clear();
            state.api_tab_state.is_executing = false;
            state.api_tab_state.is_editing = false;
        }
        TabAction::Refresh => {
            state.api_tab_state.selected_endpoint_index = 0;
            state.api_tab_state.is_executing = false;
        }
        TabAction::Toggle(index) => {
            state.api_tab_state.selected_endpoint_index = index;
        }
        _ => {}
    }
    Ok(())
}
