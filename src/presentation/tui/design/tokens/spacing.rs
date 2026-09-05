#![allow(dead_code)]

/// Spacing tokens for TUI
/// Using character-based spacing
#[derive(Debug, Clone, Copy)]
pub(crate) struct Spacing {
    pub xs: u16, // 1 character
    pub sm: u16, // 2 characters
    pub md: u16, // 4 characters
    pub lg: u16, // 8 characters
    pub xl: u16, // 16 characters
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
