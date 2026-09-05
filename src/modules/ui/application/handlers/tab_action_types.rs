/// Tab-specific actions
#[derive(Debug, Clone)]
pub enum TabAction {
    // Common
    Select,
    Execute,
    Edit(usize, String),
    Delete,
    Refresh,
    Toggle(usize),
    Add(String),
    Remove(usize),
    Clear,
    Filter(String),
    Input(String),
    Install(String),
    Uninstall(String),

    // Agent specific
    SendMessage(String),
    StartSession,
    EndSession,

    // Git specific
    Stage,
    Unstage,
    Commit(String),
    Push,
    Pull,

    // CLI specific
    RunCommand(String),
    ClearOutput,
    SearchHistory(String),

    // Snippet specific
    CreateSnippet,
    SaveSnippet,
    RunSnippet,

    // Skills specific
    LoadSkill,
    RunSkill,

    // Workflows specific
    RunWorkflow,
    StopWorkflow,

    // Files specific
    OpenFile,
    CreateFile,
    RenameFile,

    // Settings specific
    ApplySettings,
    ResetDefaults,

    // Collaboration specific
    Create,
    Join,
    Leave,

    // Macros specific
    StartRecording,
    StopRecording,
    Playback,

    // Headless session specific (Terminal tab)
    ListSessions,
    CreateSession,
    DeleteSession,
    LoadSession,
    SaveSession,

    // Performance specific (System tab)
    Snapshot,
}
