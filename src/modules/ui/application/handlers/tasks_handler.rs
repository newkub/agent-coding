use super::TabAction;
use crate::modules::ui::domain::models::AppState;
use crate::shared::kernel::result::AppResult;
use tasks_tui::TaskUseCase;

/// Tasks tab action handler — delegates to tasks-app
pub(crate) fn handle_tasks_action(state: &mut AppState, action: TabAction) -> AppResult<()> {
    let mut uc = TaskUseCase::new();
    match action {
        TabAction::Add(task) => {
            uc.add(task, String::new());
            state.tasks_tab_state.selected_task_index = 0;
        }
        TabAction::Remove(index) if index > 0 => {
            state.tasks_tab_state.selected_task_index = index - 1;
        }
        TabAction::Toggle(index) => {
            state.tasks_tab_state.show_completed = !state.tasks_tab_state.show_completed;
            uc.toggle_show_completed();
            uc.navigate(index);
        }
        _ => {}
    }
    Ok(())
}
