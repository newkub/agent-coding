use super::TabAction;
use crate::modules::ui::domain::models::AppState;
use crate::shared::kernel::result::{AppError, AppResult};

/// Tasks tab action handler — delegates to the task-tui task manager.
pub(crate) fn handle_tasks_action(state: &mut AppState, action: TabAction) -> AppResult<()> {
    let tab = &mut state.tasks_tab_state;
    let map_err = |e: task_tui::task_manager::Error| AppError::State(e.to_string());
    match action {
        TabAction::Add(task) => {
            tab.task_manager.add(task, String::new()).map_err(map_err)?;
            tab.selected_task_index = 0;
        }
        TabAction::Remove(index) => {
            tab.task_manager.remove(index).map_err(map_err)?;
            tab.selected_task_index = index.saturating_sub(1);
        }
        TabAction::Toggle(index) => {
            tab.show_completed = !tab.show_completed;
            tab.task_manager.toggle_show_completed();
            tab.task_manager.navigate(index);
            tab.selected_task_index = index;
        }
        _ => {}
    }
    Ok(())
}
