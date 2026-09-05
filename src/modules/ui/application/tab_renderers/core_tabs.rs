use super::TabColumns;
use crate::modules::ui::domain::models::AppState;
use crate::presentation::tui::components::markdown_renderer;

/// Render Agent tab columns
pub(crate) fn render_agent_tab(state: &AppState) -> TabColumns {
    let tab_state = &state.agent_tab_state;
    let cwd = std::env::current_dir()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| ".".to_string());

    // Left: sessions loaded from the session repository
    let left = {
        let session_list = if tab_state.sessions.is_empty() {
            "  (no saved sessions)".to_string()
        } else {
            tab_state
                .sessions
                .iter()
                .take(15)
                .enumerate()
                .map(|(i, s)| {
                    let marker = if i == tab_state.selected_session_index {
                        ">"
                    } else {
                        " "
                    };
                    format!("{marker} {} ({} msgs)", s.name, s.messages.len())
                })
                .collect::<Vec<_>>()
                .join("\n")
        };
        format!(
            "Session: {}\n\nSessions ({}):\n{}\n\npath: {}",
            tab_state.session_id.as_deref().unwrap_or("No session"),
            tab_state.sessions.len(),
            session_list,
            cwd,
        )
    };

    let center = if tab_state.messages.is_empty() {
        "┌─────────────────────────────────────────┐\n│                                         │\n│   Welcome to Agent-TUI                  │\n│                                         │\n│   [Ctrl+N] Start a new session          │\n│   [Shift+Tab] Switch tabs               │\n│   [f] Toggle focus                      │\n│   [?] Help                              │\n│                                         │\n└─────────────────────────────────────────┘".to_string()
    } else {
        tab_state
            .messages
            .iter()
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
                format!(
                    "[{}] {} {}\n{}",
                    m.timestamp.format("%H:%M"),
                    role_icon,
                    m.role,
                    rendered_content
                )
            })
            .collect::<Vec<_>>()
            .join("\n\n")
    };

    let right = format!(
        "Actions:\n  [Enter] {}\n  [Ctrl+N] New session\n\nSessions: {} saved\nMessages: {}",
        if tab_state.session_id.is_none() {
            "Start session"
        } else {
            "Load selected session"
        },
        tab_state.sessions.len(),
        tab_state.messages.len(),
    );

    TabColumns::new(left, center, right)
}

/// Render Git tab columns — backed by `git_tab_state` loaded via Git2Adapter
pub(crate) fn render_git_tab(state: &AppState) -> TabColumns {
    let tab_state = &state.git_tab_state;

    // Selection spans unstaged files first, then staged files.
    let selected_unstaged = tab_state.selected_file_index;
    let unstaged_len = tab_state.unstaged_files.len();

    let format_list = |files: &[String], base: usize| -> String {
        if files.is_empty() {
            "  (none)".to_string()
        } else {
            files
                .iter()
                .enumerate()
                .map(|(i, f)| {
                    let marker = if base + i == selected_unstaged {
                        ">"
                    } else {
                        " "
                    };
                    format!("{marker} {f}")
                })
                .collect::<Vec<_>>()
                .join("\n")
        }
    };

    let branch = if tab_state.current_branch.is_empty() {
        "(not a git repo)"
    } else {
        tab_state.current_branch.as_str()
    };

    let left = format!(
        "Branch: {branch}\n\nStaged ({}):\n{}\n\nUnstaged ({}):\n{}",
        tab_state.staged_files.len(),
        format_list(&tab_state.staged_files, unstaged_len),
        tab_state.unstaged_files.len(),
        format_list(&tab_state.unstaged_files, 0),
    );

    let selected = tab_state
        .unstaged_files
        .iter()
        .chain(tab_state.staged_files.iter())
        .nth(tab_state.selected_file_index);
    let center = match (selected, tab_state.diff.is_empty()) {
        (Some(file), false) => format!("File: {file}\n\n{}", tab_state.diff),
        (Some(file), true) => format!("File: {file}\n\n(press Enter to load diff)"),
        (None, _) => {
            "No file selected\n\nMake changes in the working tree to see them here.".to_string()
        }
    };

    let right = format!(
        "Keys:\n  [Enter] Diff preview\n  [s] Stage    [u] Unstage\n  [p] Push     [r] Refresh\n\nWorking tree: {} staged, {} unstaged",
        tab_state.staged_files.len(),
        tab_state.unstaged_files.len(),
    );

    TabColumns::new(left, center, right)
}

