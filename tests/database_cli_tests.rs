use std::path::Path;
use std::process::{Command, Output};

fn run_agent_tui(args: &[&str], database_path: &Path, workdir: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_agent-tui"))
        .args(args)
        .env("AGENT_TUI_DB_PATH", database_path)
        .current_dir(workdir)
        .output()
        .expect("failed to run agent-tui")
}

#[test]
fn test_database_backup_restore_and_verify_e2e() {
    let temp = tempfile::tempdir().unwrap();
    let database = temp.path().join("agent-tui.db");
    let backup = temp.path().join("backup.db");
    let restored = temp.path().join("restored.db");

    let output = run_agent_tui(&["session", "list"], &database, temp.path());
    assert!(
        output.status.success(),
        "session list failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let output = run_agent_tui(&["database", "verify"], &database, temp.path());
    assert!(
        output.status.success(),
        "verify failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let backup_arg = backup.to_string_lossy().to_string();
    let output = run_agent_tui(&["database", "backup", &backup_arg], &database, temp.path());
    assert!(
        output.status.success(),
        "backup failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(backup.exists());

    let output = run_agent_tui(
        &["database", "restore", &backup_arg, "--yes"],
        &restored,
        temp.path(),
    );
    assert!(
        output.status.success(),
        "restore failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let output = run_agent_tui(&["database", "verify"], &restored, temp.path());
    assert!(
        output.status.success(),
        "restored verify failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}
