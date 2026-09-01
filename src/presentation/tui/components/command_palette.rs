use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::ListItem,
    Frame,
};
use ratatui_ui::{CommandItem, CommandPalette};

use super::styles::*;
use crate::modules::ui::domain::models::app_commands::get_tab_specific_commands;
use crate::modules::ui::domain::models::AppState;

pub(crate) fn draw_command_palette(frame: &mut Frame, area: Rect, state: &AppState) {
    if !state.show_command_palette {
        return;
    }

    let theme = rt_theme();
    let tab_commands = get_tab_specific_commands(state.ui_state.current_tab);

    let items: Vec<CommandItem<'static>> = tab_commands
        .into_iter()
        .map(|cmd| {
            let label = cmd.name.clone();
            let mut spans = vec![
                Span::styled(
                    cmd.name.clone(),
                    Style::default()
                        .fg(text_bright())
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(" "),
            ];
            if let Some(shortcut) = &cmd.shortcut {
                spans.push(Span::styled(
                    format!("[{}]", shortcut),
                    Style::default().fg(text_dim()),
                ));
                spans.push(Span::raw(" "));
            }
            spans.push(Span::styled(
                cmd.description.clone(),
                Style::default().fg(text()),
            ));

            let item = ListItem::new(Text::from(vec![Line::from(spans)]));
            CommandItem::new(label, item)
        })
        .collect();

    let palette = CommandPalette::new(&theme)
        .items(items)
        .query(state.command_input.clone())
        .selected(Some(state.command_palette_selected))
        .title(Line::raw("🔍 Command Palette"))
        .placeholder("Type a command…");

    palette.render(area, frame.buffer_mut());
}
