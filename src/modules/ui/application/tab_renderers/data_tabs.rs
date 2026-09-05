use super::TabColumns;
use crate::modules::ui::domain::models::AppState;

/// Render API tab columns — backed by `api_tab_state` request/response fields.
pub(crate) fn render_api_tab(state: &AppState) -> TabColumns {
    let tab_state = &state.api_tab_state;

    let left = format!(
        "Selected: {}\nExecuting: {}",
        tab_state.selected_endpoint_index, tab_state.is_executing
    );

    let center = if tab_state.is_editing {
        format!(
            "Editing request\nURL: {}\nMethod: {}\nBody: {}",
            tab_state.request_url, tab_state.request_method, tab_state.request_body
        )
    } else {
        format!(
            "Request\nURL: {}\nMethod: {}",
            tab_state.request_url, tab_state.request_method
        )
    };

    let right = if tab_state.response.is_empty() {
        "No response".to_string()
    } else {
        tab_state.response.clone()
    };

    TabColumns::new(left, center, right)
}

/// Render Database tab columns — backed by `database_tab_state`
/// (tables queried from the real SQLite database)
pub(crate) fn render_database_tab(state: &AppState) -> TabColumns {
    let tab_state = &state.database_tab_state;

    let left = {
        let tables: Vec<String> = tab_state
            .tables
            .iter()
            .enumerate()
            .map(|(i, t)| {
                let marker = if i == tab_state.selected_table_index {
                    ">"
                } else {
                    " "
                };
                format!("{marker} {t}")
            })
            .collect();
        if tables.is_empty() {
            "(no tables)".to_string()
        } else {
            tables.join("\n")
        }
    };

    let center = if tab_state.query_input.is_empty() {
        "Query:\n  (type a query, Enter runs it)".to_string()
    } else {
        format!("Query:\n{}", tab_state.query_input)
    };

    let right = if tab_state.results.is_empty() {
        "Results:\n  (none — Enter on a table previews rows)".to_string()
    } else {
        format!(
            "Results:\n{}",
            tab_state
                .results
                .iter()
                .take(30)
                .cloned()
                .collect::<Vec<_>>()
                .join("\n")
        )
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

/// Render Notes tab columns — backed by `notes_tab_state`
pub(crate) fn render_notes_tab(state: &AppState) -> TabColumns {
    let tab_state = &state.notes_tab_state;

    let left = {
        let notes: Vec<String> = tab_state
            .notes
            .iter()
            .enumerate()
            .map(|(i, n)| {
                let marker = if i == tab_state.selected_note_index {
                    ">"
                } else {
                    " "
                };
                format!("{marker} {}", n.title)
            })
            .collect();
        if notes.is_empty() {
            "(no notes)".to_string()
        } else {
            notes.join("\n")
        }
    };

    let center = match tab_state.notes.get(tab_state.selected_note_index) {
        Some(note) => {
            if tab_state.is_editing {
                format!("Editing: {}\n\n{}", note.title, note.content)
            } else {
                format!("{}\n\n{}", note.title, note.content)
            }
        }
        None => "No note selected".to_string(),
    };

    let right = format!(
        "Notes: {}\nEditing: {}\n\n[Enter] Toggle edit",
        tab_state.notes.len(),
        tab_state.is_editing,
    );

    TabColumns::new(left, center, right)
}
