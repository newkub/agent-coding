use super::tab_action_types::TabAction;
use crate::modules::ui::domain::models::AppState;
use crate::shared::kernel::result::AppResult;

/// Git tab action handler.
///
/// State-only transitions; the real git operations (stage, unstage, commit,
/// push, diff) run asynchronously in `tab_effects` via the DI-provided
/// `Git2Adapter`.
pub(crate) fn handle_git_action(_state: &mut AppState, _action: TabAction) -> AppResult<()> {
    Ok(())
}
