use ratatui::style::Color;

/// Theme configuration for TUI
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Theme {
    pub name: String,
    pub colors: ColorPalette,
    pub spacing: Spacing,
}

/// Color palette for the theme
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColorPalette {
    // Primary colors
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,

    // Text colors
    pub text: Color,
    pub text_bright: Color,
    pub text_dim: Color,
    pub text_inverse: Color,

    // Background colors
    pub background: Color,
    pub background_light: Color,
    pub background_dark: Color,

    // Status colors
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub info: Color,

    // Border colors
    pub border: Color,
    pub border_active: Color,
    pub border_inactive: Color,

    // UI element colors
    pub tab_active: Color,
    pub tab_inactive: Color,
    pub tab_highlight: Color,

    pub status_bar_bg: Color,
    pub status_bar_fg: Color,
}

/// Spacing configuration
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spacing {
    pub margin: u16,
    pub padding: u16,
    pub gap: u16,
}

impl Default for Theme {
    fn default() -> Self {
        Self::modern_dark()
    }
}

impl Theme {
    /// Modern dark theme (default)
    pub(crate) fn modern_dark() -> Self {
        Self {
            name: "modern_dark".to_string(),
            colors: ColorPalette {
                // Primary colors - modern blue-purple gradient
                primary: Color::Rgb(96, 165, 250),   // Blue-500
                secondary: Color::Rgb(139, 92, 246), // Purple-500
                accent: Color::Rgb(236, 72, 153),    // Pink-500

                // Text colors
                text: Color::Rgb(243, 244, 246), // Gray-100
                text_bright: Color::Rgb(255, 255, 255),
                text_dim: Color::Rgb(156, 163, 175),  // Gray-400
                text_inverse: Color::Rgb(17, 24, 39), // Gray-900

                // Background colors
                background: Color::Rgb(17, 24, 39), // Gray-900
                background_light: Color::Rgb(31, 41, 55), // Gray-800
                background_dark: Color::Rgb(10, 10, 20),

                // Status colors
                success: Color::Rgb(34, 197, 94), // Green-500
                warning: Color::Rgb(234, 179, 8), // Yellow-500
                error: Color::Rgb(239, 68, 68),   // Red-500
                info: Color::Rgb(59, 130, 246),   // Blue-500

                // Border colors
                border: Color::Rgb(75, 85, 99),          // Gray-600
                border_active: Color::Rgb(96, 165, 250), // Blue-500
                border_inactive: Color::Rgb(55, 65, 81), // Gray-700

                // UI element colors
                tab_active: Color::Rgb(96, 165, 250), // Blue-500
                tab_inactive: Color::Rgb(107, 114, 128), // Gray-500
                tab_highlight: Color::Rgb(251, 191, 36), // Amber-400

                status_bar_bg: Color::Rgb(30, 41, 59), // Gray-800
                status_bar_fg: Color::Rgb(243, 244, 246), // Gray-100
            },
            spacing: Spacing {
                margin: 1,
                padding: 1,
                gap: 1,
            },
        }
    }

    /// Classic dark theme (original colors)
    pub(crate) fn classic_dark() -> Self {
        Self {
            name: "classic_dark".to_string(),
            colors: ColorPalette {
                primary: Color::Cyan,
                secondary: Color::Green,
                accent: Color::Yellow,

                text: Color::White,
                text_bright: Color::Rgb(220, 220, 220),
                text_dim: Color::Rgb(150, 150, 150),
                text_inverse: Color::Black,

                background: Color::Black,
                background_light: Color::Rgb(30, 30, 30),
                background_dark: Color::Black,

                success: Color::Green,
                warning: Color::Yellow,
                error: Color::Red,
                info: Color::Cyan,

                border: Color::Cyan,
                border_active: Color::Yellow,
                border_inactive: Color::Cyan,

                tab_active: Color::Cyan,
                tab_inactive: Color::Rgb(100, 100, 100),
                tab_highlight: Color::Yellow,

                status_bar_bg: Color::Blue,
                status_bar_fg: Color::White,
            },
            spacing: Spacing {
                margin: 1,
                padding: 1,
                gap: 1,
            },
        }
    }

