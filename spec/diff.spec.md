# Diff Domain Tests

## FileChange
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_file_change_new | Creates file change with path and type | `FileChange::new("test.rs", ChangeType::Modified)` |
| ✅ | test_file_change_with_hunks | Can add hunks to file change | `change.add_hunk(hunk)` |
| ✅ | test_file_change_serialization | File change serializes/deserializes correctly | `serde_json::to_string(&change)` |

## DiffHunk
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_diff_hunk_new | Creates hunk with pending status | `DiffHunk::new(1, 10, vec![])` |
| ✅ | test_diff_hunk_approve | Can approve hunk | `hunk.approve()` |
| ✅ | test_diff_hunk_reject | Can reject hunk | `hunk.reject()` |
| ✅ | test_diff_hunk_is_pending | Checks if hunk is pending | `hunk.is_pending()` |

## DiffLine
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_diff_line_context | Creates context line | `DiffLine::context("original")` |
| ✅ | test_diff_line_addition | Creates addition line | `DiffLine::addition("new line")` |
| ✅ | test_diff_line_deletion | Creates deletion line | `DiffLine::deletion("old line")` |

## DiffReview
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_diff_review_new | Creates review with file changes | `DiffReview::new(vec![change])` |
| ✅ | test_diff_review_current_file | Gets current file | `review.current_file()` |
| ✅ | test_diff_review_empty | Empty review has no current file/hunk | `DiffReview::empty()` |
| ✅ | test_diff_review_next_hunk | Can navigate to next hunk | `review.next_hunk()` |
| ✅ | test_diff_review_prev_hunk | Can navigate to previous hunk | `review.prev_hunk()` |
| ✅ | test_diff_review_approve_current_hunk | Can approve current hunk | `review.approve_current_hunk()` |
| ✅ | test_diff_review_reject_current_hunk | Can reject current hunk | `review.reject_current_hunk()` |
| ✅ | test_diff_review_approved_count | Counts approved hunks | `review.approved_count()` |
| ✅ | test_diff_review_pending_count | Counts pending hunks | `review.pending_count()` |
| ✅ | test_diff_review_serialization | Review serializes/deserializes correctly | `serde_json::to_string(&review)` |

## DiffFilter
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_diff_filter_default | Default filter is All | `DiffFilter::default()` |

## Diff Operations
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_parse_unified_diff_basic | Parses basic unified diff | `parse_unified_diff(diff_str)` |
| ✅ | test_parse_unified_diff_empty | Empty diff returns empty hunks | `parse_unified_diff("")` |
| ✅ | test_parse_unified_diff_single_line | Parses single line diff | `parse_unified_diff("@@ -1 +1 @@")` |
| ✅ | test_all_hunks_approved | Checks if all hunks approved | `all_hunks_approved(&review)` |
| ✅ | test_all_hunks_approved_empty | Empty hunks return false | `all_hunks_approved(&empty_review)` |
| ✅ | test_all_hunks_not_all_approved | Returns false if not all approved | `all_hunks_approved(&partial_review)` |
| ✅ | test_any_hunk_rejected | Checks if any hunk rejected | `any_hunk_rejected(&review)` |
| ✅ | test_any_hunk_rejected_none | Returns false if none rejected | `any_hunk_rejected(&approved_review)` |

## Variants
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_change_type_variants | All change type variants match | `assert!(matches!(ChangeType::Added, ChangeType::Added))` |
| ✅ | test_diff_line_type_variants | All line type variants match | `assert!(matches!(DiffLineType::Context, DiffLineType::Context))` |
| ✅ | test_hunk_status_variants | All status variants match | `assert!(matches!(HunkStatus::Pending, HunkStatus::Pending))` |
