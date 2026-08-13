use ratatui::{
    style::Style,
    text::{Line, Span, Text},
};

use super::super::styles::*;

pub(super) fn get_global_shortcuts() -> Text<'static> {
    Text::from(vec![
        Line::from(vec![Span::styled(
            "🌍 GLOBAL",
            Style::default()
                .fg(accent())
                .add_modifier(ratatui::style::Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![
            Span::raw("  Tab         "),
            Span::styled("Next tab", Style::default().fg(text_dim())),
        ]),
        Line::from(vec![
            Span::raw("  Shift+Tab   "),
            Span::styled("Prev tab", Style::default().fg(text_dim())),
        ]),
        Line::from(vec![
            Span::raw("  → ←         "),
            Span::styled("Next/Prev column", Style::default().fg(text_dim())),
        ]),
        Line::from(vec![
            Span::raw("  f           "),
            Span::styled("Toggle focus", Style::default().fg(text_dim())),
        ]),
        Line::from(vec![
            Span::raw("  ? / Ctrl+H   "),
            Span::styled("Toggle help", Style::default().fg(text_dim())),
        ]),
        Line::from(vec![
            Span::raw("  q / Esc      "),
            Span::styled("Quit", Style::default().fg(text_dim())),
        ]),
        Line::from(vec![
            Span::raw("  Ctrl+K       "),
            Span::styled("Command palette", Style::default().fg(text_dim())),
        ]),
    ])
}
