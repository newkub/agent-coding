use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::Style,
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use super::styles::*;
use crate::shared::kernel::types::{Column, Tab};

/// Context-based help modal that shows relevant shortcuts based on current tab and state
pub(crate) fn draw_help_modal(
    frame: &mut Frame,
    area: Rect,
    current_tab: Tab,
    current_column: Column,
) {
    // Centered help modal
    let width = 70.min(area.width - 4);
    let height = 28.min(area.height - 4);
    let x = (area.width - width) / 2;
    let y = (area.height - height) / 2;

    let modal_area = Rect::new(x, y, width, height);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(ACCENT))
        .border_type(BorderType::Rounded)
        .title(format!("📖 Help - {} Tab", current_tab.label()))
        .title_style(
            Style::default()
                .fg(TEXT_BRIGHT)
                .add_modifier(ratatui::style::Modifier::BOLD),
        );

    frame.render_widget(block, modal_area);

    let content_area = modal_area.inner(ratatui::layout::Margin {
        horizontal: 1,
        vertical: 1,
    });

    // Create three-column layout
    let [left, center, right] = Layout::horizontal([
        Constraint::Percentage(33),
        Constraint::Percentage(33),
        Constraint::Percentage(34),
    ])
    .areas(content_area);

    // Global shortcuts (left column)
    let global_content = get_global_shortcuts();
    let left_para = Paragraph::new(global_content).style(Style::default().fg(TEXT));
    frame.render_widget(left_para, left);

    // Tab-specific shortcuts (center column)
    let tab_content = get_tab_shortcuts(current_tab);
    let center_para = Paragraph::new(tab_content).style(Style::default().fg(TEXT));
    frame.render_widget(center_para, center);

    // Column-specific shortcuts (right column)
    let column_content = get_column_shortcuts(current_tab, current_column);
    let right_para = Paragraph::new(column_content).style(Style::default().fg(TEXT));
    frame.render_widget(right_para, right);

    // Footer hint
    let footer_text = "Press Esc or ? to close | Shortcuts change based on context";
    let footer_para = Paragraph::new(footer_text)
        .alignment(Alignment::Center)
        .style(Style::default().fg(TEXT_DIM));

    let footer_area = Rect::new(x, y + height - 1, width, 1);
    frame.render_widget(footer_para, footer_area);
}

