/// ANSI Diagram for all potential tabs
pub(crate) fn draw_all_tabs_diagram() -> String {
    let diagram = String::from(r#"
╔═══════════════════════════════════════════════════════════════════════════════╗
║                        AGENT-TUI: COMPLETE TAB ARCHITECTURE                     ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                                 ║
║  ┌─────────┬─────────┬─────────┬─────────┬─────────┬─────────┬─────────┬──────┐║
║  │ AGENT   │ GIT     │ CLI     │SNIPPET  │ SKILLS  │ WORKFLOW│ FILES   │SETINGS│║
║  ├─────────┴─────────┴─────────┴─────────┴─────────┴─────────┴─────────┴──────┤║
║  │                         (Potential Future Tabs)                              │║
║  │  ┌──────────┬──────────┬──────────┬──────────┬──────────┬───────────────┐  │║
║  │  │ SEARCH   │ HISTORY  │ NOTIFY   │ PLUGINS  │ ANALYTICS│ TERMINAL │DEBUG    │  │║
║  │  └──────────┴──────────┴──────────┴──────────┴──────────┴───────────────┘  │║
║  └────────────────────────────────────────────────────────────────────────────┘║
║                                                                                 ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                              CURRENT TABS (8)                                   ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                                 ║
║  ┌─ AGENT ─────────────────────────────────────────────────────────────────┐    ║
║  │  Context          │      Chat Area         │      Actions           │    ║
║  │  • Session ID     │  • Messages list      │  • [1] Execute         │    ║
║  │  • Variables      │  • Input field        │  • [2] Review          │    ║
║  │  • Environment    │  • Send button        │  • [3] Confirm         │    ║
║  └───────────────────┴────────────────────────┴───────────────────────────┘    ║
║                                                                                 ║
║  ┌─ GIT ───────────────────────────────────────────────────────────────────┐    ║
║  │  Status           │      Diff View         │      History            │    ║
║  │  • Branch name    │  • File changes       │  • Commit log          │    ║
║  │  • Staged files   │  • Hunk navigation    │  • Author info         │    ║
║  │  • Unstaged       │  • Stage/Reject       │  • Timeline view       │    ║
║  └───────────────────┴────────────────────────┴───────────────────────────┘    ║
║                                                                                 ║
║  ┌─ CLI ───────────────────────────────────────────────────────────────────┐    ║
║  │  Quick Commands    │      Output           │      History            │    ║
║  │  • cargo build     │  • Command result    │  • Recent commands     │    ║
║  │  • cargo test      │  • Error display     │  • Search (Ctrl+R)     │    ║
║  │  • cargo fmt       │  • Progress bar      │  • Favorites           │    ║
║  └───────────────────┴────────────────────────┴───────────────────────────┘    ║
║                                                                                 ║
║  ┌─ SNIPPET ──────────────────────────────────────────────────────────────┐    ║
║  │  Library          │      Editor           │      Tags               │    ║
║  │  • Categories     │  • Code editor       │  • Tag cloud           │    ║
║  │  • Search         │  • Syntax highlight   │  • Filter by tag       │    ║
║  │  • Recent         │  • Save/Load          │  • Popular tags        │    ║
║  └───────────────────┴────────────────────────┴───────────────────────────┘    ║
║                                                                                 ║
║  ┌─ SKILLS ───────────────────────────────────────────────────────────────┐    ║
║  │  Tree View         │      Detail           │      Progress           │    ║
║  │  • Stakpak         │  • Version, Trust     │  • Mastered: 8/10      │    ║
║  │  • Local           │  • Description        │  • In Progress: 4/10   │    ║
║  │  • Custom          │  • Usage guide        │  • Achievement badge   │    ║
║  └───────────────────┴────────────────────────┴───────────────────────────┘    ║
║                                                                                 ║
║  ┌─ WORKFLOWS ─────────────────────────────────────────────────────────────┐    ║
║  │  List             │      Editor           │      History            │    ║
║  │  • deploy         │  • YAML editor       │  • Run #42 (success)   │    ║
║  │  • test-all       │  • Step config       │  • Run #41 (success)   │    ║
║  │  • backup         │  • Validate          │  • Run #40 (failed)    │    ║
║  └───────────────────┴────────────────────────┴───────────────────────────┘    ║
║                                                                                 ║
║  ┌─ FILES ─────────────────────────────────────────────────────────────────┐    ║
║  │  Explorer         │      Content          │      Actions            │    ║
║  │  • Directory tree │  • File viewer       │  • Open, Edit          │    ║
║  │  • Search         │  • Line numbers      │  • Rename, Delete      │    ║
║  │  • Hidden files   │  • Syntax color      │  • Copy path           │    ║
║  └───────────────────┴────────────────────────┴───────────────────────────┘    ║
║                                                                                 ║
║  ┌─ SETTINGS ──────────────────────────────────────────────────────────────┐    ║
║  │  Categories        │      Options           │      Keybindings        │    ║
║  │  • General         │  • Theme selection    │  • Tab: Tab key        │    ║
║  │  • Display         │  • Font size          │  • Col: Arrow keys     │    ║
║  │  • Editor          │  • Transparency        │  • Focus: f key        │    ║
║  │  • Terminal        │  • Auto-save          │  • Quit: q key        │    ║
║  └───────────────────┴────────────────────────┴───────────────────────────┘    ║
║                                                                                 ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                           POTENTIAL FUTURE TABS (10)                           ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                                 ║
║  Priority 1 - High Value / Low Complexity:                                     ║
║  ┌─ SEARCH ───────────────────────────────────────────────────────────────┐    ║
║  │  Query Input     │      Results            │      Filters              │    ║
║  │  • Search bar    │  • File matches        │  • Type filter           │    ║
║  │  • Regex support│  • Command matches      │  • Date range            │    ║
║  │  • History       │  • Snippet matches     │  • Category filter       │    ║
║  └─────────────────┴─────────────────────────┴────────────────────────────┘    ║
║                                                                                 ║
║  ┌─ HISTORY ───────────────────────────────────────────────────────────────┐    ║
║  │  Timeline        │      Details            │      Actions              │    ║
║  │  • Today         │  • Entry details       │  • Replay action         │    ║
║  │  • Yesterday     │  • Diff preview        │  • Bookmark             │    ║
║  │  • This week     │  • Metadata            │  • Share                │    ║
║  └─────────────────┴─────────────────────────┴────────────────────────────┘    ║
║                                                                                 ║
║  ┌─ NOTIFICATIONS ─────────────────────────────────────────────────────────┐    ║
║  │  Categories      │      Notifications     │      Settings              │    ║
║  │  • Errors        │  • Alert messages     │  • Sound toggle          │    ║
║  │  • Warnings      │  • Info messages      │  • Duration              │    ║
║  │  • Info          │  • Success messages   │  • Channel config        │    ║
║  └─────────────────┴─────────────────────────┴────────────────────────────┘    ║
║                                                                                 ║
║  Priority 2 - Medium Value:                                                    ║
║  ┌─ TERMINAL ──────────────────────────────────────────────────────────────┐    ║
║  │  Sessions        │      Terminal           │      Output               │    ║
║  │  • Tabbed view   │  • Full PTY support    │  • Scrollback           │    ║
║  │  • New session   │  • Zsh/Bash           │  • Search               │    ║
║  │  • SSH connect   │  • Copy/Paste         │  • Export               │    ║
║  └─────────────────┴─────────────────────────┴────────────────────────────┘    ║
║                                                                                 ║
║  ┌─ BOOKMARKS ─────────────────────────────────────────────────────────────┐    ║
║  │  Folders         │      Bookmarks          │      Tags               │    ║
║  │  • Quick access  │  • Recent files        │  • Tag cloud           │    ║
║  │  • Favorites     │  • Commands           │  • Filter by tag       │    ║
║  │  • Recent        │  • Workflows          │  • Popular tags        │    ║
║  └─────────────────┴─────────────────────────┴────────────────────────────┘    ║
║                                                                                 ║
║  ┌─ TASKS ─────────────────────────────────────────────────────────────────┐    ║
║  │  Projects        │      Tasks             │      Details              │    ║
║  │  • Project list  │  • Todo items         │  • Description           │    ║
║  │  • Progress      │  • In progress        │  • Due date              │    ║
║  │  • Archive       │  • Completed          │  • Labels                │    ║
║  └─────────────────┴─────────────────────────┴────────────────────────────┘    ║
║                                                                                 ║
║  Priority 3 - Advanced Features:                                               ║
║  ┌─ ANALYTICS ─────────────────────────────────────────────────────────────┐    ║
║  │  Dashboard       │      Charts            │      Reports              │    ║
║  │  • Overview      │  • Productivity graph │  • Weekly report         │    ║
║  │  • Metrics       │  • Time spent         │  • Export PDF            │    ║
║  │  • Trends        │  • Commands used       │  • Share link            │    ║
║  └─────────────────┴─────────────────────────┴────────────────────────────┘    ║
║                                                                                 ║
║  ┌─ RESOURCES ─────────────────────────────────────────────────────────────┐    ║
║  │  Overview        │      Charts            │      Alerts              │    ║
║  │  • CPU, Memory   │  • Live graphs        │  • Threshold alerts     │    ║
║  │  • Disk usage    │  • Historical         │  • Auto-remediate      │    ║
║  │  • Network       │  • Comparison         │  • Notification        │    ║
║  └─────────────────┴─────────────────────────┴────────────────────────────┘    ║
║                                                                                 ║
║  ┌─ PLUGINS ────────────────────────────────────────────────────────────────┐    ║
║  │  Installed       │      Store             │      Config              │    ║
║  │  • Active plugins│  • Browse plugins     │  • Plugin settings     │    ║
║  │  • Update avail  │  • Search             │  • Enable/Disable      │    ║
║  │  • Config        │  • Install            │  • API keys            │    ║
║  └─────────────────┴─────────────────────────┴────────────────────────────┘    ║
║                                                                                 ║
║  ┌─ DEBUG ─────────────────────────────────────────────────────────────────┐    ║
║  │  Processes       │      Inspector         │      Console             │    ║
║  │  • Running proc  │  • Variable watch     │  • Debug output         │    ║
║  │  • Stack trace   │  • Breakpoints        │  • REPL                 │    ║
║  │  • Memory dump   │  • Call stack          │  • Profiler             │    ║
║  └─────────────────┴─────────────────────────┴────────────────────────────┘    ║
║                                                                                 ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                              KEYBOARD SHORTCUTS                                ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                                 ║
║  Navigation:                                                                   ║
║    [Tab]         Next Tab                                                      ║
║    [Shift+Tab]   Previous Tab                                                  ║
║    [→]           Next Column                                                   ║
║    [←]           Previous Column                                              ║
║                                                                                 ║
║  Focus:                                                                       ║
║    [f]           Toggle Focus Mode                                             ║
║    [Esc]         Clear Selection / Go Back                                     ║
║                                                                                 ║
║  Actions:                                                                     ║
║    [Enter]       Execute / Confirm                                             ║
║    [1-9]         Quick Actions                                                 ║
║    [Ctrl+S]      Save                                                          ║
║    [Ctrl+R]      Search / Refresh                                              ║
║                                                                                 ║
║  Quit:                                                                        ║
║    [q]           Quit Application                                              ║
║                                                                                 ║
╚═══════════════════════════════════════════════════════════════════════════════╝
"#);
    diagram
}
