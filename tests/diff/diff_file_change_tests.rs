//! File Change tests

use agent_tui::modules::diff::domain::models::{ChangeType, DiffHunk, DiffLine, FileChange};

#[test]
fn test_file_change_new() {
    let fc = FileChange::new("test.rs".to_string(), ChangeType::Modified);
    assert_eq!(fc.path, "test.rs");
    assert_eq!(fc.change_type, ChangeType::Modified);
    assert!(fc.hunks.is_empty());
}

#[test]
fn test_file_change_with_hunks() {
    let hunk = DiffHunk::create(
        uuid::Uuid::new_v4().to_string(),
        "@@ header @@".to_string(),
        1,
        10,
        1,
        10,
        vec![DiffLine::context("line".to_string(), 1, 1)],
    );
    let fc = FileChange::new("test.rs".to_string(), ChangeType::Added).with_hunks(vec![hunk]);
    assert_eq!(fc.hunk_count(), 1);
}

#[test]
fn test_change_type_variants() {
    let ct = ChangeType::Added;
    assert!(matches!(ct, ChangeType::Added));

    let ct = ChangeType::Modified;
    assert!(matches!(ct, ChangeType::Modified));

    let ct = ChangeType::Deleted;
    assert!(matches!(ct, ChangeType::Deleted));

    let ct = ChangeType::Renamed;
    assert!(matches!(ct, ChangeType::Renamed));
}
