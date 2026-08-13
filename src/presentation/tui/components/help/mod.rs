mod column_shortcuts;
mod global_shortcuts;
mod tab_shortcuts;

use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::Style,
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
        .border_style(Style::default().fg(accent()))
        .border_type(BorderType::Rounded)
        .title(format!("📖 Help - {} Tab", current_tab.label()))
        .title_style(
            Style::default()
                .fg(text_bright())
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
    let global_content = global_shortcuts::get_global_shortcuts();
    let left_para = Paragraph::new(global_content).style(Style::default().fg(text()));
    frame.render_widget(left_para, left);

    // Tab-specific shortcuts (center column)
    let tab_content = tab_shortcuts::get_tab_shortcuts(current_tab);
    let center_para = Paragraph::new(tab_content).style(Style::default().fg(text()));
    frame.render_widget(center_para, center);

    // Column-specific shortcuts (right column)
    let column_content = column_shortcuts::get_column_shortcuts(current_tab, current_column);
    let right_para = Paragraph::new(column_content).style(Style::default().fg(text()));
    frame.render_widget(right_para, right);

    // Footer hint
    let footer_text = "Press Esc or ? to close | Shortcuts change based on context";
    let footer_para = Paragraph::new(footer_text)
        .alignment(Alignment::Center)
        .style(Style::default().fg(text_dim()));

    let footer_area = Rect::new(x, y + height - 1, width, 1);
    frame.render_widget(footer_para, footer_area);
}
