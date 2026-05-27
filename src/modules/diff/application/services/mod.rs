use crate::modules::diff::domain::models::{DiffReview, DiffFilter};

/// Service: Filter diff review
pub(crate) fn filter_diff_review(review: &DiffReview, filter: DiffFilter) -> Vec<String> {
    let mut visible = Vec::new();
    
    for (file_idx, file) in review.files.iter().enumerate() {
        let file_visible = match filter {
            DiffFilter::All => true,
            DiffFilter::Pending => file.hunks.iter().any(|h| h.is_pending()),
            DiffFilter::Approved => file.hunks.iter().any(|h| !h.is_pending()),
            DiffFilter::Rejected => file.hunks.iter().any(|h| !h.is_pending()),
        };
        
        if file_visible {
            visible.push(format!("{} ({}/{})", file.path, file_idx + 1, review.files.len()));
        }
    }
    
    visible
}

/// Service: Get diff statistics
pub(crate) fn get_diff_stats(review: &DiffReview) -> DiffStats {
    let mut total_hunks = 0;
    let mut approved = 0;
    let mut pending = 0;
    let mut rejected = 0;
    let mut additions = 0;
    let mut deletions = 0;

    for file in &review.files {
        for hunk in &file.hunks {
            total_hunks += 1;
            match hunk.status {
                crate::modules::diff::domain::models::HunkStatus::Approved => approved += 1,
                crate::modules::diff::domain::models::HunkStatus::Rejected => rejected += 1,
                crate::modules::diff::domain::models::HunkStatus::Pending => pending += 1,
            }
            
            for line in &hunk.lines {
                match line.line_type {
                    crate::modules::diff::domain::models::DiffLineType::Addition => additions += 1,
                    crate::modules::diff::domain::models::DiffLineType::Deletion => deletions += 1,
                    _ => {}
                }
            }
        }
    }

    DiffStats {
        file_count: review.files.len(),
        total_hunks,
        approved,
        pending,
        rejected,
        additions,
        deletions,
    }
}

#[derive(Debug, Clone)]
pub(crate) struct DiffStats {
    pub file_count: usize,
    pub total_hunks: usize,
    pub approved: usize,
    pub pending: usize,
    pub rejected: usize,
    pub additions: usize,
    pub deletions: usize,
}