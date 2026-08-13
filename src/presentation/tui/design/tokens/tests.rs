use super::*;
use ratatui::style::Color;

#[test]
fn test_default_tokens() {
    let tokens = DesignTokens::default();
    assert_eq!(tokens.spacing.xs, 1);
    assert_eq!(tokens.spacing.sm, 2);
}

#[test]
fn test_high_contrast_tokens() {
    let tokens = DesignTokens::high_contrast();
    assert_eq!(tokens.colors.background, Color::Black);
    assert_eq!(tokens.colors.foreground, Color::White);
    assert_eq!(tokens.typography.font_size, 14); // Larger for accessibility
}

#[test]
fn test_compact_spacing() {
    let spacing = Spacing::compact();
    assert_eq!(spacing.xs, 0);
    assert_eq!(spacing.sm, 1);
}

#[test]
fn test_comfortable_spacing() {
    let spacing = Spacing::comfortable();
    assert_eq!(spacing.sm, 3);
    assert_eq!(spacing.md, 6);
}
