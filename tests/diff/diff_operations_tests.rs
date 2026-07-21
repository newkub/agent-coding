//! Diff operations tests

use agent_tui::modules::diff::domain::models::{DiffFilter, DiffHunk, HunkStatus};
use agent_tui::modules::diff::domain::operations::*;

#[test]
fn test_diff_filter_default() {
    let filter = DiffFilter::default();
    assert!(matches!(filter, DiffFilter::All));
}

#[test]
fn test_parse_unified_diff_basic() {
    let diff = "@@ -1,3 +1,4 @@\n line1\n-removed\n+added\n line3";
    let hunks = parse_unified_diff(diff);
    assert!(!hunks.is_empty());
}

#[test]
fn test_parse_unified_diff_empty() {
    let hunks = parse_unified_diff("");
    assert!(hunks.is_empty());
}

#[test]
fn test_parse_unified_diff_single_line() {
    let diff = "@@ -1 +1 @@\n context";
    let hunks = parse_unified_diff(diff);
    assert_eq!(hunks.len(), 1);
}

#[test]
fn test_all_hunks_approved() {
    let mut hunk1 = DiffHunk::create(
        uuid::Uuid::new_v4().to_string(),
        "@@ @@".to_string(),
        1,
        1,
        1,
        1,
        vec![],
    );
    let mut hunk2 = DiffHunk::create(
        uuid::Uuid::new_v4().to_string(),
        "@@ @@".to_string(),
        1,
        1,
        1,
        1,
        vec![],
    );
    hunk1.approve();
    hunk2.approve();

    assert!(all_hunks_approved(&[hunk1, hunk2]));
}

#[test]
fn test_all_hunks_approved_empty() {
    let hunks: Vec<DiffHunk> = vec![];
    assert!(!all_hunks_approved(&hunks));
}

#[test]
fn test_all_hunks_not_all_approved() {
    let mut hunk1 = DiffHunk::create(
        uuid::Uuid::new_v4().to_string(),
        "@@ @@".to_string(),
        1,
        1,
        1,
        1,
        vec![],
    );
    let mut hunk2 = DiffHunk::create(
        uuid::Uuid::new_v4().to_string(),
        "@@ @@".to_string(),
        1,
        1,
        1,
        1,
        vec![],
    );
    hunk1.approve();
    // hunk2 is pending

    assert!(!all_hunks_approved(&[hunk1, hunk2]));
}

#[test]
fn test_any_hunk_rejected() {
    let mut hunk1 = DiffHunk::create(
        uuid::Uuid::new_v4().to_string(),
        "@@ @@".to_string(),
        1,
        1,
        1,
        1,
        vec![],
    );
    let mut hunk2 = DiffHunk::create(
        uuid::Uuid::new_v4().to_string(),
        "@@ @@".to_string(),
        1,
        1,
        1,
        1,
        vec![],
    );
    hunk2.reject();

    assert!(any_hunk_rejected(&[hunk1, hunk2]));
}

#[test]
fn test_any_hunk_rejected_none() {
    let mut hunk1 = DiffHunk::create(
        uuid::Uuid::new_v4().to_string(),
        "@@ @@".to_string(),
        1,
        1,
        1,
        1,
        vec![],
    );
    hunk1.approve();

    assert!(!any_hunk_rejected(&[hunk1]));
}
