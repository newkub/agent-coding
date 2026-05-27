use crate::shared::types::{Column, Tab};
use crate::modules::ui::domain::validators::{validate_column, validate_tab, validate_ui_state};

#[test]
fn test_validate_tab() {
    assert!(validate_tab(Tab::Agent));
    assert!(validate_tab(Tab::Git));
    assert!(validate_tab(Tab::Cli));
}

#[test]
fn test_validate_column() {
    assert!(validate_column(Column::Left));
    assert!(validate_column(Column::Center));
    assert!(validate_column(Column::Right));
}

#[test]
fn test_validate_ui_state() {
    assert!(validate_ui_state(Tab::Agent, Column::Center));
    assert!(validate_ui_state(Tab::Git, Column::Left));
    assert!(validate_ui_state(Tab::Cli, Column::Right));
}
