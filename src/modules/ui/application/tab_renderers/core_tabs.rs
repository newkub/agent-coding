use crate::modules::ui::domain::models::AppState;
use crate::presentation::tui::components::markdown_renderer;
use super::TabRenderResult;
use ratatui::layout::Rect;
use ratatui::widgets::Paragraph;

/// Render Agent tab content
pub(crate) fn render_agent_tab(state: &AppState) -> TabRenderResult<'_> {
    let tab_state = &state.agent_tab_state;
    
    let left = format!(
        "Session: {}\n\nVariables:\n  path: /project\n  env: dev\n  target: local\n\n[Context]\n• System info\n• User prefs\n• Workspace",
        tab_state.session_id.as_deref().unwrap_or("No session")
    );
    
    let center = if tab_state.messages.is_empty() {
        "┌─────────────────────────────────────────┐\n│                                         │\n│   Welcome to Agent-TUI                  │\n│                                         │\n│   Type your message below...           │\n│                                         │\n│   [Shift+Tab] Switch tabs              │\n│   [f] Toggle focus                     │\n│                                         │\n└─────────────────────────────────────────┘".to_string()
    } else {
        tab_state.messages.iter()
            .map(|m| {
                let role_icon = if m.role == "user" { "👤" } else { "🤖" };
                let rendered_content = if m.role == "assistant" {
                    markdown_renderer::render_markdown(&m.content)
                        .iter()
                        .map(|line| line.to_string())
                        .collect::<Vec<_>>()
                        .join("\n")
                } else {
                    m.content.clone()
                };
                format!("[{}] {} {}\n{}", 
                    m.timestamp.format("%H:%M"), 
                    role_icon, 
                    m.role, 
                    rendered_content
                )
            })
            .collect::<Vec<_>>()
            .join("\n\n")
    };
    
    let right = "[1] Execute\n[2] Review\n[3] Confirm\n[4] Cancel\n\n[Ctrl+S] Save\n[Ctrl+E] Export\n[Ctrl+H] History";
    
    let content = Paragraph::new(format!("{}\n\n{}\n\n{}", left, center, right));
    TabRenderResult::new(content, Rect::new(0, 0, 0, 0))
}

/// Render Git tab content
pub(crate) fn render_git_tab(state: &AppState) -> TabRenderResult<'_> {
    let tab_state = &state.git_tab_state;
    
    let content = Paragraph::new(format!(
        "Git Status\n\nStaged: {}\nUnstaged: {}\n\nSelected: {}",
        tab_state.staged_files.len(),
        tab_state.unstaged_files.len(),
        tab_state.selected_file_index
    ));
    TabRenderResult::new(content, Rect::new(0, 0, 0, 0))
}

/// Render Files tab content
pub(crate) fn render_files_tab(state: &AppState) -> TabRenderResult<'_> {
    let tab_state = &state.files_tab_state;
    
    let content = Paragraph::new(format!(
        "Files\n\nPath: {}\nSelected: {}\nShow Hidden: {}",
        tab_state.current_path,
        tab_state.selected_file_index,
        tab_state.show_hidden
    ));
    TabRenderResult::new(content, Rect::new(0, 0, 0, 0))
}

/// Render Terminal tab content
pub(crate) fn render_terminal_tab(state: &AppState) -> TabRenderResult<'_> {
    let tab_state = &state.terminal_tab_state;
    
    let content = Paragraph::new(format!(
        "Terminal\n\nInput: {}\nHistory Index: {:?}",
        tab_state.terminal_input,
        tab_state.selected_history_index
    ));
    TabRenderResult::new(content, Rect::new(0, 0, 0, 0))
}
