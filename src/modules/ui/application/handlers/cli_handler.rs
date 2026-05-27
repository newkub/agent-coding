use crate::modules::ui::domain::models::AppState;
use crate::shared::kernel::result::AppResult;
use super::tab_action_types::TabAction;

/// CLI tab action handler
pub(super) fn handle_cli_action(state: &mut AppState, action: TabAction) -> AppResult<()> {
    match action {
        TabAction::RunCommand(cmd) => {
            state.cli_tab_state.command_input = cmd;
            // Execute would be handled elsewhere
        }
        TabAction::ClearOutput => {
            state.cli_tab_state.command_input.clear();
        }
        _ => {}
    }
    Ok(())
}
