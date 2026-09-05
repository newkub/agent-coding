#![allow(dead_code)]
mod color_palette;
mod spacing;
mod typography;

#[cfg(test)]
mod tests;

pub(crate) use color_palette::ColorPalette;
pub(crate) use spacing::Spacing;
pub(crate) use typography::Typography;

use ratatui::style::Style;

/// Design tokens container
#[derive(Debug, Clone)]
pub(crate) struct DesignTokens {
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
