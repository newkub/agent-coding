//! Tab and Column enum tests

use agent_tui::shared::kernel::types::{Tab, Column};

#[test]
fn test_tab_all() {
    let tabs = Tab::all();
    assert_eq!(tabs.len(), 17);
    assert!(tabs.contains(&Tab::Agent));
    assert!(tabs.contains(&Tab::Git));
    assert!(tabs.contains(&Tab::Cli));
    assert!(tabs.contains(&Tab::Snippet));
    assert!(tabs.contains(&Tab::Snippets));
    assert!(tabs.contains(&Tab::Skills));
    assert!(tabs.contains(&Tab::Workflows));
    assert!(tabs.contains(&Tab::Files));
    assert!(tabs.contains(&Tab::Settings));
    assert!(tabs.contains(&Tab::Packages));
    assert!(tabs.contains(&Tab::Terminal));
    assert!(tabs.contains(&Tab::Api));
    assert!(tabs.contains(&Tab::Database));
    assert!(tabs.contains(&Tab::Tasks));
    assert!(tabs.contains(&Tab::Notes));
    assert!(tabs.contains(&Tab::Logs));
    assert!(tabs.contains(&Tab::System));
}

#[test]
fn test_tab_next() {
    assert_eq!(Tab::Agent.next(), Tab::Packages);
    assert_eq!(Tab::Packages.next(), Tab::Files);
    assert_eq!(Tab::Files.next(), Tab::Git);
    assert_eq!(Tab::Git.next(), Tab::Terminal);
    assert_eq!(Tab::Terminal.next(), Tab::Snippet);
    assert_eq!(Tab::Snippet.next(), Tab::Snippets);
    assert_eq!(Tab::Snippets.next(), Tab::Api);
    assert_eq!(Tab::Api.next(), Tab::Database);
    assert_eq!(Tab::Database.next(), Tab::Tasks);
    assert_eq!(Tab::Tasks.next(), Tab::Notes);
    assert_eq!(Tab::Notes.next(), Tab::Logs);
    assert_eq!(Tab::Logs.next(), Tab::System);
    assert_eq!(Tab::System.next(), Tab::Skills);
    assert_eq!(Tab::Skills.next(), Tab::Workflows);
    assert_eq!(Tab::Workflows.next(), Tab::Settings);
    assert_eq!(Tab::Settings.next(), Tab::Cli);
    assert_eq!(Tab::Cli.next(), Tab::Agent);
}

#[test]
fn test_tab_prev() {
    assert_eq!(Tab::Agent.prev(), Tab::Cli);
    assert_eq!(Tab::Packages.prev(), Tab::Agent);
    assert_eq!(Tab::Files.prev(), Tab::Packages);
    assert_eq!(Tab::Git.prev(), Tab::Files);
    assert_eq!(Tab::Terminal.prev(), Tab::Git);
    assert_eq!(Tab::Snippet.prev(), Tab::Terminal);
    assert_eq!(Tab::Snippets.prev(), Tab::Snippet);
    assert_eq!(Tab::Api.prev(), Tab::Snippets);
    assert_eq!(Tab::Database.prev(), Tab::Api);
    assert_eq!(Tab::Tasks.prev(), Tab::Database);
    assert_eq!(Tab::Notes.prev(), Tab::Tasks);
    assert_eq!(Tab::Logs.prev(), Tab::Notes);
    assert_eq!(Tab::System.prev(), Tab::Logs);
    assert_eq!(Tab::Skills.prev(), Tab::System);
    assert_eq!(Tab::Workflows.prev(), Tab::Skills);
    assert_eq!(Tab::Settings.prev(), Tab::Workflows);
    assert_eq!(Tab::Cli.prev(), Tab::Settings);
}

#[test]
fn test_tab_label() {
    assert_eq!(Tab::Agent.label(), "Agent");
    assert_eq!(Tab::Git.label(), "Git");
    assert_eq!(Tab::Cli.label(), "CLI");
}

#[test]
fn test_column_all() {
    let cols = Column::all();
    assert_eq!(cols.len(), 3);
    assert!(cols.contains(&Column::Left));
    assert!(cols.contains(&Column::Center));
    assert!(cols.contains(&Column::Right));
}

#[test]
fn test_column_next() {
    assert_eq!(Column::Left.next(), Column::Center);
    assert_eq!(Column::Center.next(), Column::Right);
    assert_eq!(Column::Right.next(), Column::Left);
}

#[test]
fn test_column_prev() {
    assert_eq!(Column::Left.prev(), Column::Right);
    assert_eq!(Column::Center.prev(), Column::Left);
    assert_eq!(Column::Right.prev(), Column::Center);
}

#[test]
fn test_column_label() {
    assert_eq!(Column::Left.label(), "Left");
    assert_eq!(Column::Center.label(), "Center");
    assert_eq!(Column::Right.label(), "Right");
}
