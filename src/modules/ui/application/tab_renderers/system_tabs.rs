use super::TabColumns;
use crate::modules::ui::domain::models::AppState;

/// Render Logs tab columns — backed by `logs_tab_state`
/// (audit entries loaded from the audit repository)
pub(crate) fn render_logs_tab(state: &AppState) -> TabColumns {
    let tab_state = &state.logs_tab_state;

    let left = {
        let entries: Vec<String> = tab_state
            .entries
            .iter()
            .rev()
            .take(20)
            .enumerate()
            .map(|(i, e)| {
                let marker = if i == tab_state.selected_log_index {
                    ">"
                } else {
                    " "
                };
                format!(
                    "{marker} [{}] {} ({:?})",
                    e.timestamp.format("%H:%M:%S"),
                    e.action.category(),
                    e.result,
                )
            })
            .collect();
        if entries.is_empty() {
            "(no log entries)".to_string()
        } else {
            entries.join("\n")
        }
    };

    let center = {
        let len = tab_state.entries.len();
        // `left` shows newest first, so mirror that order here.
        match len
            .checked_sub(1 + tab_state.selected_log_index)
            .and_then(|i| tab_state.entries.get(i))
        {
            Some(e) => format!(
                "id: {}\ntime: {}\nactor: {} ({})\naction: {:?}\ncategory: {}\nresource: {} {}\nresult: {:?}",
                e.id.as_str(),
                e.timestamp.format("%Y-%m-%d %H:%M:%S"),
                e.actor.name,
                e.actor.id,
                e.action,
                e.action.category(),
                e.resource.type_,
                e.resource.id,
                e.result,
            ),
            None => "Select a log entry".to_string(),
        }
    };

    let right = format!(
        "Filter: {}\nEntries: {}\n\n[r] Refresh",
        tab_state.log_level_filter.as_deref().unwrap_or("all"),
        tab_state.entries.len(),
    );

    TabColumns::new(left, center, right)
}

/// Render System tab columns — backed by `system_tab_state`
/// (metrics collected via the sysinfo-backed collector)
pub(crate) fn render_system_tab(state: &AppState) -> TabColumns {
    let tab_state = &state.system_tab_state;

    let left = if tab_state.metrics.is_empty() {
        "No metrics collected\n\n[r] Refresh".to_string()
    } else {
        tab_state
            .metrics
            .iter()
            .enumerate()
            .map(|(i, (label, value))| {
                let marker = if i == tab_state.selected_metric_index {
                    ">"
                } else {
                    " "
                };
                format!("{marker} {label}: {value}")
            })
            .collect::<Vec<_>>()
            .join("\n")
    };

    let center = tab_state
        .metrics
        .get(tab_state.selected_metric_index)
        .map(|(label, value)| format!("{label}\n\n{value}"))
        .unwrap_or_else(|| "Select a metric".to_string());

    let right = if tab_state.alerts.is_empty() {
        "Alerts: none\n\n[r] Refresh".to_string()
    } else {
        format!("Alerts:\n{}", tab_state.alerts.join("\n"))
    };

    TabColumns::new(left, center, right)
}
