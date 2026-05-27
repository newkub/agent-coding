---
title: Error Types Reference
description: Error codes และ handling
---

# Error Types Reference

Error types และ error handling ใน agent-tui

## Error Hierarchy

```
AppError (root)
├── ConfigurationError
├── DatabaseError
├── AIError
├── GitError
├── LSPError
├── MCPError
├── FileError
├── CommandError
├── NetworkError
└── UIError
```

## Error Types

### AppError

Root error type สำหรับ application

```rust
pub enum AppError {
    Configuration(ConfigurationError),
    Database(DatabaseError),
    AI(AIError),
    Git(GitError),
    LSP(LSPError),
    MCP(MCPError),
    File(FileError),
    Command(CommandError),
    Network(NetworkError),
    UI(UIError),
}
```

### ConfigurationError

Errors จาก configuration

```rust
pub enum ConfigurationError {
    ConfigNotFound(String),
    ConfigParseError(String),
    InvalidConfig(String),
    MissingConfig(String),
    EnvVarNotFound(String),
}
```

**Example**:
```rust
// Config file not found
ConfigurationError::ConfigNotFound("~/.agent-tui/config.toml".to_string())

// Invalid TOML
ConfigurationError::ConfigParseError("invalid TOML syntax".to_string())

// Missing required field
ConfigurationError::MissingConfig("ai.model".to_string())
```

### DatabaseError

Errors จาก database operations

```rust
pub enum DatabaseError {
    ConnectionError(String),
    QueryError(String),
    MigrationError(String),
    NotFound(String),
    ConstraintViolation(String),
    LockError(String),
}
```

**Example**:
```rust
// Connection failed
DatabaseError::ConnectionError("unable to connect to database".to_string())

// Query failed
DatabaseError::QueryError("SELECT failed: syntax error".to_string())

// Record not found
DatabaseError::NotFound("session abc123".to_string())
```

### AIError

Errors จาก AI providers

```rust
pub enum AIError {
    AuthenticationError(String),
    RateLimitError(String),
    InvalidRequest(String),
    ModelNotFound(String),
    TimeoutError(String),
    ProviderError(String),
    ResponseError(String),
}
```

**Example**:
```rust
// Invalid API key
AIError::AuthenticationError("invalid API key".to_string())

// Rate limited
AIError::RateLimitError("rate limit exceeded".to_string())

// Model not found
AIError::ModelNotFound("gpt-5".to_string())
```

### GitError

Errors จาก Git operations

```rust
pub enum GitError {
    NotAGitRepository(String),
    GitCommandError(String),
    AuthenticationError(String),
    MergeConflict(String),
    BranchNotFound(String),
    CommitError(String),
}
```

**Example**:
```rust
// Not a git repo
GitError::NotAGitRepository("/path/to/dir".to_string())

// Git command failed
GitError::GitCommandError("git status failed".to_string())

// Merge conflict
GitError::MergeConflict("conflict in main.rs".to_string())
```

### LSPError

Errors จาก LSP operations

```rust
pub enum LSPError {
    ServerNotStarted(String),
    ServerError(String),
    RequestError(String),
    TimeoutError(String),
    InvalidResponse(String),
}
```

**Example**:
```rust
// LSP server not running
LSPError::ServerNotStarted("rust-analyzer".to_string())

// Request timeout
LSPError::TimeoutError("hover request timed out".to_string())
```

### MCPError

Errors จาก MCP operations

```rust
pub enum MCPError {
    ConnectionError(String),
    AuthenticationError(String),
    RequestError(String),
    TimeoutError(String),
    InvalidResponse(String),
    ToolNotFound(String),
}
```

**Example**:
```rust
// MCP server connection failed
MCPError::ConnectionError("unable to connect to MCP server".to_string())

// Tool not found
MCPError::ToolNotFound("custom-tool".to_string())
```

### FileError

Errors จาก file operations

```rust
pub enum FileError {
    NotFound(String),
    PermissionDenied(String),
    IsDirectory(String),
    TooLarge(String),
    InvalidPath(String),
    ReadError(String),
    WriteError(String),
}
```

**Example**:
```rust
// File not found
FileError::NotFound("/path/to/file.rs".to_string())

// Permission denied
FileError::PermissionDenied("/etc/passwd".to_string())

// File too large
FileError::TooLarge("file exceeds 10MB limit".to_string())
```

