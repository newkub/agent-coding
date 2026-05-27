// Pure utility functions for shared kernel

/// Utility: Format text with width limit
pub(crate) fn format_text_width(text: &str, width: usize) -> String {
    if text.len() <= width {
        text.to_string()
    } else {
        format!("{}...", &text[..width.saturating_sub(3)])
    }
}

/// Utility: Truncate string to max length
pub(crate) fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        s[..max_len].to_string()
    }
}
