use super::TabColumns;
use crate::modules::ui::domain::models::AppState;

/// Render API tab columns — uses api-app for data
pub(crate) fn render_api_tab(state: &AppState) -> TabColumns {
    let tab_state = &state.api_tab_state;
    let api_uc = api_tui::ApiUseCase::new();

    let left = format!(
        "Endpoints: {}\nSelected: {}",
        api_uc.endpoints().len(),
        tab_state.selected_endpoint_index,
    );

    let center = if tab_state.is_editing {
        "Editing request…".to_string()
    } else {
        "No request selected".to_string()
    };

    let right = api_uc
        .last_response()
        .map(|r| format!("Status: {}\n\n{}", r.status, r.body))
        .unwrap_or_else(|| "No response".to_string());

    TabColumns::new(left, center, right)
}

/// Render Database tab columns — uses database-app for data
pub(crate) fn render_database_tab(state: &AppState) -> TabColumns {
    let tab_state = &state.database_tab_state;
    let db_uc = database_tui::DatabaseUseCase::new(
        database_tui::ConnectionConfig {
            url: String::new(),
            database: String::new(),
        },
        Box::new(database_tui::SqliteDbPort::new()),
    );

    let left = format!(
        "Tables: {}\nSelected: {}",
        db_uc.tables().len(),
        tab_state.selected_table_index,
    );

    let center = if tab_state.query_input.is_empty() {
        "Query:\n  (type a query)".to_string()
    } else {
        format!("Query:\n{}", tab_state.query_input)
    };

    let result = db_uc.formatted_result();
    let right = if result.is_empty() {
        "No result".to_string()
    } else {
        result
    };

    TabColumns::new(left, center, right)
}

/// Render Tasks tab columns — uses the task-tui task manager
pub(crate) fn render_tasks_tab(state: &AppState) -> TabColumns {
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

    let left = {
        let path_str = path.as_deref().unwrap_or("No workspace");
        let items: Vec<String> = tm
            .filtered_tasks()
            .iter()
            .take(15)
            .enumerate()
            .map(|(i, t)| {
                let marker = if i == tab.selected_task_index {
                    ">"
                } else {
                    " "
                };
                format!("{marker} {}", t.title)
            })
            .collect();
        if items.is_empty() {
            format!("{path_str}\n\n(no tasks)")
        } else {
            format!("{path_str}\n\n{}", items.join("\n"))
        }
    };

    let center = tm
        .selected_task()
        .map(|t| t.title.clone())
        .unwrap_or_else(|_| "No task selected".to_string());

    let right = format!(
        "Show Completed: {}\n\n{} total, {} filtered",
        tab.show_completed,
        tm.tasks().len(),
        tm.filtered_tasks().len(),
    );

    TabColumns::new(left, center, right)
}

/// Render Notes tab columns
pub(crate) fn render_notes_tab(state: &AppState) -> TabColumns {
    let tab_state = &state.notes_tab_state;

    let left = format!("Selected note: {}", tab_state.selected_note_index);
    let center = if tab_state.is_editing {
        "Editing note…".to_string()
    } else {
        "No note selected".to_string()
    };
    let right = "Tags: -".to_string();

    TabColumns::new(left, center, right)
}
