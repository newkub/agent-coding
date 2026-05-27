---
title: Constants Reference
description: Constants และ enums ทั้งหมด
---

# Constants Reference

Constants และ enums ที่ใช้ใน agent-tui

## Application Constants

### Version

```rust
pub const VERSION: &str = "0.1.0";
pub const NAME: &str = "agent-tui";
```

### Paths

```rust
pub const DEFAULT_CONFIG_DIR: &str = ".agent-tui";
pub const DEFAULT_CONFIG_FILE: &str = "config.toml";
pub const DEFAULT_DATABASE_FILE: &str = "sessions.db";
pub const DEFAULT_LOG_FILE: &str = "agent-tui.log";
pub const DEFAULT_CACHE_DIR: &str = "cache";
```

## AI Provider Constants

### OpenAI

```rust
pub const OPENAI_DEFAULT_MODEL: &str = "gpt-4";
pub const OPENAI_DEFAULT_TEMPERATURE: f32 = 0.7;
pub const OPENAI_DEFAULT_MAX_TOKENS: u32 = 4096;
pub const OPENAI_BASE_URL: &str = "https://api.openai.com/v1";
```

### Anthropic

```rust
pub const ANTHROPIC_DEFAULT_MODEL: &str = "claude-3-opus-20240229";
pub const ANTHROPIC_DEFAULT_TEMPERATURE: f32 = 0.7;
pub const ANTHROPIC_DEFAULT_MAX_TOKENS: u32 = 4096;
pub const ANTHROPIC_BASE_URL: &str = "https://api.anthropic.com";
```

### Google

```rust
pub const GOOGLE_DEFAULT_MODEL: &str = "gemini-pro";
pub const GOOGLE_DEFAULT_TEMPERATURE: f32 = 0.7;
pub const GOOGLE_DEFAULT_MAX_TOKENS: u32 = 4096;
pub const GOOGLE_BASE_URL: &str = "https://generativelanguage.googleapis.com";
```

## Database Constants

```rust
pub const DATABASE_POOL_SIZE: u32 = 5;
pub const DATABASE_TIMEOUT_SECONDS: u64 = 30;
pub const DATABASE_AUTO_BACKUP: bool = true;
pub const DATABASE_BACKUP_INTERVAL_HOURS: u64 = 24;
```

## LSP Constants

```rust
pub const LSP_HOVER_DELAY_MS: u64 = 300;
pub const LSP_COMPLETION_TRIGGER_CHARS: &[char] = &['.', ':', '('];
pub const LSP_DEFAULT_TIMEOUT_SECONDS: u64 = 30;
```

## MCP Constants

```rust
pub const MCP_DEFAULT_TIMEOUT_SECONDS: u64 = 30;
pub const MCP_MAX_RETRIES: u32 = 3;
pub const MCP_RETRY_DELAY_MS: u64 = 1000;
```

## TUI Constants

### Colors

```rust
pub const COLOR_PRIMARY: Color = Color::Rgb(96, 165, 250);
pub const COLOR_SECONDARY: Color = Color::Rgb(139, 92, 246);
pub const COLOR_SUCCESS: Color = Color::Rgb(34, 197, 94);
pub const COLOR_WARNING: Color = Color::Rgb(251, 191, 36);
pub const COLOR_ERROR: Color = Color::Rgb(239, 68, 68);
pub const COLOR_INFO: Color = Color::Rgb(59, 130, 246);
```

### Layout

```rust
pub const MIN_WIDTH: u16 = 80;
pub const MIN_HEIGHT: u16 = 24;
pub const DEFAULT_WIDTH: u16 = 120;
pub const DEFAULT_HEIGHT: u16 = 40;
```

### Keyboard Shortcuts

```rust
pub const KEY_EXIT: Key = Key::Char('c').with_ctrl();
pub const KEY_SAVE: Key = Key::Char('s').with_ctrl();
pub const KEY_NEW_SESSION: Key = Key::Char('n').with_ctrl();
pub const KEY_LIST_SESSIONS: Key = Key::Char('l').with_ctrl();
pub const KEY_SWITCH_PANEL: Key = Key::Tab;
pub const KEY_CANCEL: Key = Key::Esc;
pub const KEY_HELP: Key = Key::Char('?');
```

## Security Constants

```rust
pub const MAX_FILE_SIZE_MB: u64 = 10;
pub const MAX_PATH_LENGTH: usize = 4096;
pub const ALLOWED_SCHEMES: &[&str] = &["file", "http", "https"];
pub const BLOCKED_PATTERNS: &[&str] = &["/etc", "/usr", "/sys", "/proc"];
```

## Cache Constants

```rust
pub const CACHE_ENABLED: bool = true;
pub const CACHE_TTL_SECONDS: u64 = 3600;
pub const CACHE_MAX_SIZE_MB: u64 = 1000;
pub const CACHE_CLEANUP_INTERVAL_HOURS: u64 = 24;
```

## Memory Constants

```rust
pub const MEMORY_ENABLED: bool = true;
pub const MEMORY_VECTOR_SIZE: usize = 1536;
pub const MEMORY_MAX_RESULTS: usize = 10;
pub const MEMORY_SIMILARITY_THRESHOLD: f32 = 0.7;
```

## Git Constants

```rust
pub const GIT_ENABLED: bool = true;
pub const GIT_AUTO_COMMIT: bool = false;
pub const GIT_COMMIT_TEMPLATE: &str = "feat: %s";
pub const GIT_DEFAULT_BRANCH: &str = "main";
```

## Network Constants

```rust
pub const REQUEST_TIMEOUT_SECONDS: u64 = 60;
pub const MAX_RETRIES: u32 = 3;
pub const RETRY_DELAY_MS: u64 = 1000;
pub const USER_AGENT: &str = "agent-tui/0.1.0";
```

## Enums

### LogLevel

```rust
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}
```

### Theme

```rust
pub enum Theme {
    Default,
    Dark,
    Light,
}
```

### Provider

```rust
pub enum Provider {
    OpenAI,
    Anthropic,
    Google,
    Custom(String),
}
```

### SessionStatus

```rust
pub enum SessionStatus {
    Active,
    Archived,
    Deleted,
}
```

### MessageRole

```rust
pub enum MessageRole {
    User,
    Assistant,
    System,
}
```

### FileType

```rust
pub enum FileType {
    Rust,
    JavaScript,
    TypeScript,
    Python,
    Go,
    Java,
    Markdown,
    Text,
    Binary,
}
```

## Usage Example

```rust
use agent_tui::shared::constants::*;

fn main() {
    // Use constants
    let config_dir = DEFAULT_CONFIG_DIR;
    let model = OPENAI_DEFAULT_MODEL;
    let timeout = DATABASE_TIMEOUT_SECONDS;
    
    // Use enums
    let log_level = LogLevel::Info;
    let theme = Theme::Dark;
    let provider = Provider::OpenAI;
}
```

## Location

Constants อยู่ใน:
- `src/shared/constants/` - Application constants
- `src/modules/*/domain/constants/` - Module-specific constants
