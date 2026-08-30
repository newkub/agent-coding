use super::TabRenderResult;
use crate::modules::ui::domain::models::AppState;
use ratatui::layout::Rect;
use ratatui::widgets::Paragraph;

/// Render API tab content — uses api-app for data
pub(crate) fn render_api_tab(state: &AppState) -> TabRenderResult<'_> {
    let tab_state = &state.api_tab_state;
    let api_uc = api_tui::ApiUseCase::new();

    let content = Paragraph::new(format!(
        "API\n\nSelected Endpoint: {}\n\n(api-app: {} endpoints, last response: {})",
        tab_state.selected_endpoint_index,
        api_uc.endpoints().len(),
        api_uc
            .last_response()
            .map(|r| format!("{} {}", r.status, r.body))
            .unwrap_or_else(|| "None".to_string()),
    ));
    TabRenderResult::new(content, Rect::new(0, 0, 0, 0))
}

/// Render Database tab content — uses database-app for data
pub(crate) fn render_database_tab(state: &AppState) -> TabRenderResult<'_> {
    let tab_state = &state.database_tab_state;
    let db_uc = database_tui::DatabaseUseCase::new();

    let content = Paragraph::new(format!(
        "Database\n\nSelected Table: {}\nQuery: {}\n\n(database-app: {} tables, result: {})",
        tab_state.selected_table_index,
        tab_state.query_input,
        db_uc.tables().len(),
        db_uc.formatted_result(),
    ));
    TabRenderResult::new(content, Rect::new(0, 0, 0, 0))
}

/// Render Tasks tab content — uses the task-tui task manager.
pub(crate) fn render_tasks_tab(state: &AppState) -> TabRenderResult<'_> {
    let tab = &state.tasks_tab_state;
    let tm = &tab.task_manager;

    let path = tm.tree.selected.workspace.and_then(|ws_id| {
        tm.tree.find_workspace(ws_id).map(|ws| {
            let sp_name = tm
                .tree
                .selected
                .space
                .and_then(|sp_id| ws.find_space(sp_id))
                .map(|sp| sp.name.as_str())
                .unwrap_or("-");
            let list_name = tm
                .tree
                .selected
                .list
                .and_then(|list_id| ws.find_list(list_id))
                .map(|list| list.name.as_str())
                .unwrap_or("-");
            format!("{} / {} / {}", ws.name, sp_name, list_name)
        })
    });

    let total = tm.tasks().len();
    let filtered = tm.filtered_tasks().len();
    let selected = tm.selected_task().map(|t| t.title.as_str()).unwrap_or("-");
    let path_str = path.as_deref().unwrap_or("No workspace");

    let content = Paragraph::new(format!(
        "Tasks\n\n{path_str}\n\nSelected Task: {selected} ({})\nShow Completed: {}\n\n(task-tui: {} total, {} filtered)",
        tab.selected_task_index,
        tab.show_completed,
        total,
        filtered,
    ));
    TabRenderResult::new(content, Rect::new(0, 0, 0, 0))
}

/// Render Notes tab content
pub(crate) fn render_notes_tab(state: &AppState) -> TabRenderResult<'_> {
    let tab_state = &state.notes_tab_state;

    let content = Paragraph::new(format!(
        "Notes\n\nSelected Note: {}",
        tab_state.selected_note_index
    ));
    TabRenderResult::new(content, Rect::new(0, 0, 0, 0))
}
