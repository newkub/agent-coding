// Design Tokens for TUI (Terminal User Interface)
// Follows design system principles for consistency and accessibility

use ratatui::style::{Color, Style};

/// Color palette for TUI (16 terminal colors)
/// Using ANSI color codes for maximum compatibility
#[derive(Debug, Clone, Copy)]
pub struct ColorPalette {
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

/// Typography tokens for TUI
/// TUI only supports monospace fonts
#[derive(Debug, Clone, Copy)]
pub struct Typography {
    pub font_family: &'static str,
    pub font_size: u8,
    pub line_height: u8,
}

impl Default for Typography {
    fn default() -> Self {
        Self {
            font_family: "monospace",
            font_size: 12,
            line_height: 1,
        }
    }
}

impl Typography {
    /// Default typography
    pub(crate) const fn default() -> Self {
        Self {
            font_family: "monospace",
            font_size: 12,
            line_height: 1,
        }
    }
    
    /// Large typography (for accessibility)
    pub(crate) const fn large() -> Self {
        Self {
            font_family: "monospace",
            font_size: 14,
            line_height: 1,
        }
    }
    
    /// Small typography (for dense information)
    pub(crate) const fn small() -> Self {
        Self {
            font_family: "monospace",
            font_size: 10,
            line_height: 1,
        }
    }
}

/// Spacing tokens for TUI
/// Using character-based spacing
#[derive(Debug, Clone, Copy)]
pub struct Spacing {
    pub xs: u16,  // 1 character
    pub sm: u16,  // 2 characters
    pub md: u16,  // 4 characters
    pub lg: u16,  // 8 characters
    pub xl: u16,  // 16 characters
}

impl Default for Spacing {
    fn default() -> Self {
        Self {
            xs: 1,
            sm: 2,
            md: 4,
            lg: 8,
            xl: 16,
        }
    }
}

impl Spacing {
    /// Default spacing
    pub(crate) const fn default() -> Self {
        Self {
            xs: 1,
            sm: 2,
            md: 4,
            lg: 8,
            xl: 16,
        }
    }
    
    /// Compact spacing (for dense layouts)
    pub(crate) const fn compact() -> Self {
        Self {
            xs: 0,
            sm: 1,
            md: 2,
            lg: 4,
            xl: 8,
        }
    }
    
    /// Comfortable spacing (for accessibility)
    pub(crate) const fn comfortable() -> Self {
        Self {
            xs: 1,
            sm: 3,
            md: 6,
            lg: 12,
            xl: 24,
        }
    }
}

/// Design tokens container
#[derive(Debug, Clone)]
pub struct DesignTokens {
    pub colors: ColorPalette,
    pub typography: Typography,
    pub spacing: Spacing,
}

impl Default for DesignTokens {
    fn default() -> Self {
        Self {
            colors: ColorPalette::default(),
            typography: Typography::default(),
            spacing: Spacing::default(),
        }
    }
}

impl DesignTokens {
    /// Create default design tokens
    pub(crate) fn default() -> Self {
        Self {
            colors: ColorPalette::default(),
            typography: Typography::default(),
            spacing: Spacing::default(),
        }
    }
    
    /// Create high contrast design tokens (for accessibility)
    pub(crate) const fn high_contrast() -> Self {
        Self {
            colors: ColorPalette::high_contrast_theme(),
            typography: Typography::large(),
            spacing: Spacing::comfortable(),
        }
    }
    
    /// Create dark theme design tokens
    pub(crate) const fn dark_theme() -> Self {
        Self {
            colors: ColorPalette::dark_theme(),
            typography: Typography::default(),
            spacing: Spacing::default(),
        }
    }
    
    /// Get style for primary text
    pub(crate) fn primary_text(&self) -> Style {
        Style::default().fg(self.colors.primary)
    }
    
    /// Get style for secondary text
    pub(crate) fn secondary_text(&self) -> Style {
        Style::default().fg(self.colors.secondary)
    }
    
    /// Get style for muted text
    pub(crate) fn muted_text(&self) -> Style {
        Style::default().fg(self.colors.muted)
    }
    
    /// Get style for success text
    pub(crate) fn success_text(&self) -> Style {
        Style::default().fg(self.colors.success)
    }
    
    /// Get style for warning text
    pub(crate) fn warning_text(&self) -> Style {
        Style::default().fg(self.colors.warning)
    }
    
    /// Get style for error text
    pub(crate) fn error_text(&self) -> Style {
        Style::default().fg(self.colors.error)
    }
    
    /// Get style for info text
    pub(crate) fn info_text(&self) -> Style {
        Style::default().fg(self.colors.info)
    }
    
    /// Get style for border
    pub(crate) fn border(&self) -> Style {
        Style::default().fg(self.colors.border)
    }
    
    /// Get style for background
    pub(crate) fn background(&self) -> Style {
        Style::default().bg(self.colors.background)
    }
    
    /// Get style for high contrast foreground
    pub(crate) fn high_contrast_fg(&self) -> Style {
        Style::default().fg(self.colors.high_contrast_fg)
    }
    
    /// Get style for high contrast background
    pub(crate) fn high_contrast_bg(&self) -> Style {
        Style::default().bg(self.colors.high_contrast_bg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
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
}
