---
title: Domain Functions
description: Function signatures แล parameters
---

# Domain Functions

## Session Functions

### create_session

```rust
pub fn create_session(name: String) -> DomainResult<Session>
```

**Parameters**:
- `name: String` - Session name

**Returns**:
- `Ok(Session)` - Created session
- `Err(ValidationError)` - Validation error

**Constraints**:
- Name cannot be empty
- Name max length: 255 characters
- Name must be alphanumeric with spaces

### add_message

```rust
pub fn add_message(session: Session, message: Message) -> Session
```

**Parameters**:
- `session: Session` - Current session
- `message: Message` - Message to add

**Returns**:
- `Session` - Updated session

### update_session_name

```rust
pub fn update_session_name(session: Session, name: String) -> DomainResult<Session>
```

**Parameters**:
- `session: Session` - Current session
- `name: String` - New name

**Returns**:
- `Ok(Session)` - Updated session
- `Err(ValidationError)` - Validation error

## Message Functions

### create_message

```rust
pub fn create_message(
    session_id: Uuid,
    role: MessageRole,
    content: String,
) -> DomainResult<Message>
```

**Parameters**:
- `session_id: Uuid` - Session ID
- `role: MessageRole` - Message role
- `content: String` - Message content

**Returns**:
- `Ok(Message)` - Created message
- `Err(ValidationError)` - Validation error

**Constraints**:
- Content cannot be empty
- Content max length: 100,000 characters

## Agent Functions

### create_agent

```rust
pub fn create_agent(name: String, config: AgentConfig) -> DomainResult<Agent>
```

**Parameters**:
- `name: String` - Agent name
- `config: AgentConfig` - Agent configuration

**Returns**:
- `Ok(Agent)` - Created agent
- `Err(ValidationError)` - Validation error

### validate_agent_config

```rust
pub fn validate_agent_config(config: &AgentConfig) -> DomainResult<()>
```

**Parameters**:
- `config: &AgentConfig` - Agent configuration

**Returns**:
- `Ok(())` - Valid configuration
- `Err(ValidationError)` - Validation error

**Constraints**:
- Temperature: 0.0 - 2.0
- Max tokens: 1 - 128,000
- Provider must be supported
- Model must be valid for provider

## Validation Functions

### validate_name

```rust
pub fn validate_name(name: &str) -> DomainResult<()>
```

**Parameters**:
- `name: &str` - Name to validate

**Returns**:
- `Ok(())` - Valid name
- `Err(ValidationError)` - Validation error

### validate_content

```rust
pub fn validate_content(content: &str) -> DomainResult<()>
```

**Parameters**:
- `content: &str` - Content to validate

**Returns**:
- `Ok(())` - Valid content
- `Err(ValidationError)` - Validation error

### validate_path

```rust
pub fn validate_path(path: &str) -> DomainResult<()>
```

**Parameters**:
- `path: &str` - Path to validate

**Returns**:
- `Ok(())` - Valid path
- `Err(ValidationError)` - Validation error

**Security**:
- Prevents directory traversal
- Prevents absolute paths (configurable)
- Validates path format
