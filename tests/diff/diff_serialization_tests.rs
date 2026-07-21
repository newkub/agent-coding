//! Diff serialization tests

use agent_tui::modules::diff::domain::models::{ChangeType, DiffReview, FileChange};

#[test]
fn test_diff_review_serialization() {
    let fc = FileChange::new("test.rs".to_string(), ChangeType::Modified);
    let review = DiffReview::new(vec![fc]);
    let json = serde_json::to_string(&review).unwrap();
    let parsed: DiffReview = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.files.len(), 1);
}

#[test]
fn test_file_change_serialization() {
    let fc = FileChange::new("test.rs".to_string(), ChangeType::Added);
    let json = serde_json::to_string(&fc).unwrap();
    let parsed: FileChange = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.path, "test.rs");
}
