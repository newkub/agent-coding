---
title: Session Management Example
description: Session management examples
---

# Session Management Example

## Overview

ตัวอย่างการจัดการ sessions ใน agent-tui

## Create Session

### CLI

```bash
agent-tui create-session --name "coding-session"
```

### Programmatic

```rust
use agent_tui::presentation::di::DIContainer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let container = DIContainer::new();
    let use_case = container.create_session_use_case();
    
    let session = use_case.execute("coding-session".to_string()).await?;
    
    println!("Session created: {}", session.name);
    println!("Session ID: {}", session.id);
    
    Ok(())
}
```

## List Sessions

### CLI

```bash
agent-tui list-sessions
```

### Programmatic

```rust
use agent_tui::presentation::di::DIContainer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let container = DIContainer::new();
    let use_case = container.list_sessions_use_case();
    
    let sessions = use_case.execute().await?;
    
    for session in sessions {
        println!("{}: {}", session.name, session.id);
    }
    
    Ok(())
}
```

## Switch Session

### TUI

กด `Ctrl+L` เพื่อ list sessions
เลือก session ที่ต้องการด้วย arrow keys
กด Enter เพื่อ switch

### Programmatic

```rust
use agent_tui::presentation::di::DIContainer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let container = DIContainer::new();
    let use_case = container.switch_session_use_case();
    
    let session_id = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000")?;
    use_case.execute(session_id).await?;
    
    println!("Switched to session");
    
    Ok(())
}
```

## Delete Session

### CLI

```bash
agent-tui delete-session --id <session-id>
```

### Programmatic

```rust
use agent_tui::presentation::di::DIContainer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let container = DIContainer::new();
    let use_case = container.delete_session_use_case();
    
    let session_id = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000")?;
    use_case.execute(session_id).await?;
    
    println!("Session deleted");
    
    Ok(())
}
```

## Session with Agent Configuration

```rust
use agent_tui::presentation::di::DIContainer;
use agent_tui::modules::agent::domain::models::AgentConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let container = DIContainer::new();
    let use_case = container.create_session_use_case();
    
    let agent_config = AgentConfig {
        provider: "openai".to_string(),
        model: "gpt-4".to_string(),
        temperature: 0.7,
        max_tokens: 4096,
        system_prompt: Some("You are a helpful coding assistant".to_string()),
    };
    
    let session = use_case
        .with_agent(agent_config)
        .execute("coding-session".to_string())
        .await?;
    
    println!("Session created with agent config");
    
    Ok(())
}
```

## Expected Output

```
Session created: coding-session
Session ID: 550e8400-e29b-41d4-a716-446655440000
Created at: 2024-01-01 12:00:00 UTC
```
