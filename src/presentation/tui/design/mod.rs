// Design system module for TUI
#![allow(dead_code)]
// Provides design tokens, themes, and accessibility features

pub(crate) mod tokens;

pub(crate) use tokens::DesignTokens;

/// Theme variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum Theme {
    #[default]
    Default,
    HighContrast,
    Dark,
}

impl Theme {
    /// Get design tokens for this theme
    pub(crate) fn tokens(self) -> DesignTokens {
        match self {
            Self::Default => DesignTokens::default(),
            Self::HighContrast => DesignTokens::high_contrast(),
            Self::Dark => DesignTokens::dark_theme(),
        }
    }

    /// Get theme name
    pub(crate) const fn name(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::HighContrast => "high-contrast",
            Self::Dark => "dark",
        }
    }

    /// Parse theme from string
    pub(crate) fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "default" => Some(Self::Default),
            "high-contrast" | "high_contrast" => Some(Self::HighContrast),
            "dark" => Some(Self::Dark),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::Color;

    #[test]
    fn test_theme_parsing() {
        assert_eq!(Theme::from_str("default"), Some(Theme::Default));
        assert_eq!(Theme::from_str("high-contrast"), Some(Theme::HighContrast));
        assert_eq!(Theme::from_str("dark"), Some(Theme::Dark));
        assert_eq!(Theme::from_str("invalid"), None);
    }

    #[test]
    fn test_theme_tokens() {
        let theme = Theme::HighContrast;
        let tokens = theme.tokens();
        assert_eq!(tokens.colors.background, Color::Black);
        assert_eq!(tokens.colors.foreground, Color::White);
    }
}
