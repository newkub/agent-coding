use super::TabColumns;
use crate::modules::ui::domain::models::AppState;

/// Render Logs tab columns
pub(crate) fn render_logs_tab(state: &AppState) -> TabColumns {
    let tab_state = &state.logs_tab_state;

    let left = format!(
        "Level: {}\nSelected: {}",
        tab_state.log_level_filter.as_deref().unwrap_or("all"),
        tab_state.selected_log_index,
    );
    let center = "No logs loaded".to_string();
    let right = "Filters: -".to_string();

    TabColumns::new(left, center, right)
}

/// Render System tab columns — uses system-app for data
pub(crate) fn render_system_tab(state: &AppState) -> TabColumns {
    let sys_uc = system_tui::SystemUseCase::new();

    let left = if sys_uc.metrics().is_empty() {
        "No metrics collected".to_string()
    } else {
        sys_uc
            .metrics()
            .iter()
            .enumerate()
            .map(|(i, m)| {
                let marker = if i == state.system_tab_state.selected_metric_index {
                    ">"
                } else {
                    " "
                };
                format!("{marker} {}: {:.2}", m.label, m.value)
            })
            .collect::<Vec<_>>()
            .join("\n")
    };

    let center = format!(
        "Selected metric: {}",
        state.system_tab_state.selected_metric_index
    );

    let alerts = sys_uc.alerts();
    let right = if alerts.is_empty() {
        "No alerts".to_string()
    } else {
        alerts
            .iter()
            .map(|a| a.message.clone())
            .collect::<Vec<_>>()
            .join("\n")
    };

    TabColumns::new(left, center, right)
}
