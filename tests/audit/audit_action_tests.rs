//! Audit Action tests

use agent_tui::modules::audit::domain::models::AuditAction;

#[test]
fn test_audit_action_category_file() {
    assert_eq!(
        AuditAction::FileRead { path: "test.txt".to_string() }.category(),
        "file"
    );
    assert_eq!(
        AuditAction::FileWrite { path: "test.txt".to_string() }.category(),
        "file"
    );
    assert_eq!(
        AuditAction::FileDelete { path: "test.txt".to_string() }.category(),
        "file"
    );
}

#[test]
fn test_audit_action_category_command() {
    assert_eq!(
        AuditAction::CommandExecute { command: "ls".to_string() }.category(),
        "command"
    );
    assert_eq!(
        AuditAction::CommandApprove { command: "ls".to_string() }.category(),
        "command"
    );
    assert_eq!(
        AuditAction::CommandReject { command: "ls".to_string() }.category(),
        "command"
    );
}

#[test]
fn test_audit_action_category_git() {
    assert_eq!(
        AuditAction::GitCommit { message: "test".to_string(), files: vec![] }.category(),
        "git"
    );
    assert_eq!(AuditAction::GitPush.category(), "git");
    assert_eq!(
        AuditAction::GitBranch { name: "main".to_string() }.category(),
        "git"
    );
}

#[test]
fn test_audit_action_category_session() {
    assert_eq!(
        AuditAction::SessionCreate { name: "test".to_string() }.category(),
        "session"
    );
    assert_eq!(
        AuditAction::SessionDelete { id: "123".to_string() }.category(),
        "session"
    );
    assert_eq!(
        AuditAction::MessageSend { session_id: "123".to_string() }.category(),
        "session"
    );
}

#[test]
fn test_audit_action_category_ai() {
    assert_eq!(AuditAction::AiRequest { model: "gpt-4".to_string(), tokens: 100 }.category(), "ai");
}

#[test]
fn test_audit_action_category_system() {
    assert_eq!(AuditAction::ConfigChange { key: "test".to_string() }.category(), "system");
    assert_eq!(AuditAction::PluginLoad { name: "plugin".to_string() }.category(), "system");
}
