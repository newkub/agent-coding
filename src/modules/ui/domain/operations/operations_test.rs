use crate::shared::types::{Column, Tab};
use crate::modules::ui::domain::operations::{
    calculate_next_column, calculate_next_tab, calculate_prev_tab, column_to_index,
    validate_tab_transition,
};

#[test]
fn test_calculate_next_tab() {
    assert_eq!(calculate_next_tab(Tab::Agent), Tab::Git);
    assert_eq!(calculate_next_tab(Tab::Git), Tab::Cli);
    assert_eq!(calculate_next_tab(Tab::Cli), Tab::Agent);
}

#[test]
fn test_calculate_prev_tab() {
    assert_eq!(calculate_prev_tab(Tab::Agent), Tab::Cli);
    assert_eq!(calculate_prev_tab(Tab::Git), Tab::Agent);
    assert_eq!(calculate_prev_tab(Tab::Cli), Tab::Git);
}

#[test]
fn test_calculate_next_column() {
    assert_eq!(calculate_next_column(Column::Left), Column::Center);
    assert_eq!(calculate_next_column(Column::Center), Column::Right);
    assert_eq!(calculate_next_column(Column::Right), Column::Left);
}

#[test]
fn test_validate_tab_transition() {
    assert!(validate_tab_transition(Tab::Agent, Tab::Git));
    assert!(validate_tab_transition(Tab::Git, Tab::Cli));
    assert!(!validate_tab_transition(Tab::Agent, Tab::Agent));
}

#[test]
fn test_column_to_index() {
    assert_eq!(column_to_index(Column::Left), 0);
    assert_eq!(column_to_index(Column::Center), 1);
    assert_eq!(column_to_index(Column::Right), 2);
}
