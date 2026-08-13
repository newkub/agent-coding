use ratatui::{
    style::Style,
    text::{Line, Span, Text},
};

use super::super::styles::*;
use crate::shared::kernel::types::Tab;

pub(super) fn get_tab_shortcuts(tab: Tab) -> Text<'static> {
    match tab {
        Tab::Agent => Text::from(vec![
            Line::from(vec![Span::styled(
                "🤖 AGENT",
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
                Span::raw("  Ctrl+N      "),
                Span::styled("New session", Style::default().fg(text_dim())),
            ]),
            Line::from(vec![
                Span::raw("  Ctrl+S      "),
                Span::styled("Save session", Style::default().fg(text_dim())),
            ]),
            Line::from(vec![
                Span::raw("  Ctrl+L      "),
                Span::styled("List sessions", Style::default().fg(text_dim())),
            ]),
            Line::from(vec![
                Span::raw("  ↑↓          "),
                Span::styled("Navigate history", Style::default().fg(text_dim())),
            ]),
        ]),
        Tab::Git => Text::from(vec![
            Line::from(vec![Span::styled(
                "📚 GIT",
                Style::default()
                    .fg(accent())
                    .add_modifier(ratatui::style::Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![
                Span::raw("  s           "),
                Span::styled("Stage file", Style::default().fg(text_dim())),
            ]),
            Line::from(vec![
                Span::raw("  u           "),
                Span::styled("Unstage file", Style::default().fg(text_dim())),
            ]),
            Line::from(vec![
                Span::raw("  c           "),
                Span::styled("Commit", Style::default().fg(text_dim())),
            ]),
            Line::from(vec![
                Span::raw("  p           "),
                Span::styled("Push", Style::default().fg(text_dim())),
            ]),
            Line::from(vec![
                Span::raw("  f           "),
                Span::styled("Fetch", Style::default().fg(text_dim())),
            ]),
            Line::from(vec![
                Span::raw("  b           "),
                Span::styled("Create branch", Style::default().fg(text_dim())),
            ]),
        ]),
        Tab::Files => Text::from(vec![
            Line::from(vec![Span::styled(
                "📁 FILES",
                Style::default()
                    .fg(accent())
                    .add_modifier(ratatui::style::Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![
                Span::raw("  Enter       "),
                Span::styled("Open file", Style::default().fg(text_dim())),
            ]),
            Line::from(vec![
                Span::raw("  e           "),
                Span::styled("Edit file", Style::default().fg(text_dim())),
            ]),
            Line::from(vec![
                Span::raw("  d           "),
                Span::styled("Delete file", Style::default().fg(text_dim())),
            ]),
            Line::from(vec![
                Span::raw("  r           "),
                Span::styled("Rename file", Style::default().fg(text_dim())),
            ]),
            Line::from(vec![
                Span::raw("  /           "),
                Span::styled("Search files", Style::default().fg(text_dim())),
            ]),
        ]),
        Tab::Terminal => Text::from(vec![
            Line::from(vec![Span::styled(
                "??? TERMINAL",
                Style::default()
                    .fg(accent())
                    .add_modifier(ratatui::style::Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![
                Span::raw("  Enter       "),
                Span::styled("Execute command", Style::default().fg(text_dim())),
            ]),
            Line::from(vec![
                Span::raw("  Ctrl+C      "),
                Span::styled("Cancel command", Style::default().fg(text_dim())),
            ]),
            Line::from(vec![
                Span::raw("  ↑↓          "),
                Span::styled("Command history", Style::default().fg(text_dim())),
            ]),
            Line::from(vec![
                Span::raw("  Ctrl+L      "),
                Span::styled("Clear output", Style::default().fg(text_dim())),
            ]),
        ]),
        _ => Text::from(vec![
            Line::from(vec![Span::styled(
                "📋 TAB",
                Style::default()
                    .fg(accent())
                    .add_modifier(ratatui::style::Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![
                Span::raw("  ↑↓          "),
                Span::styled("Navigate items", Style::default().fg(text_dim())),
            ]),
            Line::from(vec![
                Span::raw("  Enter       "),
                Span::styled("Select item", Style::default().fg(text_dim())),
            ]),
            Line::from(vec![
                Span::raw("  Esc         "),
                Span::styled("Go back", Style::default().fg(text_dim())),
            ]),
        ]),
    }
}
