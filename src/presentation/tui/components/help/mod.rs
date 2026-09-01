mod column_shortcuts;
mod global_shortcuts;
mod tab_shortcuts;

use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::Line,
    widgets::{Clear, Paragraph, Widget},
    Frame,
};
use ratatui_ui::{Panel, Popup, PopupSize, TextBlock};

use super::styles::*;
use crate::shared::kernel::types::{Column, Tab};

/// Context-based help modal that shows relevant shortcuts based on current tab and state
pub(crate) fn draw_help_modal(
    frame: &mut Frame,
    area: Rect,
    current_tab: Tab,
    current_column: Column,
) {
    let theme = rt_theme();

    let popup = Popup::new(&theme)
        .size(PopupSize::Custom { w: 70, h: 60 })
        .area(area);
    Clear.render(popup, frame.buffer_mut());

    let title = format!("📖 Help - {} Tab", current_tab.label());
    let block = Panel::new(&theme)
        .title(Line::raw(title))
        .style(Style::default().fg(accent()).bg(bg()))
        .into_block()
        .title_style(
            Style::default()
                .fg(text_bright())
                .bg(bg())
                .add_modifier(Modifier::BOLD),
        );

    let inner = block.inner(popup);
    block.render(popup, frame.buffer_mut());

    // Create three-column layout
    let [left, center, right] = Layout::horizontal([
        Constraint::Percentage(33),
        Constraint::Percentage(33),
        Constraint::Percentage(34),
    ])
    .areas(inner);

    let base_style = Style::default().fg(text()).bg(bg());

    // Global shortcuts (left column)
    let global_content = global_shortcuts::get_global_shortcuts();
    let left_para = TextBlock::new(global_content, &theme)
        .style(base_style)
        .wrap(false)
        .into_paragraph();
    frame.render_widget(left_para, left);

    // Tab-specific shortcuts (center column)
    let tab_content = tab_shortcuts::get_tab_shortcuts(current_tab);
    let center_para = TextBlock::new(tab_content, &theme)
        .style(base_style)
        .wrap(false)
        .into_paragraph();
    frame.render_widget(center_para, center);

    // Column-specific shortcuts (right column)
    let column_content = column_shortcuts::get_column_shortcuts(current_tab, current_column);
    let right_para = TextBlock::new(column_content, &theme)
        .style(base_style)
        .wrap(false)
        .into_paragraph();
    frame.render_widget(right_para, right);

    // Footer hint
    let footer_text = "Press Esc or ? to close | Shortcuts change based on context";
    let footer_para = Paragraph::new(footer_text)
        .alignment(Alignment::Center)
        .style(Style::default().fg(text_dim()).bg(bg()));

    let footer_area = Rect::new(popup.x, popup.y + popup.height - 1, popup.width, 1);
    frame.render_widget(footer_para, footer_area);
}
