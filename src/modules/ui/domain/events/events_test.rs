use crate::shared::types::{Column, Tab};
use crate::modules::ui::domain::events::UIEvent;

#[test]
fn test_ui_event_tab_changed() {
    let event = UIEvent::tab_changed(Tab::Agent, Tab::Git);
    assert_eq!(event, UIEvent::TabChanged { from: Tab::Agent, to: Tab::Git });
}

#[test]
fn test_ui_event_column_changed() {
    let event = UIEvent::column_changed(Column::Left, Column::Center);
    assert_eq!(event, UIEvent::ColumnChanged { from: Column::Left, to: Column::Center });
}

#[test]
fn test_ui_event_focus_toggled() {
    let event = UIEvent::focus_toggled(true);
    assert_eq!(event, UIEvent::FocusToggled { is_focused: true });
}

#[test]
fn test_ui_event_content_updated() {
    let event = UIEvent::content_updated(Tab::Agent);
    assert_eq!(event, UIEvent::ContentUpdated { tab: Tab::Agent });
}
