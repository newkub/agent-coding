//! Diff Line tests

use agent_tui::modules::diff::domain::models::{DiffLine, DiffLineType};

#[test]
fn test_diff_line_context() {
    let line = DiffLine::context("content".to_string(), 1, 2);
    assert_eq!(line.line_type, DiffLineType::Context);
    assert_eq!(line.old_line_num, Some(1));
    assert_eq!(line.new_line_num, Some(2));
}

#[test]
fn test_diff_line_addition() {
    let line = DiffLine::addition("+ added".to_string(), 5);
    assert_eq!(line.line_type, DiffLineType::Addition);
    assert!(line.old_line_num.is_none());
    assert_eq!(line.new_line_num, Some(5));
}

#[test]
fn test_diff_line_deletion() {
    let line = DiffLine::deletion("- removed".to_string(), 3);
    assert_eq!(line.line_type, DiffLineType::Deletion);
    assert_eq!(line.old_line_num, Some(3));
    assert!(line.new_line_num.is_none());
}

#[test]
fn test_diff_line_type_variants() {
    assert!(matches!(DiffLineType::Context, DiffLineType::Context));
    assert!(matches!(DiffLineType::Addition, DiffLineType::Addition));
    assert!(matches!(DiffLineType::Deletion, DiffLineType::Deletion));
    assert!(matches!(DiffLineType::Header, DiffLineType::Header));
}
