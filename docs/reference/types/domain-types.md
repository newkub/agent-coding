---
title: Domain Types
description: Type definitions แล interfaces
---

# Domain Types

## Session Types

### Session

```rust
pub struct Session {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub messages: Vec<Message>,
    pub agent_config: Option<AgentConfig>,
}
```

### SessionId

```rust
pub type SessionId = Uuid;
```

## Message Types

### Message

```rust
pub struct Message {
    pub id: Uuid,
    pub session_id: Uuid,
    pub role: MessageRole,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub tokens: u32,
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

## Agent Types

### Agent

```rust
pub struct Agent {
    pub id: Uuid,
    pub name: String,
    pub provider: String,
    pub model: String,
    pub temperature: f32,
    pub max_tokens: u32,
    pub system_prompt: Option<String>,
}
```

### AgentConfig

```rust
pub struct AgentConfig {
    pub provider: String,
    pub model: String,
    pub temperature: f32,
    pub max_tokens: u32,
    pub system_prompt: Option<String>,
}
```

## Error Types

### ValidationError

```rust
pub enum ValidationError {
    EmptyName,
    NameTooLong,
    InvalidCharacters,
    InvalidProvider,
    InvalidModel,
    InvalidTemperature,
    InvalidMaxTokens,
}
```

### AppError

```rust
pub enum AppError {
    Validation(ValidationError),
    Database(DbError),
    AI(AIError),
    File(FileError),
    Git(GitError),
    LSP(LSPError),
    MCP(MCPError),
}
```

## Result Types

### DomainResult

```rust
pub type DomainResult<T> = Result<T, ValidationError>;
```

### AppResult

```rust
pub type AppResult<T> = Result<T, AppError>;
```

## Event Types

### SessionEvent

```rust
pub enum SessionEvent {
    Created(Session),
    Updated(Session),
    Deleted(SessionId),
    MessageAdded(SessionId, Message),
}
```

### AgentEvent

```rust
pub enum AgentEvent {
    Created(Agent),
    Updated(Agent),
    Deleted(AgentId),
}
```
