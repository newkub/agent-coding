use super::command::Command;
use crate::shared::kernel::types::Tab;

/// Get available commands for command palette
pub fn get_available_commands() -> Vec<Command> {
    vec![
        Command::new(
            "Switch to Agent Tab",
            Some("1"),
            "Switch to AI assistant tab",
        ),
        Command::new(
            "Switch to Packages Tab",
            Some("2"),
            "Switch to package manager tab",
        ),
        Command::new(
            "Switch to Files Tab",
            Some("3"),
            "Switch to file explorer tab",
        ),
        Command::new(
            "Switch to Git Tab",
            Some("4"),
            "Switch to Git operations tab",
        ),
        Command::new(
            "Switch to Terminal Tab",
            Some("5"),
            "Switch to terminal tab",
        ),
        Command::new(
            "Switch to Snippets Tab",
            Some("6"),
            "Switch to code snippets tab",
        ),
        Command::new("Switch to API Tab", Some("7"), "Switch to API testing tab"),
        Command::new(
            "Switch to Database Tab",
            Some("8"),
            "Switch to database tab",
        ),
        Command::new(
            "Switch to Tasks Tab",
            Some("9"),
            "Switch to task management tab",
        ),
        Command::new("Switch to Notes Tab", Some("0"), "Switch to notes tab"),
        Command::new("Switch to Logs Tab", Some("-"), "Switch to logs viewer tab"),
        Command::new(
            "Switch to System Tab",
            Some("="),
            "Switch to system monitoring tab",
        ),
        Command::new(
            "Toggle Focus Mode",
            Some("f"),
            "Toggle focus mode for current column",
        ),
        Command::new(
            "Save Current State",
            Some("Ctrl+S"),
            "Save current application state",
        ),
        Command::new(
            "Refresh/Reload",
            Some("Ctrl+R"),
            "Refresh current view or reload data",
        ),
        Command::new("Quit Application", Some("q"), "Exit the application"),
    ]
}

/// Get tab-specific commands for command palette
pub fn get_tab_specific_commands(tab: Tab) -> Vec<Command> {
    let mut commands = match tab {
        Tab::Agent => vec![
            Command::new("Skills", None, "Browse and use skills"),
            Command::new("Workflows", None, "Manage workflows"),
            Command::new("New Session", None, "Start new agent session"),
            Command::new("Clear Chat", None, "Clear chat history"),
            Command::new("Export Chat", None, "Export chat to file"),
            Command::new("Settings", None, "Agent settings"),
        ],
        Tab::Packages => vec![
            Command::new("Search Packages", None, "Search for packages"),
            Command::new("Install", None, "Install selected package"),
            Command::new("Uninstall", None, "Uninstall selected package"),
            Command::new("Update All", None, "Update all packages"),
            Command::new("Settings", None, "Package manager settings"),
        ],
        Tab::Files => vec![
            Command::new("Quick Open", Some("Ctrl+P"), "Quick file open"),
            Command::new("New File", None, "Create new file"),
            Command::new("Delete File", None, "Delete selected file"),
            Command::new("Copy Path", None, "Copy file path"),
            Command::new("Git Status", None, "Show git status"),
            Command::new("Settings", None, "File explorer settings"),
        ],
        Tab::Git => vec![
            Command::new("Branch", None, "Branch operations"),
            Command::new("Commit", None, "Commit changes"),
            Command::new("Push", None, "Push to remote"),
            Command::new("Pull", None, "Pull from remote"),
            Command::new("Merge", None, "Merge branches"),
            Command::new("Settings", None, "Git settings"),
        ],
        Tab::Terminal => vec![
            Command::new("New Terminal", None, "Open new terminal session"),
            Command::new("Split", None, "Split terminal"),
            Command::new("Clear", Some("Ctrl+L"), "Clear terminal"),
            Command::new("Kill Process", None, "Kill current process"),
            Command::new("Settings", None, "Terminal settings"),
        ],
        Tab::Snippets => vec![
            Command::new("New Snippet", None, "Create new snippet"),
            Command::new("Search", None, "Search snippets"),
            Command::new("Import", None, "Import snippets"),
            Command::new("Export", None, "Export snippets"),
            Command::new("Settings", None, "Snippet settings"),
        ],
        Tab::Api => vec![
            Command::new("New Request", None, "Create new API request"),
            Command::new("Save Collection", None, "Save request collection"),
            Command::new("Import", None, "Import collection"),
            Command::new("Export", None, "Export collection"),
            Command::new("Settings", None, "API settings"),
        ],
        Tab::Database => vec![
            Command::new("New Connection", None, "Add database connection"),
            Command::new("Run Query", None, "Execute query"),
            Command::new("Export Data", None, "Export query results"),
            Command::new("Settings", None, "Database settings"),
        ],
        Tab::Tasks => vec![
            Command::new("New Task", None, "Create new task"),
            Command::new("Filter", None, "Filter tasks"),
            Command::new("Complete", None, "Mark task as complete"),
            Command::new("Settings", None, "Task settings"),
        ],
        Tab::Notes => vec![
            Command::new("New Note", None, "Create new note"),
            Command::new("Search", None, "Search notes"),
            Command::new("Export", None, "Export notes"),
            Command::new("Settings", None, "Note settings"),
        ],
        Tab::Logs => vec![
            Command::new("Add Source", None, "Add log source"),
            Command::new("Filter", None, "Filter logs"),
            Command::new("Export", None, "Export logs"),
            Command::new("Settings", None, "Log settings"),
        ],
        Tab::System => vec![
            Command::new("Refresh", Some("Ctrl+R"), "Refresh system info"),
            Command::new("Kill Process", None, "Kill selected process"),
            Command::new("Alerts", None, "Configure alerts"),
            Command::new("Settings", None, "System settings"),
        ],
        Tab::Snippet => vec![
            Command::new("New Snippet", None, "Create new snippet"),
            Command::new("Search", None, "Search snippets"),
            Command::new("Import", None, "Import snippets"),
            Command::new("Export", None, "Export snippets"),
            Command::new("Settings", None, "Snippet settings"),
        ],
        Tab::Skills => vec![
            Command::new("Browse Skills", None, "Browse available skills"),
            Command::new("Search", None, "Search skills"),
            Command::new("Settings", None, "Skills settings"),
        ],
        Tab::Workflows => vec![
            Command::new("New Workflow", None, "Create new workflow"),
            Command::new("Run", None, "Run workflow"),
            Command::new("Settings", None, "Workflow settings"),
        ],
        Tab::Settings => vec![
            Command::new("General", None, "General settings"),
            Command::new("Appearance", None, "Appearance settings"),
            Command::new("Keybindings", None, "Keybindings"),
        ],
        Tab::Cli => vec![
            Command::new("Execute", None, "Execute CLI command"),
            Command::new("History", None, "Command history"),
            Command::new("Settings", None, "CLI settings"),
        ],
    };

    // Add custom commands to all tabs
    commands.push(Command::new(
        "Custom Commands",
        None,
        "Manage custom commands",
    ));

    commands
}

/// Search commands by query
pub fn search_commands(query: &str) -> Vec<Command> {
    let commands = get_available_commands();
    if query.is_empty() {
        return commands;
    }

    let query_lower = query.to_lowercase();
    commands
        .into_iter()
        .filter(|cmd| {
            cmd.name.to_lowercase().contains(&query_lower)
                || cmd.description.to_lowercase().contains(&query_lower)
                || cmd
                    .shortcut
                    .as_ref()
                    .is_some_and(|s| s.to_lowercase().contains(&query_lower))
        })
        .collect()
}
