//! Ratatui adapter - Rendering implementation

use crate::modules::ui::domain::models::AppState;
use crate::modules::ui::ports::UIRenderer;
use crate::presentation::tui::components::help::draw_help_modal;
use crate::presentation::tui::components::styles::with_theme;
use crate::shared::constants::{COLUMN_PERCENTAGES, DEFAULT_TAB_HEIGHT};
use crate::shared::kernel::result::AppResult;
use crate::shared::types::{Column, Tab};
use async_trait::async_trait;
use ratatui::{
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::{Modifier, Style},
    symbols,
    widgets::Tabs,
    Frame,
};
use ratatui_ui::{Panel, StatusBar, TextBlock, Theme as RatatuiTheme};

use super::types::{RATerminal, RatatuiAdapter};

#[async_trait]
impl UIRenderer for RatatuiAdapter {
    async fn render(&mut self, state: &AppState) -> AppResult<()> {
        if let Some(terminal) = self.terminal.as_mut() {
            render_app_state(terminal, state)?;
        }
        Ok(())
    }

    async fn clear(&mut self) -> AppResult<()> {
        if let Some(terminal) = self.terminal.as_mut() {
            terminal.clear()?;
        }
        Ok(())
    }
}

pub(crate) fn render_app_state(terminal: &mut RATerminal, state: &AppState) -> AppResult<()> {
    terminal.draw(|f| {
        let size = f.area();

        // Get current theme colors
        let theme = with_theme(|t| t.clone());
        let colors = &theme.colors;

        // Render help modal if active
        if state.show_help {
            draw_help_modal(
                f,
                size,
                state.ui_state.current_tab,
                state.ui_state.current_column,
            );
            return;
        }

        // Get current tab index
        let tab_index = state.ui_state.current_tab as usize;

        // Build tab titles
        let tab_titles: Vec<&str> = Tab::all().iter().map(|t| t.label()).collect();

        // Create tabs widget with theme colors
        let tabs = Tabs::new(tab_titles)
            .select(tab_index)
            .style(Style::default().fg(colors.tab_inactive))
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(colors.tab_highlight),
            )
            .divider(symbols::DOT);

        // Calculate layout
        let tab_height = DEFAULT_TAB_HEIGHT;
        let tabs_area = Rect::new(0, 0, size.width, tab_height);
        let status_height = 1u16;
        let content_height = size
            .height
            .saturating_sub(tab_height)
            .saturating_sub(status_height);
        let content_area = Rect::new(0, tab_height, size.width, content_height);

        // Render tabs at top
        f.render_widget(tabs, tabs_area);

        // Render tab content using the renderer
        render_tab_content(f, state, content_area, colors);

        // Render status bar at bottom
        render_status_bar(f, state, size, colors);
    })?;

    Ok(())
}

fn render_tab_content(
    f: &mut Frame,
    state: &AppState,
    area: Rect,
    colors: &crate::presentation::tui::components::theme::ColorPalette,
) {
    // Get tab content from app state
    let content = state.current_tab_content();

    // 3-column layout
    let column_widths = [
        Constraint::Percentage(COLUMN_PERCENTAGES[0]),
        Constraint::Percentage(COLUMN_PERCENTAGES[1]),
        Constraint::Percentage(COLUMN_PERCENTAGES[2]),
    ];

    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(column_widths.as_ref())
        .split(area);

    // Render each column
    render_column(f, columns[0], Column::Left, &content.left, state, colors);
    render_column(
        f,
        columns[1],
        Column::Center,
        &content.center,
        state,
        colors,
    );
    render_column(f, columns[2], Column::Right, &content.right, state, colors);
}

