use ratatui::{
    style::Style,
    text::{Line, Span, Text},
};

use super::super::styles::*;
use crate::shared::kernel::types::{Column, Tab};

pub(super) fn get_column_shortcuts(tab: Tab, column: Column) -> Text<'static> {
    match (tab, column) {
        (Tab::Git, Column::Left) => Text::from(vec![
            Line::from(vec![Span::styled(
                "📄 STATUS",
                Style::default()
                    .fg(accent())
                    .add_modifier(ratatui::style::Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![
                Span::raw("  s           "),
                Span::styled("Stage selected", Style::default().fg(text_dim())),
            ]),
            Line::from(vec![
                Span::raw("  u           "),
                Span::styled("Unstage selected", Style::default().fg(text_dim())),
            ]),
            Line::from(vec![
                Span::raw("  d           "),
                Span::styled("Discard changes", Style::default().fg(text_dim())),
            ]),
            Line::from(vec![
                Span::raw("  i           "),
                Span::styled("Stage hunk", Style::default().fg(text_dim())),
            ]),
        ]),
        (Tab::Git, Column::Center) => Text::from(vec![
            Line::from(vec![Span::styled(
                "📝 DIFF",
                Style::default()
                    .fg(accent())
                    .add_modifier(ratatui::style::Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![
                Span::raw("  ↑↓          "),
                Span::styled("Scroll diff", Style::default().fg(text_dim())),
            ]),
            Line::from(vec![
                Span::raw("  PageUp/Down "),
                Span::styled("Page scroll", Style::default().fg(text_dim())),
            ]),
            Line::from(vec![
                Span::raw("  Home/End    "),
                Span::styled("Jump to start/end", Style::default().fg(text_dim())),
            ]),
        ]),
        (Tab::Git, Column::Right) => Text::from(vec![
            Line::from(vec![Span::styled(
                "📜 HISTORY",
                Style::default()
                    .fg(accent())
                    .add_modifier(ratatui::style::Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![
                Span::raw("  Enter       "),
                Span::styled("View commit", Style::default().fg(text_dim())),
            ]),
            Line::from(vec![
                Span::raw("  b           "),
                Span::styled("Checkout commit", Style::default().fg(text_dim())),
            ]),
            Line::from(vec![
                Span::raw("  /           "),
                Span::styled("Search commits", Style::default().fg(text_dim())),
            ]),
        ]),
        (Tab::Agent, Column::Center) => Text::from(vec![
            Line::from(vec![Span::styled(
                "💬 CHAT",
                Style::default()
                    .fg(accent())
                    .add_modifier(ratatui::style::Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![
                Span::raw("  Enter       "),
                Span::styled("Send message", Style::default().fg(text_dim())),
            ]),
            Line::from(vec![
                Span::raw("  Ctrl+G      "),
                Span::styled("Send (alt)", Style::default().fg(text_dim())),
            ]),
            Line::from(vec![
                Span::raw("  Tab         "),
                Span::styled("Auto-complete", Style::default().fg(text_dim())),
            ]),
            Line::from(vec![
                Span::raw("  Esc         "),
                Span::styled("Clear input", Style::default().fg(text_dim())),
            ]),
        ]),
        _ => Text::from(vec![
            Line::from(vec![Span::styled(
                "📍 COLUMN",
                Style::default()
                    .fg(accent())
                    .add_modifier(ratatui::style::Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![
                Span::raw("  ↑↓          "),
                Span::styled("Navigate", Style::default().fg(text_dim())),
            ]),
            Line::from(vec![
                Span::raw("  Enter       "),
                Span::styled("Select", Style::default().fg(text_dim())),
            ]),
            Line::from(vec![
                Span::raw("  Esc         "),
                Span::styled("Back", Style::default().fg(text_dim())),
            ]),
        ]),
    }
}
