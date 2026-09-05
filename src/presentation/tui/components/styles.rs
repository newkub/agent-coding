#![allow(dead_code)]
use super::theme::Theme;
use ratatui::style::Color;
use ratatui_ui::Theme as RatatuiTheme;
use std::sync::Mutex;

// Get current theme (thread-safe singleton pattern)
static CURRENT_THEME: Mutex<Option<Theme>> = Mutex::new(None);

pub(crate) fn set_theme(theme: Theme) {
    let mut current = CURRENT_THEME.lock().unwrap();
    *current = Some(theme);
}

pub(crate) fn get_theme() -> Theme {
    let current = CURRENT_THEME.lock().unwrap();
    current.clone().unwrap_or_default()
}

pub(crate) fn with_theme<F, R>(f: F) -> R
where
    F: FnOnce(&Theme) -> R,
{
    let theme = get_theme();
    f(&theme)
}

// Theme color accessors - resolve colors from the active theme at call time.
pub(crate) fn accent() -> Color {
    get_theme().colors.accent
}
pub(crate) fn accent_green() -> Color {
    get_theme().colors.success
}
pub(crate) fn accent_yellow() -> Color {
    get_theme().colors.warning
}
pub(crate) fn accent_red() -> Color {
    get_theme().colors.error
}
pub(crate) fn text() -> Color {
    get_theme().colors.text
}
pub(crate) fn text_bright() -> Color {
    get_theme().colors.text_bright
}
pub(crate) fn text_dim() -> Color {
    get_theme().colors.text_dim
}
pub(crate) fn bg() -> Color {
    get_theme().colors.background
}
pub(crate) fn bg_light() -> Color {
    get_theme().colors.background_light
}

/// Convert the active app theme to the shared `ratatui-ui` theme.
pub(crate) fn rt_theme() -> RatatuiTheme {
    let t = get_theme().colors;
    RatatuiTheme {
        primary: t.accent,
        success: t.success,
        warning: t.warning,
        error: t.error,
        info: t.info,
        muted: t.text_dim,
        border: t.border,
        bg: t.background,
        fg: t.text,
    }
}
