//! Diff Review tests

use agent_tui::modules::diff::domain::models::{FileChange, DiffHunk, ChangeType, HunkStatus};

#[test]
fn test_diff_review_new() {
    let fc = FileChange::new("test.rs".to_string(), ChangeType::Modified);
    let review = agent_tui::modules::diff::domain::models::DiffReview::new(vec![fc]);
    assert_eq!(review.current_file_index, 0);
    assert_eq!(review.current_hunk_index, 0);
}

#[test]
fn test_diff_review_current_file() {
    let fc = FileChange::new("test.rs".to_string(), ChangeType::Modified);
    let review = agent_tui::modules::diff::domain::models::DiffReview::new(vec![fc]);
    assert!(review.current_file().is_some());
    assert_eq!(review.current_file().unwrap().path, "test.rs");
}

#[test]
fn test_diff_review_empty() {
    let review = agent_tui::modules::diff::domain::models::DiffReview::new(vec![]);
    assert!(review.current_file().is_none());
    assert!(review.current_hunk().is_none());
}

#[test]
fn test_diff_review_next_hunk() {
    let mut hunk1 = DiffHunk::create(uuid::Uuid::new_v4().to_string(), "@@ -1 @@".to_string(), 1, 1, 1, 1, vec![]);
    let mut hunk2 = DiffHunk::create(uuid::Uuid::new_v4().to_string(), "@@ -5 @@".to_string(), 5, 1, 5, 1, vec![]);
    
    let mut fc = FileChange::new("test.rs".to_string(), ChangeType::Modified);
    fc.hunks.push(hunk1);
    fc.hunks.push(hunk2);
    
    let mut review = agent_tui::modules::diff::domain::models::DiffReview::new(vec![fc]);
    assert!(review.next_hunk());
    assert_eq!(review.current_hunk_index, 1);
}

#[test]
fn test_diff_review_prev_hunk() {
    let mut hunk1 = DiffHunk::create(uuid::Uuid::new_v4().to_string(), "@@ -1 @@".to_string(), 1, 1, 1, 1, vec![]);
    let mut hunk2 = DiffHunk::create(uuid::Uuid::new_v4().to_string(), "@@ -5 @@".to_string(), 5, 1, 5, 1, vec![]);
    
    let mut fc = FileChange::new("test.rs".to_string(), ChangeType::Modified);
    fc.hunks.push(hunk1);
    fc.hunks.push(hunk2);
    
    let mut review = agent_tui::modules::diff::domain::models::DiffReview::new(vec![fc]);
    review.current_hunk_index = 1;
    assert!(review.prev_hunk());
    assert_eq!(review.current_hunk_index, 0);
}

#[test]
fn test_diff_review_approve_current_hunk() {
    let mut hunk = DiffHunk::create(uuid::Uuid::new_v4().to_string(), "@@ -1 @@".to_string(), 1, 1, 1, 1, vec![]);
    let mut fc = FileChange::new("test.rs".to_string(), ChangeType::Modified);
    fc.hunks.push(hunk);
    
    let mut review = agent_tui::modules::diff::domain::models::DiffReview::new(vec![fc]);
    review.approve_current_hunk();
    assert_eq!(review.files[0].hunks[0].status, HunkStatus::Approved);
}

#[test]
fn test_diff_review_reject_current_hunk() {
    let mut hunk = DiffHunk::create(uuid::Uuid::new_v4().to_string(), "@@ -1 @@".to_string(), 1, 1, 1, 1, vec![]);
    let mut fc = FileChange::new("test.rs".to_string(), ChangeType::Modified);
    fc.hunks.push(hunk);
    
    let mut review = agent_tui::modules::diff::domain::models::DiffReview::new(vec![fc]);
    review.reject_current_hunk();
    assert_eq!(review.files[0].hunks[0].status, HunkStatus::Rejected);
}

#[test]
fn test_diff_review_approved_count() {
    let mut hunk1 = DiffHunk::create(uuid::Uuid::new_v4().to_string(), "@@ @@".to_string(), 1, 1, 1, 1, vec![]);
    hunk1.approve();
    let mut hunk2 = DiffHunk::create(uuid::Uuid::new_v4().to_string(), "@@ @@".to_string(), 1, 1, 1, 1, vec![]);
    
    let mut fc = FileChange::new("test.rs".to_string(), ChangeType::Modified);
    fc.hunks.push(hunk1);
    fc.hunks.push(hunk2);
    
    let review = agent_tui::modules::diff::domain::models::DiffReview::new(vec![fc]);
    assert_eq!(review.approved_count(), 1);
}

#[test]
fn test_diff_review_pending_count() {
    let mut hunk1 = DiffHunk::create(uuid::Uuid::new_v4().to_string(), "@@ @@".to_string(), 1, 1, 1, 1, vec![]);
    let mut hunk2 = DiffHunk::create(uuid::Uuid::new_v4().to_string(), "@@ @@".to_string(), 1, 1, 1, 1, vec![]);
    
    let mut fc = FileChange::new("test.rs".to_string(), ChangeType::Modified);
    fc.hunks.push(hunk1);
    fc.hunks.push(hunk2);
    
    let review = agent_tui::modules::diff::domain::models::DiffReview::new(vec![fc]);
    assert_eq!(review.pending_count(), 2);
}