### CommandError

Errors จาก command execution

```rust
pub enum CommandError {
    CommandNotFound(String),
    ExecutionError(String),
    TimeoutError(String),
    PermissionDenied(String),
    InvalidCommand(String),
}
```

**Example**:
```rust
// Command not found
CommandError::CommandNotFound("unknown-cmd".to_string())

// Execution failed
CommandError::ExecutionError("exit code 1".to_string())

// Timeout
CommandError::TimeoutError("command timed out".to_string())
```

### NetworkError

Errors จาก network operations

```rust
pub enum NetworkError {
    ConnectionError(String),
    TimeoutError(String),
    DNSError(String),
    SSLError(String),
    ProxyError(String),
}
```

**Example**:
```rust
// Connection failed
NetworkError::ConnectionError("unable to connect".to_string())

// DNS resolution failed
NetworkError::DNSError("host not found".to_string())

// SSL error
NetworkError::SSLError("certificate verify failed".to_string())
```

### UIError

Errors จาก UI operations

```rust
pub enum UIError {
    RenderError(String),
    InputError(String),
    ResizeError(String),
    TerminalError(String),
}
```

**Example**:
```rust
// Render failed
UIError::RenderError("unable to render frame".to_string())

// Terminal too small
UIError::TerminalError("terminal too small: 80x20".to_string())
```

## Error Handling

### Using `?` Operator

```rust
use agent_tui::shared::errors::AppError;

fn load_session(id: &str) -> Result<Session, AppError> {
    let session = database::get_session(id)?; // Returns AppError on failure
    Ok(session)
}
```

### Custom Error Messages

```rust
use agent_tui::shared::errors::AppError;

fn validate_config(config: &Config) -> Result<(), AppError> {
    if config.api_key.is_empty() {
        return Err(AppError::Configuration(
            ConfigurationError::MissingConfig("ai.api_key".to_string())
        ));
    }
    Ok(())
}
```

### Error Conversion

```rust
use agent_tui::shared::errors::AppError;

fn from_sqlx_error(err: sqlx::Error) -> AppError {
    match err {
        sqlx::Error::Database(_) => AppError::Database(
            DatabaseError::QueryError(err.to_string())
        ),
        sqlx::Error::PoolTimedOut => AppError::Database(
            DatabaseError::ConnectionError(err.to_string())
        ),
        _ => AppError::Database(
            DatabaseError::QueryError(err.to_string())
        ),
    }
}
```

## Error Display

### User-Friendly Messages

```rust
impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AppError::Configuration(e) => write!(f, "Configuration error: {}", e),
            AppError::Database(e) => write!(f, "Database error: {}", e),
            AppError::AI(e) => write!(f, "AI error: {}", e),
            // ... other variants
        }
    }
}
```

### Error Context

```rust
use anyhow::Context;

fn load_config() -> Result<Config, AppError> {
    let content = std::fs::read_to_string("config.toml")
        .context("failed to read config file")
        .map_err(|e| AppError::Configuration(
            ConfigurationError::ConfigNotFound(e.to_string())
        ))?;
    
    // ... parse config
}
```

## Error Codes

| Error Type | Code | HTTP Status |
|------------|------|-------------|
| ConfigurationError | `CONFIG_001` | 400 |
| DatabaseError | `DB_001` | 500 |
| AIError | `AI_001` | 502 |
| GitError | `GIT_001` | 500 |
| LSPError | `LSP_001` | 503 |
| MCPError | `MCP_001` | 502 |
| FileError | `FILE_001` | 404 |
| CommandError | `CMD_001` | 500 |
| NetworkError | `NET_001` | 502 |
| UIError | `UI_001` | 500 |

## Best Practices

1. **Use specific error types** - ใช้ error type ที่เฉพาะเจาะจง
2. **Provide context** - เพิ่ม context เมื่อ convert errors
3. **Handle gracefully** - handle errors อย่างเหมาะสมตาม context
4. **Log errors** - log errors สำหรับ debugging
5. **User-friendly messages** - แสดง error messages ที่เข้าใจง่าย

## Location

Error types อยู่ใน:
- `src/shared/errors.rs` - Root error types
- `src/modules/*/domain/errors/` - Module-specific errors
