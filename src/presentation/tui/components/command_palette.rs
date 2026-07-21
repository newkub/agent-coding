use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};

use super::styles::*;
use crate::modules::ui::domain::models::app_commands::get_tab_specific_commands;
use crate::modules::ui::domain::models::AppState;

pub(crate) fn draw_command_palette(frame: &mut Frame, area: Rect, state: &AppState) {
    if !state.show_command_palette {
        return;
    }

    let [input_area, list_area] =
        Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(area);

    // Search input
    let input_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(ACCENT))
        .border_type(BorderType::Rounded)
        .title("🔍 Command Palette")
        .title_style(Style::default().fg(TEXT_BRIGHT));

    let input_text = format!("> {}", state.command_input);
    let input_para = Paragraph::new(input_text).style(Style::default().fg(TEXT));

    frame.render_widget(input_block, input_area);
    frame.render_widget(
        input_para,
        input_area.inner(ratatui::layout::Margin {
            horizontal: 2,
            vertical: 1,
        }),
    );

    // Command list - use tab-specific commands
    let query = if state.command_input.len() > 1 {
        state.command_input[1..].trim_start()
    } else {
        ""
    };

    // Get tab-specific commands
    let tab_commands = get_tab_specific_commands(state.ui_state.current_tab);

    // Filter commands by query
    let filtered_commands = if query.is_empty() {
        tab_commands
    } else {
        let query_lower = query.to_lowercase();
        tab_commands
            .into_iter()
            .filter(|cmd| {
                cmd.name.to_lowercase().contains(&query_lower)
                    || cmd.description.to_lowercase().contains(&query_lower)
            })
            .collect()
    };

    let items: Vec<ListItem> = filtered_commands
        .iter()
        .enumerate()
        .map(|(idx, cmd)| {
            let highlight = idx == state.command_palette_selected;

            let shortcut_text = cmd
                .shortcut
                .as_ref()
                .map(|s| format!("[{}]", s))
                .unwrap_or_default();
            ListItem::new(vec![
                Line::from(vec![
                    Span::raw("  "),
                    Span::styled(
                        &cmd.name,
                        Style::default()
                            .fg(if highlight { BG } else { TEXT_BRIGHT })
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::raw(" "),
                    Span::styled(
                        shortcut_text,
                        Style::default().fg(if highlight { TEXT_BRIGHT } else { TEXT_DIM }),
                    ),
                ]),
                Line::from(vec![
                    Span::raw("    "),
                    Span::styled(
                        &cmd.description,
                        Style::default().fg(if highlight { TEXT } else { TEXT_DIM }),
                    ),
                ]),
            ])
            .style(Style::default().bg(if highlight { ACCENT } else { BG_LIGHT }))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM))
        .style(Style::default().fg(ACCENT));

    frame.render_widget(list, list_area);

    // Instructions
    let hint = "↑↓ Navigate | Enter Select | Esc Close";
    let hint_para = Paragraph::new(hint)
        .style(Style::default().fg(TEXT_DIM))
        .alignment(Alignment::Center);

    let hint_area = Rect::new(area.x, area.y + area.height - 1, area.width, 1);
    frame.render_widget(hint_para, hint_area);
}