    /// Light theme
    pub(crate) fn light() -> Self {
        Self {
            name: "light".to_string(),
            colors: ColorPalette {
                primary: Color::Rgb(37, 99, 235),    // Blue-600
                secondary: Color::Rgb(124, 58, 237), // Purple-600
                accent: Color::Rgb(219, 39, 119),    // Pink-600

                text: Color::Rgb(17, 24, 39), // Gray-900
                text_bright: Color::Rgb(0, 0, 0),
                text_dim: Color::Rgb(107, 114, 128), // Gray-500
                text_inverse: Color::Rgb(255, 255, 255),

                background: Color::Rgb(255, 255, 255),
                background_light: Color::Rgb(243, 244, 246), // Gray-100
                background_dark: Color::Rgb(229, 231, 235),  // Gray-200

                success: Color::Rgb(22, 163, 74), // Green-600
                warning: Color::Rgb(202, 138, 4), // Yellow-600
                error: Color::Rgb(220, 38, 38),   // Red-600
                info: Color::Rgb(37, 99, 235),    // Blue-600

                border: Color::Rgb(209, 213, 219),      // Gray-300
                border_active: Color::Rgb(37, 99, 235), // Blue-600
                border_inactive: Color::Rgb(229, 231, 235), // Gray-200

                tab_active: Color::Rgb(37, 99, 235), // Blue-600
                tab_inactive: Color::Rgb(156, 163, 175), // Gray-400
                tab_highlight: Color::Rgb(245, 158, 11), // Amber-500

                status_bar_bg: Color::Rgb(243, 244, 246), // Gray-100
                status_bar_fg: Color::Rgb(17, 24, 39),    // Gray-900
            },
            spacing: Spacing {
                margin: 1,
                padding: 1,
                gap: 1,
            },
        }
    }

    /// Dracula theme
    pub(crate) fn dracula() -> Self {
        Self {
            name: "dracula".to_string(),
            colors: ColorPalette {
                primary: Color::Rgb(189, 147, 249),   // Purple
                secondary: Color::Rgb(139, 233, 253), // Cyan
                accent: Color::Rgb(255, 121, 198),    // Pink

                text: Color::Rgb(248, 248, 242), // Foreground
                text_bright: Color::Rgb(255, 255, 255),
                text_dim: Color::Rgb(98, 114, 164),   // Comment
                text_inverse: Color::Rgb(40, 42, 54), // Background

                background: Color::Rgb(40, 42, 54), // Background
                background_light: Color::Rgb(68, 71, 90), // Current Line
                background_dark: Color::Rgb(28, 30, 40),

                success: Color::Rgb(80, 250, 123),  // Green
                warning: Color::Rgb(241, 250, 140), // Yellow
                error: Color::Rgb(255, 85, 85),     // Red
                info: Color::Rgb(139, 233, 253),    // Cyan

                border: Color::Rgb(98, 114, 164),         // Comment
                border_active: Color::Rgb(189, 147, 249), // Purple
                border_inactive: Color::Rgb(68, 71, 90),  // Current Line

                tab_active: Color::Rgb(189, 147, 249), // Purple
                tab_inactive: Color::Rgb(98, 114, 164), // Comment
                tab_highlight: Color::Rgb(255, 121, 198), // Pink

                status_bar_bg: Color::Rgb(68, 71, 90), // Current Line
                status_bar_fg: Color::Rgb(248, 248, 242), // Foreground
            },
            spacing: Spacing {
                margin: 1,
                padding: 1,
                gap: 1,
            },
        }
    }

    /// Get theme by name
    pub(crate) fn from_name(name: &str) -> Self {
        match name.to_lowercase().as_str() {
            "classic" | "classic_dark" => Self::classic_dark(),
            "light" => Self::light(),
            "dracula" => Self::dracula(),
            _ => Self::modern_dark(),
        }
    }

    /// List available theme names
    pub(crate) fn available_themes() -> Vec<&'static str> {
        vec!["modern_dark", "classic_dark", "light", "dracula"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_default() {
        let theme = Theme::default();
        assert_eq!(theme.name, "modern_dark");
    }

    #[test]
    fn test_theme_from_name() {
        assert_eq!(Theme::from_name("modern_dark").name, "modern_dark");
        assert_eq!(Theme::from_name("classic").name, "classic_dark");
        assert_eq!(Theme::from_name("light").name, "light");
        assert_eq!(Theme::from_name("dracula").name, "dracula");
        assert_eq!(Theme::from_name("unknown").name, "modern_dark");
    }

    #[test]
    fn test_available_themes() {
        let themes = Theme::available_themes();
        assert!(themes.contains(&"modern_dark"));
        assert!(themes.contains(&"classic_dark"));
        assert!(themes.contains(&"light"));
        assert!(themes.contains(&"dracula"));
    }
}