fn get_global_shortcuts() -> Text<'static> {
    Text::from(vec![
        Line::from(vec![Span::styled(
            "🌍 GLOBAL",
            Style::default()
                .fg(ACCENT)
                .add_modifier(ratatui::style::Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![
            Span::raw("  Tab         "),
            Span::styled("Next tab", Style::default().fg(TEXT_DIM)),
        ]),
        Line::from(vec![
            Span::raw("  Shift+Tab   "),
            Span::styled("Prev tab", Style::default().fg(TEXT_DIM)),
        ]),
        Line::from(vec![
            Span::raw("  → ←         "),
            Span::styled("Next/Prev column", Style::default().fg(TEXT_DIM)),
        ]),
        Line::from(vec![
            Span::raw("  f           "),
            Span::styled("Toggle focus", Style::default().fg(TEXT_DIM)),
        ]),
        Line::from(vec![
            Span::raw("  ? / Ctrl+H   "),
            Span::styled("Toggle help", Style::default().fg(TEXT_DIM)),
        ]),
        Line::from(vec![
            Span::raw("  q / Esc      "),
            Span::styled("Quit", Style::default().fg(TEXT_DIM)),
        ]),
        Line::from(vec![
            Span::raw("  Ctrl+K       "),
            Span::styled("Command palette", Style::default().fg(TEXT_DIM)),
        ]),
    ])
}

fn get_tab_shortcuts(tab: Tab) -> Text<'static> {
    match tab {
        Tab::Agent => Text::from(vec![
            Line::from(vec![Span::styled(
                "🤖 AGENT",
                Style::default()
                    .fg(ACCENT)
                    .add_modifier(ratatui::style::Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![
                Span::raw("  Enter       "),
                Span::styled("Send message", Style::default().fg(TEXT_DIM)),
            ]),
            Line::from(vec![
                Span::raw("  Ctrl+N      "),
                Span::styled("New session", Style::default().fg(TEXT_DIM)),
            ]),
            Line::from(vec![
                Span::raw("  Ctrl+S      "),
                Span::styled("Save session", Style::default().fg(TEXT_DIM)),
            ]),
            Line::from(vec![
                Span::raw("  Ctrl+L      "),
                Span::styled("List sessions", Style::default().fg(TEXT_DIM)),
            ]),
            Line::from(vec![
                Span::raw("  ↑↓          "),
                Span::styled("Navigate history", Style::default().fg(TEXT_DIM)),
            ]),
        ]),
        Tab::Git => Text::from(vec![
            Line::from(vec![Span::styled(
                "📚 GIT",
                Style::default()
                    .fg(ACCENT)
                    .add_modifier(ratatui::style::Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![
                Span::raw("  s           "),
                Span::styled("Stage file", Style::default().fg(TEXT_DIM)),
            ]),
            Line::from(vec![
                Span::raw("  u           "),
                Span::styled("Unstage file", Style::default().fg(TEXT_DIM)),
            ]),
            Line::from(vec![
                Span::raw("  c           "),
                Span::styled("Commit", Style::default().fg(TEXT_DIM)),
            ]),
            Line::from(vec![
                Span::raw("  p           "),
                Span::styled("Push", Style::default().fg(TEXT_DIM)),
            ]),
            Line::from(vec![
                Span::raw("  f           "),
                Span::styled("Fetch", Style::default().fg(TEXT_DIM)),
            ]),
            Line::from(vec![
                Span::raw("  b           "),
                Span::styled("Create branch", Style::default().fg(TEXT_DIM)),
            ]),
        ]),
        Tab::Files => Text::from(vec![
            Line::from(vec![Span::styled(
                "📁 FILES",
                Style::default()
                    .fg(ACCENT)
                    .add_modifier(ratatui::style::Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![
                Span::raw("  Enter       "),
                Span::styled("Open file", Style::default().fg(TEXT_DIM)),
            ]),
            Line::from(vec![
                Span::raw("  e           "),
                Span::styled("Edit file", Style::default().fg(TEXT_DIM)),
            ]),
            Line::from(vec![
                Span::raw("  d           "),
                Span::styled("Delete file", Style::default().fg(TEXT_DIM)),
            ]),
            Line::from(vec![
                Span::raw("  r           "),
                Span::styled("Rename file", Style::default().fg(TEXT_DIM)),
            ]),
            Line::from(vec![
                Span::raw("  /           "),
                Span::styled("Search files", Style::default().fg(TEXT_DIM)),
            ]),
        ]),
        Tab::Terminal => Text::from(vec![
            Line::from(vec![Span::styled(
                "� TERMINAL",
                Style::default()
                    .fg(ACCENT)
                    .add_modifier(ratatui::style::Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![
                Span::raw("  Enter       "),
                Span::styled("Execute command", Style::default().fg(TEXT_DIM)),
            ]),
            Line::from(vec![
                Span::raw("  Ctrl+C      "),
                Span::styled("Cancel command", Style::default().fg(TEXT_DIM)),
            ]),
            Line::from(vec![
                Span::raw("  ↑↓          "),
                Span::styled("Command history", Style::default().fg(TEXT_DIM)),
            ]),
            Line::from(vec![
                Span::raw("  Ctrl+L      "),
                Span::styled("Clear output", Style::default().fg(TEXT_DIM)),
            ]),
        ]),
        _ => Text::from(vec![
            Line::from(vec![Span::styled(
                "📋 TAB",
                Style::default()
                    .fg(ACCENT)
                    .add_modifier(ratatui::style::Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![
                Span::raw("  ↑↓          "),
                Span::styled("Navigate items", Style::default().fg(TEXT_DIM)),
            ]),
            Line::from(vec![
                Span::raw("  Enter       "),
                Span::styled("Select item", Style::default().fg(TEXT_DIM)),
            ]),
            Line::from(vec![
                Span::raw("  Esc         "),
                Span::styled("Go back", Style::default().fg(TEXT_DIM)),
            ]),
        ]),
    }
}

fn get_column_shortcuts(tab: Tab, column: Column) -> Text<'static> {
    match (tab, column) {
        (Tab::Git, Column::Left) => Text::from(vec![
            Line::from(vec![Span::styled(
                "📄 STATUS",
                Style::default()
                    .fg(ACCENT)
                    .add_modifier(ratatui::style::Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![
                Span::raw("  s           "),
                Span::styled("Stage selected", Style::default().fg(TEXT_DIM)),
            ]),
            Line::from(vec![
                Span::raw("  u           "),
                Span::styled("Unstage selected", Style::default().fg(TEXT_DIM)),
            ]),
            Line::from(vec![
                Span::raw("  d           "),
                Span::styled("Discard changes", Style::default().fg(TEXT_DIM)),
            ]),
            Line::from(vec![
                Span::raw("  i           "),
                Span::styled("Stage hunk", Style::default().fg(TEXT_DIM)),
            ]),
        ]),
        (Tab::Git, Column::Center) => Text::from(vec![
            Line::from(vec![Span::styled(
                "📝 DIFF",
                Style::default()
                    .fg(ACCENT)
                    .add_modifier(ratatui::style::Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![
                Span::raw("  ↑↓          "),
                Span::styled("Scroll diff", Style::default().fg(TEXT_DIM)),
            ]),
            Line::from(vec![
                Span::raw("  PageUp/Down "),
                Span::styled("Page scroll", Style::default().fg(TEXT_DIM)),
            ]),
            Line::from(vec![
                Span::raw("  Home/End    "),
                Span::styled("Jump to start/end", Style::default().fg(TEXT_DIM)),
            ]),
        ]),
        (Tab::Git, Column::Right) => Text::from(vec![
            Line::from(vec![Span::styled(
                "📜 HISTORY",
                Style::default()
                    .fg(ACCENT)
                    .add_modifier(ratatui::style::Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![
                Span::raw("  Enter       "),
                Span::styled("View commit", Style::default().fg(TEXT_DIM)),
            ]),
            Line::from(vec![
                Span::raw("  b           "),
                Span::styled("Checkout commit", Style::default().fg(TEXT_DIM)),
            ]),
            Line::from(vec![
                Span::raw("  /           "),
                Span::styled("Search commits", Style::default().fg(TEXT_DIM)),
            ]),
        ]),
        (Tab::Agent, Column::Center) => Text::from(vec![
            Line::from(vec![Span::styled(
                "💬 CHAT",
                Style::default()
                    .fg(ACCENT)
                    .add_modifier(ratatui::style::Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![
                Span::raw("  Enter       "),
                Span::styled("Send message", Style::default().fg(TEXT_DIM)),
            ]),
            Line::from(vec![
                Span::raw("  Ctrl+G      "),
                Span::styled("Send (alt)", Style::default().fg(TEXT_DIM)),
            ]),
            Line::from(vec![
                Span::raw("  Tab         "),
                Span::styled("Auto-complete", Style::default().fg(TEXT_DIM)),
            ]),
            Line::from(vec![
                Span::raw("  Esc         "),
                Span::styled("Clear input", Style::default().fg(TEXT_DIM)),
            ]),
        ]),
        _ => Text::from(vec![
            Line::from(vec![Span::styled(
                "📍 COLUMN",
                Style::default()
                    .fg(ACCENT)
                    .add_modifier(ratatui::style::Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![
                Span::raw("  ↑↓          "),
                Span::styled("Navigate", Style::default().fg(TEXT_DIM)),
            ]),
            Line::from(vec![
                Span::raw("  Enter       "),
                Span::styled("Select", Style::default().fg(TEXT_DIM)),
            ]),
            Line::from(vec![
                Span::raw("  Esc         "),
                Span::styled("Back", Style::default().fg(TEXT_DIM)),
            ]),
        ]),
    }
}
