use super::TabColumns;
use crate::modules::ui::domain::models::AppState;
use crate::presentation::tui::components::markdown_renderer;
use std::sync::Arc;

/// Render Agent tab columns
pub(crate) fn render_agent_tab(state: &AppState) -> TabColumns {
    let tab_state = &state.agent_tab_state;
    let cwd = std::env::current_dir()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| ".".to_string());

    let left = format!(
        "Session: {}\n\nVariables:\n  path: {}\n  env: dev\n  target: local\n\n[Context]\n• System info\n• User prefs\n• Workspace",
        tab_state.session_id.as_deref().unwrap_or("No session"),
        cwd,
    );

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

    let right =
        "Actions:\n  [1] Execute\n  [2] Review\n  [3] Confirm\n  [4] Cancel\n\n  [Ctrl+N] New session\n  [Ctrl+S] Save\n  [Ctrl+L] History";

    TabColumns::new(left, center, right)
}

/// Render Git tab columns — uses git-app for data
pub(crate) fn render_git_tab(state: &AppState) -> TabColumns {
    let tab_state = &state.git_tab_state;
    let (git_staged, git_unstaged) = git_usecase()
        .map(|uc| (uc.state().staged.len(), uc.state().unstaged.len()))
        .unwrap_or((0, 0));

    let format_list = |files: &[String]| -> String {
        if files.is_empty() {
            "  (none)".to_string()
        } else {
            files
                .iter()
                .enumerate()
                .map(|(i, f)| {
                    let marker = if i == tab_state.selected_file_index {
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

    let left = format!(
        "Staged ({}):\n{}\n\nUnstaged ({}):\n{}",
        tab_state.staged_files.len(),
        format_list(&tab_state.staged_files),
        tab_state.unstaged_files.len(),
        format_list(&tab_state.unstaged_files),
    );

    let selected = tab_state
        .unstaged_files
        .iter()
        .chain(tab_state.staged_files.iter())
        .nth(tab_state.selected_file_index);
    let center = match selected {
        Some(file) => format!("File: {file}\n\nDiff preview is not loaded."),
        None => "No file selected".to_string(),
    };

    let right = format!(
        "Keys:\n  [s] Stage    [u] Unstage\n  [c] Commit   [p] Push\n  [f] Fetch    [b] Branch\n\nWorking tree: {git_staged} staged, {git_unstaged} unstaged"
    );

    TabColumns::new(left, center, right)
}

fn git_usecase() -> Option<git_tui::GitUseCase> {
    let path = std::env::current_dir().ok()?;
    let repo = git_tui::GitCliRepository::new(path.as_path()).ok()?;
    Some(git_tui::GitUseCase::new(
        Arc::new(repo) as Arc<dyn git_tui::GitRepository>
    ))
}

/// Render Files tab columns — uses files-app for data
pub(crate) fn render_files_tab(state: &AppState) -> TabColumns {
    let tab_state = &state.files_tab_state;
    let files_uc = files_tui::FilesUseCase::new();

    let left = {
        let entries: Vec<String> = files_uc
            .entries()
            .iter()
            .take(20)
            .enumerate()
            .map(|(i, e)| {
                let marker = if i == tab_state.selected_file_index {
                    ">"
                } else {
                    " "
                };
                let kind = if e.is_dir { "dir " } else { "file" };
                format!("{marker} [{kind}] {}", e.name)
            })
            .collect();
        if entries.is_empty() {
            "No entries".to_string()
        } else {
            entries.join("\n")
        }
    };

    let center = match files_uc.entries().get(tab_state.selected_file_index) {
        Some(entry) => format!("{}\n\n(preview not loaded)", entry.path.display()),
        None => "Select a file to preview".to_string(),
    };

    let right = format!(
        "Path: {}\nShow hidden: {}\n\n[Enter] Open\n[e] Edit    [d] Delete\n[r] Rename  [/] Search",
        files_uc.filter().current_path,
        tab_state.show_hidden
    );

    TabColumns::new(left, center, right)
}

/// Render Terminal tab columns — uses terminal-app for data
pub(crate) fn render_terminal_tab(state: &AppState) -> TabColumns {
    let tab_state = &state.terminal_tab_state;
    let term_uc = terminal_tui::TerminalUseCase::new();

    let left = if term_uc.history().commands.is_empty() {
        "No sessions".to_string()
    } else {
        term_uc
            .history()
            .commands
            .iter()
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

    let center = format!("$ {}\n\nOutput:\n  (no output)", tab_state.terminal_input);

    let right = format!(
        "History: {} commands\nSelected: {}\n\n[Enter] Execute\n[Ctrl+L] Clear\n[Up/Down] History",
        term_uc.history().commands.len(),
        tab_state
            .selected_history_index
            .map(|i| i.to_string())
            .unwrap_or_else(|| "-".to_string())
    );

    TabColumns::new(left, center, right)
}

/// Render CLI tab columns
pub(crate) fn render_cli_tab(state: &AppState) -> TabColumns {
    let tab_state = &state.cli_tab_state;

    let left = format!("Input:\n> {}", tab_state.command_input);
    let center = "Output:\n  (no output)".to_string();
    let right = format!(
        "History:\n  selected: {}",
        tab_state
            .selected_history_index
            .map(|i| i.to_string())
            .unwrap_or_else(|| "-".to_string())
    );

    TabColumns::new(left, center, right)
}
