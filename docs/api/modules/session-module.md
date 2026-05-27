---
title: Session Module API
description: Session module APIs แล exports
---

# Session Module API

## Overview

Session module จัดการ session persistence แล context management

## Domain Models

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

pub enum MessageRole {
    User,
    Assistant,
    System,
}
```

## Domain Operations

### Create Session

```rust
pub fn create_session(name: String) -> Result<Session, ValidationError>
```

**Parameters**:
- `name`: Session name

**Returns**:
- `Ok(Session)`: Created session
- `Err(ValidationError)`: Validation error

**Example**:
```rust
let session = create_session("my-session".to_string())?;
```

### Add Message

```rust
pub fn add_message(session: Session, message: Message) -> Session
```

**Parameters**:
- `session`: Current session
- `message`: Message to add

**Returns**:
- `Session`: Updated session

**Example**:
```rust
let updated_session = add_message(session, message);
```

## Application Use Cases

### CreateSessionUseCase

```rust
pub struct CreateSessionUseCase {
    session_repository: Arc<dyn SessionRepository>,
}

impl CreateSessionUseCase {
    pub async fn execute(&self, name: String) -> Result<Session>
}
```

**Example**:
```rust
let use_case = container.create_session_use_case();
let session = use_case.execute("my-session".to_string()).await?;
```

### SendMessageUseCase

```rust
pub struct SendMessageUseCase {
    session_repository: Arc<dyn SessionRepository>,
    ai_provider: Arc<dyn AIProvider>,
}

impl SendMessageUseCase {
    pub async fn execute(&self, session_id: Uuid, content: String) -> Result<Message>
}
```

**Example**:
```rust
let use_case = container.send_message_use_case();
let message = use_case.execute(session_id, "Hello AI".to_string()).await?;
```

## Ports

### SessionRepository

```rust
#[async_trait]
pub trait SessionRepository: Send + Sync {
    async fn save(&self, session: Session) -> Result<()>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Session>>;
    async fn find_all(&self) -> Result<Vec<Session>>;
    async fn delete(&self, id: Uuid) -> Result<()>;
    async fn update(&self, session: Session) -> Result<()>;
}
```

## Adapters

### SqliteSessionRepository

```rust
pub struct SqliteSessionRepository {
    pool: SqlitePool,
}

impl SessionRepository for SqliteSessionRepository {
    async fn save(&self, session: Session) -> Result<()> {
        // SQLite implementation
    }
    // ... other methods
}
```

## Usage Examples

### Create Session

```rust
use agent_tui::modules::session::application::usecases::CreateSessionUseCase;

let use_case = container.create_session_use_case();
let session = use_case.execute("coding-session".to_string()).await?;
```

### Send Message

```rust
use agent_tui::modules::session::application::usecases::SendMessageUseCase;

let use_case = container.send_message_use_case();
let message = use_case.execute(session_id, "Explain Rust ownership".to_string()).await?;
```

### List Sessions

```rust
use agent_tui::modules::session::application::usecases::ListSessionsUseCase;

let use_case = container.list_sessions_use_case();
let sessions = use_case.execute().await?;
```

## Error Types

### ValidationError

```rust
pub enum ValidationError {
    EmptyName,
    NameTooLong,
    InvalidCharacters,
}
```

### SessionError

```rust
pub enum SessionError {
    NotFound,
    Validation(ValidationError),
    Database(DbError),
}
```
