// Design Tokens for TUI (Terminal User Interface)
#![allow(dead_code)]
// Follows design system principles for consistency and accessibility

use ratatui::style::Color;

/// Color palette for TUI (16 terminal colors)
/// Using ANSI color codes for maximum compatibility
#[derive(Debug, Clone, Copy)]
pub(crate) struct ColorPalette {
    // Primary colors
    pub primary: Color,
    pub primary_light: Color,
    pub primary_dark: Color,

    // Secondary colors
    pub secondary: Color,
    pub secondary_light: Color,
    pub secondary_dark: Color,

    // Semantic colors
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub info: Color,

    // Neutral colors
    pub background: Color,
    pub foreground: Color,
    pub border: Color,
    pub muted: Color,

    // High contrast colors (for accessibility)
    pub high_contrast_bg: Color,
    pub high_contrast_fg: Color,
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self::default_theme()
    }
}

impl ColorPalette {
    /// Default theme (balanced colors)
    pub(crate) const fn default_theme() -> Self {
        Self {
            primary: Color::Blue,
            primary_light: Color::LightBlue,
            primary_dark: Color::DarkGray,
            secondary: Color::Cyan,
            secondary_light: Color::LightCyan,
            secondary_dark: Color::Gray,
            success: Color::Green,
            warning: Color::Yellow,
            error: Color::Red,
            info: Color::Blue,
            background: Color::Reset,
            foreground: Color::Reset,
            border: Color::Gray,
            muted: Color::DarkGray,
            high_contrast_bg: Color::Black,
            high_contrast_fg: Color::White,
        }
    }

    /// High contrast theme (WCAG AA compliant)
    pub(crate) const fn high_contrast_theme() -> Self {
        Self {
            primary: Color::Cyan,
            primary_light: Color::White,
            primary_dark: Color::Blue,
            secondary: Color::Green,
            secondary_light: Color::White,
            secondary_dark: Color::DarkGray,
            success: Color::Green,
            warning: Color::Yellow,
            error: Color::Red,
            info: Color::Cyan,
            background: Color::Black,
            foreground: Color::White,
            border: Color::White,
            muted: Color::Gray,
            high_contrast_bg: Color::Black,
            high_contrast_fg: Color::White,
        }
    }

    /// Dark theme (reduced eye strain)
    pub(crate) const fn dark_theme() -> Self {
        Self {
            primary: Color::LightBlue,
            primary_light: Color::Cyan,
            primary_dark: Color::Blue,
            secondary: Color::LightCyan,
            secondary_light: Color::White,
            secondary_dark: Color::Cyan,
            success: Color::LightGreen,
            warning: Color::LightYellow,
            error: Color::LightRed,
            info: Color::LightBlue,
            background: Color::Black,
            foreground: Color::White,
            border: Color::DarkGray,
            muted: Color::Gray,
            high_contrast_bg: Color::Black,
            high_contrast_fg: Color::White,
        }
    }
}
