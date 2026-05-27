use ratatui::style::Color;
use std::sync::Mutex;
use super::theme::Theme;

// Legacy color constants for backward compatibility
// These are deprecated - use Theme instead
#[deprecated(note = "Use Theme::colors instead")]
pub(crate) const ACCENT: Color = Color::Cyan;
#[deprecated(note = "Use Theme::colors instead")]
pub(crate) const ACCENT_GREEN: Color = Color::Green;
#[deprecated(note = "Use Theme::colors instead")]
pub(crate) const ACCENT_YELLOW: Color = Color::Yellow;
#[deprecated(note = "Use Theme::colors instead")]
pub(crate) const ACCENT_RED: Color = Color::Red;

#[deprecated(note = "Use Theme::colors instead")]
pub(crate) const TEXT: Color = Color::White;
#[deprecated(note = "Use Theme::colors instead")]
pub(crate) const TEXT_BRIGHT: Color = Color::Rgb(220, 220, 220);
#[deprecated(note = "Use Theme::colors instead")]
pub(crate) const TEXT_DIM: Color = Color::Rgb(150, 150, 150);

#[deprecated(note = "Use Theme::colors instead")]
pub(crate) const BG: Color = Color::Black;
#[deprecated(note = "Use Theme::colors instead")]
pub(crate) const BG_LIGHT: Color = Color::Rgb(30, 30, 30);

// Get current theme (thread-safe singleton pattern)
static CURRENT_THEME: Mutex<Option<Theme>> = Mutex::new(None);

pub fn set_theme(theme: Theme) {
    let mut current = CURRENT_THEME.lock().unwrap();
    *current = Some(theme);
}

pub fn get_theme() -> Theme {
    let current = CURRENT_THEME.lock().unwrap();
    current.clone().unwrap_or_default()
}

pub fn with_theme<F, R>(f: F) -> R
where
    F: FnOnce(&Theme) -> R,
{
    let theme = get_theme();
    f(&theme)
}
