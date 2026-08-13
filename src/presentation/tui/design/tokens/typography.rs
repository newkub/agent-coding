/// Typography tokens for TUI
/// TUI only supports monospace fonts
#[derive(Debug, Clone, Copy)]
pub(crate) struct Typography {
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
