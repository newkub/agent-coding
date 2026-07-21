use super::models::{DiffHunk, DiffLine, DiffLineType};

/// Pure domain operation: Parse unified diff format
pub fn parse_unified_diff(diff_text: &str) -> Vec<DiffHunk> {
    let mut hunks = Vec::new();
    let lines: Vec<&str> = diff_text.lines().collect();
    let mut current_lines: Vec<DiffLine> = Vec::new();
    let mut hunk_start = 0;
    let mut old_start = 0u32;
    let mut old_count = 0u32;
    let mut new_start = 0u32;
    let mut new_count = 0u32;
    let mut in_hunk = false;
    let mut old_line = 0u32;
    let mut new_line = 0u32;

    for (i, line) in lines.iter().enumerate() {
        if line.starts_with("@@") {
            if in_hunk && !current_lines.is_empty() {
                let id = uuid::Uuid::new_v4().to_string();
                hunks.push(DiffHunk::create(
                    id,
                    lines[hunk_start..i].join("\n"),
                    old_start,
                    old_count,
                    new_start,
                    new_count,
                    std::mem::take(&mut current_lines),
                ));
            }

            // Parse hunk header: @@ -old_start,old_count +new_start,new_count @@
            if let Some(parsed) = parse_hunk_header(line) {
                old_start = parsed.0;
                old_count = parsed.1;
                new_start = parsed.2;
                new_count = parsed.3;
            }

            old_line = old_start;
            new_line = new_start;
            hunk_start = i;
            in_hunk = true;
            current_lines.push(DiffLine {
                line_type: DiffLineType::Header,
                content: line.to_string(),
                old_line_num: None,
                new_line_num: None,
            });
        } else if in_hunk {
            let (line_type, is_old, is_new) = match line.chars().next() {
                Some('+') => (DiffLineType::Addition, false, true),
                Some('-') => (DiffLineType::Deletion, true, false),
                Some(' ') | None => (DiffLineType::Context, true, true),
                _ => (DiffLineType::Context, false, false),
            };

            let content = line
                .strip_prefix(|c: char| c == '+' || c == '-' || c == ' ')
                .unwrap_or(line)
                .to_string();

            current_lines.push(DiffLine {
                line_type,
                content,
                old_line_num: if is_old {
                    Some(old_line)
                } else {
                    old_line.checked_sub(1)
                },
                new_line_num: if is_new {
                    Some(new_line)
                } else {
                    new_line.checked_sub(1)
                },
            });

            if is_old {
                old_line += 1;
            }
            if is_new {
                new_line += 1;
            }
        }
    }

    // Push last hunk
    if in_hunk && !current_lines.is_empty() {
        let id = uuid::Uuid::new_v4().to_string();
        hunks.push(DiffHunk::create(
            id,
            lines[hunk_start..].join("\n"),
            old_start,
            old_count,
            new_start,
            new_count,
            current_lines,
        ));
    }

    hunks
}

fn parse_hunk_header(header: &str) -> Option<(u32, u32, u32, u32)> {
    // Format: @@ -old_start[,old_count] +new_start[,new_count] @@
    let parts: Vec<&str> = header.split_whitespace().collect();
    if parts.len() < 3 {
        return None;
    }

    let old_part = parts[1].trim_start_matches('-');
    let new_part = parts[2].trim_start_matches('+');

    let old_nums: Vec<u32> = old_part.split(',').filter_map(|s| s.parse().ok()).collect();
    let new_nums: Vec<u32> = new_part.split(',').filter_map(|s| s.parse().ok()).collect();

    Some((
        old_nums.first().copied().unwrap_or(1),
        old_nums.get(1).copied().unwrap_or(1),
        new_nums.first().copied().unwrap_or(1),
        new_nums.get(1).copied().unwrap_or(1),
    ))
}

/// Pure domain operation: Check if all hunks are approved
pub fn all_hunks_approved(hunks: &[DiffHunk]) -> bool {
    !hunks.is_empty()
        && hunks
            .iter()
            .all(|h| h.status == crate::modules::diff::domain::models::HunkStatus::Approved)
}

/// Pure domain operation: Check if any hunk is rejected
pub fn any_hunk_rejected(hunks: &[DiffHunk]) -> bool {
    hunks
        .iter()
        .any(|h| h.status == crate::modules::diff::domain::models::HunkStatus::Rejected)
}
