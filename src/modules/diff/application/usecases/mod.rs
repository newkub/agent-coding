use crate::modules::diff::domain::events::DiffEvent;
use crate::modules::diff::domain::models::*;
use crate::modules::diff::ports::{DiffEventPublisher, DiffParser};
use crate::shared::kernel::result::AppResult;

/// Use case: Parse and review diff
pub(crate) async fn review_diff<P>(parser: &P, diff_text: &str) -> AppResult<DiffReview>
where
    P: DiffParser,
{
    parser.parse(diff_text).await
}

/// Use case: Approve current hunk
pub(crate) async fn approve_hunk<P>(review: &mut DiffReview, publisher: &P) -> AppResult<()>
where
    P: DiffEventPublisher,
{
    let file = review.current_file().cloned();
    let hunk_id = review.current_hunk().map(|h| h.id.clone());

    review.approve_current_hunk();

    if let (Some(f), Some(h)) = (file, hunk_id) {
        publisher
            .publish(DiffEvent::HunkApproved {
                file_path: f.path,
                hunk_id: h,
            })
            .await?;
    }

    // Move to next hunk
    review.next_hunk();
    Ok(())
}

/// Use case: Reject current hunk
pub(crate) async fn reject_hunk<P>(review: &mut DiffReview, publisher: &P) -> AppResult<()>
where
    P: DiffEventPublisher,
{
    let file = review.current_file().cloned();
    let hunk_id = review.current_hunk().map(|h| h.id.clone());

    review.reject_current_hunk();

    if let (Some(f), Some(h)) = (file, hunk_id) {
        publisher
            .publish(DiffEvent::HunkRejected {
                file_path: f.path,
                hunk_id: h,
            })
            .await?;
    }

    // Move to next hunk
    review.next_hunk();
    Ok(())
}

/// Use case: Apply approved changes
pub(crate) async fn apply_approved_changes(review: &DiffReview) -> AppResult<ApplyResult> {
    let mut applied = 0;
    let mut skipped = 0;

    for file in &review.files {
        let approved_count = file
            .hunks
            .iter()
            .filter(|h| h.status == crate::modules::diff::domain::models::HunkStatus::Approved)
            .count();

        if approved_count == file.hunks.len() {
            // All hunks approved, can apply
            applied += 1;
        } else if approved_count > 0 {
            // Partial approval, need user action
            skipped += 1;
        }
    }

    Ok(ApplyResult { applied, skipped })
}

#[derive(Debug)]
pub(crate) struct ApplyResult {
    pub applied: usize,
    pub skipped: usize,
}

/// Use case: Navigate diff
pub(crate) fn navigate_diff(review: &mut DiffReview, direction: NavigationDirection) -> bool {
    match direction {
        NavigationDirection::Next => review.next_hunk(),
        NavigationDirection::Previous => review.prev_hunk(),
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum NavigationDirection {
    Next,
    Previous,
}
