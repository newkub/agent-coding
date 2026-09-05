use super::TabAction;
use crate::modules::ui::domain::models::AppState;
use crate::shared::kernel::result::AppResult;
use terminal_tui::TerminalUseCase;

/// Terminal tab action handler — delegates to terminal-app
pub(crate) fn handle_terminal_action(state: &mut AppState, action: TabAction) -> AppResult<()> {
    let mut uc = TerminalUseCase::new();
    match action {
        TabAction::Input(text) => {
            state.terminal_tab_state.terminal_input = text;
        }
        TabAction::Execute => {
            let input = terminal_tui::TerminalInput {
                command: state.terminal_tab_state.terminal_input.clone(),
            };
            if let Ok(cmd) = uc.submit_input(input) {
                let _ = cmd; // command parsed and validated
            }
            state.terminal_tab_state.selected_history_index = None;
        }
        TabAction::Clear => {
            state.terminal_tab_state.terminal_input.clear();
            state.terminal_tab_state.output.clear();
            state.terminal_tab_state.history.clear();
            uc.clear_history();
        }
        _ => {}
    }
    Ok(())
}