/// Render Files tab columns — backed by `files_tab_state` from the file scanner
pub(crate) fn render_files_tab(state: &AppState) -> TabColumns {
    let tab_state = &state.files_tab_state;

    let left = {
        let entries: Vec<String> = tab_state
            .files
            .iter()
            .take(20)
            .enumerate()
            .map(|(i, name)| {
                let marker = if i == tab_state.selected_file_index {
                    ">"
                } else {
                    " "
                };
                format!("{marker} {name}")
            })
            .collect();
        if entries.is_empty() {
            "(empty directory)".to_string()
        } else {
            entries.join("\n")
        }
    };

    let center = if tab_state.preview.is_empty() {
        match tab_state.files.get(tab_state.selected_file_index) {
            Some(name) => format!("{name}\n\n(press Enter to open/preview)"),
            None => "Select a file to preview".to_string(),
        }
    } else {
        tab_state.preview.clone()
    };

    let right = format!(
        "Path: {}\nShow hidden: {}\nEntries: {}\n\n[Enter] Open/Preview\n[r] Refresh",
        tab_state.current_path,
        tab_state.show_hidden,
        tab_state.files.len(),
    );

    TabColumns::new(left, center, right)
}

/// Render Terminal tab columns — backed by `terminal_tab_state`
pub(crate) fn render_terminal_tab(state: &AppState) -> TabColumns {
    let tab_state = &state.terminal_tab_state;

    let left = if tab_state.history.is_empty() {
        "No commands yet".to_string()
    } else {
        tab_state
            .history
            .iter()
            .rev()
            .take(15)
            .enumerate()
            .map(|(i, c)| {
                let marker = if Some(i) == tab_state.selected_history_index {
                    ">"
                } else {
                    " "
                };
                format!("{marker} {c}")
            })
            .collect::<Vec<_>>()
            .join("\n")
    };

    let center = if tab_state.output.is_empty() {
        format!("$ {}\n\nOutput:\n  (no output)", tab_state.terminal_input)
    } else {
        format!(
            "$ {}\n\n{}",
            tab_state.terminal_input,
            tab_state
                .output
                .iter()
                .rev()
                .take(30)
                .rev()
                .cloned()
                .collect::<Vec<_>>()
                .join("\n")
        )
    };

    let right = format!(
        "History: {} commands\nSelected: {}\n\n[Enter] Execute\n[Ctrl+L] Clear\n[Up/Down] History",
        tab_state.history.len(),
        tab_state
            .selected_history_index
            .map(|i| i.to_string())
            .unwrap_or_else(|| "-".to_string())
    );

    TabColumns::new(left, center, right)
}

/// Render CLI tab columns — backed by `cli_tab_state`
pub(crate) fn render_cli_tab(state: &AppState) -> TabColumns {
    let tab_state = &state.cli_tab_state;

    let left = format!("Input:\n> {}", tab_state.command_input);
    let center = if tab_state.output.is_empty() {
        "Output:\n  (no output)".to_string()
    } else {
        format!(
            "Output:\n{}",
            tab_state
                .output
                .iter()
                .rev()
                .take(30)
                .rev()
                .cloned()
                .collect::<Vec<_>>()
                .join("\n")
        )
    };
    let right = {
        let history = if tab_state.history.is_empty() {
            "  (empty)".to_string()
        } else {
            tab_state
                .history
                .iter()
                .rev()
                .take(15)
                .enumerate()
                .map(|(i, c)| {
                    let marker = if Some(i) == tab_state.selected_history_index {
                        ">"
                    } else {
                        " "
                    };
                    format!("{marker} {c}")
                })
                .collect::<Vec<_>>()
                .join("\n")
        };
        format!("History ({}):\n{}", tab_state.history.len(), history)
    };

    TabColumns::new(left, center, right)
}
