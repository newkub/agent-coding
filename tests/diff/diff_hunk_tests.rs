//! Diff Hunk tests

use agent_tui::modules::diff::domain::models::{DiffHunk, DiffLine, HunkStatus};

#[test]
fn test_diff_hunk_new() {
    let hunk = DiffHunk::create(
        uuid::Uuid::new_v4().to_string(),
        "@@ -1,3 +1,4 @@".to_string(),
        1,
        3,
        1,
        4,
        vec![DiffLine::addition("+ new line".to_string(), 5)],
    );
    assert!(!hunk.id.is_empty());
    assert_eq!(hunk.status, HunkStatus::Pending);
}

#[test]
fn test_diff_hunk_approve() {
    let mut hunk = DiffHunk::create(
        uuid::Uuid::new_v4().to_string(),
        "@@ -1,3 +1,4 @@".to_string(),
        1,
        3,
        1,
        4,
        vec![],
    );
    hunk.approve();
    assert_eq!(hunk.status, HunkStatus::Approved);
}

#[test]
fn test_diff_hunk_reject() {
    let mut hunk = DiffHunk::create(
        uuid::Uuid::new_v4().to_string(),
        "@@ -1,3 +1,4 @@".to_string(),
        1,
        3,
        1,
        4,
        vec![],
    );
    hunk.reject();
    assert_eq!(hunk.status, HunkStatus::Rejected);
}

#[test]
fn test_diff_hunk_is_pending() {
    let mut hunk = DiffHunk::create(
        uuid::Uuid::new_v4().to_string(),
        "@@".to_string(),
        1,
        1,
        1,
        1,
        vec![],
    );
    assert!(hunk.is_pending());
    hunk.approve();
    assert!(!hunk.is_pending());
}

#[test]
fn test_hunk_status_variants() {
    assert!(matches!(HunkStatus::Pending, HunkStatus::Pending));
    assert!(matches!(HunkStatus::Approved, HunkStatus::Approved));
    assert!(matches!(HunkStatus::Rejected, HunkStatus::Rejected));
}