fn render_column(
    f: &mut Frame,
    area: Rect,
    column: Column,
    content: &str,
    state: &AppState,
    colors: &crate::presentation::tui::components::theme::ColorPalette,
) {
    let rt_theme = RatatuiTheme::default();
    let is_selected = state.ui_state.current_column == column;
    let border_color = if is_selected {
        colors.border_active
    } else {
        colors.border_inactive
    };

    let panel = Panel::new(&rt_theme)
        .title(format!(
            " {} ",
            get_column_title(state.ui_state.current_tab, column)
        ))
        .style(Style::default().fg(border_color).bg(colors.background));

    // Content inside column (inset by 1 for border)
    let inner = area.inner(Margin::default());
    let paragraph = TextBlock::new(content, &rt_theme)
        .style(Style::default().fg(colors.text).bg(colors.background))
        .into_paragraph()
        .block(panel.into_block());
    f.render_widget(paragraph, area);
}

fn render_status_bar(
    f: &mut Frame,
    state: &AppState,
    size: Rect,
    colors: &crate::presentation::tui::components::theme::ColorPalette,
) {
    let status_height = 1u16;
    let status_area = Rect::new(
        0,
        size.height.saturating_sub(status_height),
        size.width,
        status_height,
    );

    let tab_name = state.ui_state.current_tab.label();
    let col_name = match state.ui_state.current_column {
        Column::Left => "Left",
        Column::Center => "Center",
        Column::Right => "Right",
    };
    let focus_status = if state.ui_state.is_focused {
        "ON"
    } else {
        "OFF"
    };
    let selected_idx = get_selected_index(state);

    let status_text = format!(
        " [Tab] {} | [Col] {} | [Focus] {} | [{}] | [f] Toggle | [q] Quit ",
        tab_name, col_name, focus_status, selected_idx
    );

    let rt_theme = RatatuiTheme::default();
    let status = StatusBar::new(status_text, &rt_theme).style(
        Style::default()
            .fg(colors.status_bar_fg)
            .bg(colors.status_bar_bg),
    );
    f.render_widget(status.to_line(), status_area);
}

// === Column title helpers ===

const fn get_column_title(tab: Tab, column: Column) -> &'static str {
    match (tab, column) {
        (Tab::Agent, Column::Left) => "Context",
        (Tab::Agent, Column::Center) => "Chat",
        (Tab::Agent, Column::Right) => "Actions",
        (Tab::Git, Column::Left) => "Status",
        (Tab::Git, Column::Center) => "Diff",
        (Tab::Git, Column::Right) => "History",
        (Tab::Cli, Column::Left) => "Input",
        (Tab::Cli, Column::Center) => "Output",
        (Tab::Cli, Column::Right) => "History",
        (Tab::Snippet, Column::Left) => "Library",
        (Tab::Snippet, Column::Center) => "Editor",
        (Tab::Snippet, Column::Right) => "Tags",
        (Tab::Skills, Column::Left) => "Tree",
        (Tab::Skills, Column::Center) => "Detail",
        (Tab::Skills, Column::Right) => "Progress",
        (Tab::Workflows, Column::Left) => "List",
        (Tab::Workflows, Column::Center) => "Editor",
        (Tab::Workflows, Column::Right) => "History",
        (Tab::Files, Column::Left) => "Explorer",
        (Tab::Files, Column::Center) => "Content",
        (Tab::Files, Column::Right) => "Actions",
        (Tab::Settings, Column::Left) => "Categories",
        (Tab::Settings, Column::Center) => "Options",
        (Tab::Settings, Column::Right) => "Keys",
        _ => "Unknown",
    }
}

fn get_selected_index(state: &AppState) -> String {
    match state.ui_state.current_tab {
        Tab::Agent => format!("msg:{}", state.agent_tab_state.messages.len()),
        Tab::Git => format!("file:{}", state.git_tab_state.selected_file_index),
        Tab::Cli => format!(
            "cmd:{}",
            state.cli_tab_state.selected_history_index.unwrap_or(0)
        ),
        Tab::Snippet => format!("snip:{}", state.snippet_tab_state.selected_snippet_index),
        Tab::Skills => format!("skill:{}", state.skills_tab_state.selected_skill_index),
        Tab::Workflows => format!("wf:{}", state.workflows_tab_state.selected_workflow_index),
        Tab::Files => format!("f:{}", state.files_tab_state.selected_file_index),
        Tab::Settings => format!("cat:{}", state.settings_tab_state.selected_category_index),
        _ => "unknown".to_string(),
    }
}
