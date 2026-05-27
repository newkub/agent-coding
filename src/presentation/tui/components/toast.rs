use std::collections::VecDeque;

use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use super::styles::*;
use crate::modules::ui::domain::models::{AppState, ToastKind, ToastNotification};

pub(crate) fn draw_toasts(frame: &mut Frame, area: Rect, state: &AppState) {
    if state.toasts.is_empty() {
        return;
    }

    let [toast_area, _] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Fill(1),
    ]).areas(area);

    let toasts: Vec<Line> = state.toasts.iter().map(|toast| {
        let icon = match toast.kind {
            ToastKind::Info => "ℹ️",
            ToastKind::Success => "✅",
            ToastKind::Warning => "⚠️",
            ToastKind::Error => "❌",
        };
        let time = toast.timestamp.format("%H:%M").to_string();
        
        Line::from(vec![
            Span::raw(" "),
            Span::raw(icon),
            Span::raw(" "),
            Span::raw(&toast.message),
            Span::raw("  "),
            Span::styled(time, Style::default().fg(TEXT_DIM)),
        ])
    }).collect();

    let paragraph = Paragraph::new(toasts)
        .alignment(Alignment::Left)
        .style(Style::default().bg(BG_LIGHT));

    frame.render_widget(paragraph, toast_area);
}

pub(crate) fn draw_toast_overlay(frame: &mut Frame, area: Rect, toasts: &VecDeque<ToastNotification>) {
    // Draw a toast notification in the bottom-left corner
    if toasts.is_empty() {
        return;
    }

    let toast_width = 40.min(area.width - 2);
    let toast_height = 3;
    let x = 1;
    let y = area.height.saturating_sub(toast_height + 1);

    let toast_area = Rect::new(x, y, toast_width, toast_height);

    if let Some(toast) = toasts.front() {
        let icon = match toast.kind {
            ToastKind::Info => "ℹ️",
            ToastKind::Success => "✅",
            ToastKind::Warning => "⚠️",
            ToastKind::Error => "❌",
        };
        let border_color = match toast.kind {
            ToastKind::Info => ACCENT,
            ToastKind::Success => ACCENT_GREEN,
            ToastKind::Warning => ACCENT_YELLOW,
            ToastKind::Error => ACCENT_RED,
        };

        let content = format!("{} {} ", icon, toast.message);
        
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(border_color))
            .style(Style::default().bg(BG_LIGHT));

        let paragraph = Paragraph::new(content)
            .style(Style::default().fg(TEXT));

        frame.render_widget(block, toast_area);
        frame.render_widget(paragraph, toast_area.inner(ratatui::layout::Margin { horizontal: 1, vertical: 0 }));
    }
}