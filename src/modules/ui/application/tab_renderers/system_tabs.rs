use super::TabRenderResult;
use crate::modules::ui::domain::models::AppState;
use ratatui::layout::Rect;
use ratatui::widgets::Paragraph;

/// Render Logs tab content
pub(crate) fn render_logs_tab(state: &AppState) -> TabRenderResult<'_> {
    let tab_state = &state.logs_tab_state;

    let content = Paragraph::new(format!(
        "Logs\n\nSelected Log: {}",
        tab_state.selected_log_index
    ));
    TabRenderResult::new(content, Rect::new(0, 0, 0, 0))
}

/// Render System tab content — uses system-app for data
pub(crate) fn render_system_tab(_state: &AppState) -> TabRenderResult<'_> {
    let sys_uc = system_tui::SystemUseCase::new();
    let metrics_text = sys_uc
        .metrics()
        .iter()
        .map(|m| format!("{:?}: {:.2}", m.metric_type, m.value))
        .collect::<Vec<_>>()
        .join("\n");

    let alerts = sys_uc.alerts();
    let alerts_text = if alerts.is_empty() {
        "No alerts".to_string()
    } else {
        alerts
            .iter()
            .map(|a| a.message.clone())
            .collect::<Vec<_>>()
            .join("\n")
    };

    let content = Paragraph::new(format!(
        "System\n\nMonitoring system resources\n\n(system-app)\n{}\n\nAlerts:\n{}",
        metrics_text, alerts_text
    ));
    TabRenderResult::new(content, Rect::new(0, 0, 0, 0))
}
