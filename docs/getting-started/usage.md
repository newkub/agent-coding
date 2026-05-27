---
title: Usage
description: วิธีใช้งานพื้นฐานและ examples
---

# Usage

## Method 1: Usage via CLI

### Basic Commands

```bash
# Run TUI interface
agent-tui run

# Run with specific session
agent-tui run --session <session-id>

# Run with specific agent
agent-tui run --agent <agent-id>

# List available sessions
agent-tui list-sessions

# List available agents
agent-tui list-agents

# Create new session
agent-tui create-session --name "my-session"

# Show version
agent-tui version

# Show help
agent-tui --help
```

### TUI Interface Controls

เมื่ออยู่ใน TUI mode:

- **Type**: Enter message ใน input field
- **Enter**: Send message ไปยัง AI
- **Ctrl+C**: Exit application
- **Ctrl+S**: Save current session
- **Ctrl+N**: Create new session
- **Ctrl+L**: List sessions
- **Tab**: Switch ระหว่าง panels
- **Esc**: Cancel current operation หรือ go back
- **?**: Show help menu

### Session Management

```bash
# CLI: Create session
agent-tui create-session --name "project-review"

# CLI: List sessions
agent-tui list-sessions

# TUI: Use Ctrl+N to create new session
# TUI: Use Ctrl+L to list and switch sessions
```

### File Operations

```bash
# In TUI chat, use natural language:
"Read the file src/main.rs"
"Write hello world to test.txt"
"List all files in src/"
"Search for function names in src/"
```

### Git Operations

```bash
# In TUI chat, use natural language:
"Show git status"
"Commit changes with message 'fix bug'"
"View diff of main.rs"
"Create new branch feature/login"
```

## Method 2: Usage via Programmatic API

### Basic Example

```rust
use agent_tui::presentation::di::DIContainer;
use agent_tui::modules::session::domain::models::Session;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create DI container
    let container = DIContainer::new();
    
    // Create session use case
    let create_session = container.create_session_use_case();
    let session = create_session.execute("my-session").await?;
    
    // Send message use case
    let send_message = container.send_message_use_case();
    let response = send_message.execute(session.id, "Hello AI").await?;
    
    println!("Response: {}", response.content);
    
    Ok(())
}
```

### Advanced Example

```rust
use agent_tui::presentation::di::DIContainer;
use agent_tui::modules::agent::domain::models::AgentConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let container = DIContainer::new();
    
    // Configure agent
    let agent_config = AgentConfig {
        provider: "openai".to_string(),
        model: "gpt-4".to_string(),
        temperature: 0.7,
        max_tokens: 4096,
    };
    
    // Create session with agent
    let create_session = container.create_session_use_case();
    let session = create_session
        .with_agent(agent_config)
        .execute("coding-session")
        .await?;
    
    // Send multiple messages
    let send_message = container.send_message_use_case();
    
    let response1 = send_message
        .execute(session.id, "Explain Rust ownership")
        .await?;
    
    let response2 = send_message
        .execute(session.id, "Give me an example")
        .await?;
    
    Ok(())
}
```

## Common Workflows

### 1. Code Review Workflow

```bash
# Start TUI
agent-tui run

# In TUI:
"Review the code in src/main.rs"
"What are the potential issues?"
"Suggest improvements"
```

### 2. Debugging Workflow

```bash
# Start TUI
agent-tui run

# In TUI:
"Read the error logs"
"Analyze the stack trace"
"Suggest fixes"
"Apply the fix to src/error.rs"
```

### 3. Feature Development Workflow

```bash
# Start TUI
agent-tui run

# In TUI:
"Create a new feature for user authentication"
"Design the data models"
"Implement the authentication logic"
"Write tests for the feature"
```

### 4. Git Workflow

```bash
# Start TUI
agent-tui run

# In TUI:
"Show git status"
"Review the changes"
"Commit with message 'feat: add authentication'"
"Push to remote"
```

## Keyboard Shortcuts

### Global Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl+C` | Exit application |
| `Ctrl+S` | Save current session |
| `Ctrl+N` | Create new session |
| `Ctrl+L` | List sessions |
| `Ctrl+P` | Previous message |
| `Ctrl+N` | Next message |
| `Ctrl+F` | Search in messages |
| `Ctrl+G` | Go to line |
| `?` | Show help menu |
| `Esc` | Cancel/Go back |

### Input Shortcuts

| Shortcut | Action |
|----------|--------|
| `Enter` | Send message |
| `Ctrl+K` | Clear input |
| `Ctrl+U` | Clear to start of line |
| `Ctrl+W` | Delete word |
| `Ctrl+A` | Go to start of line |
| `Ctrl+E` | Go to end of line |
| `Tab` | Autocomplete |

### Panel Shortcuts

| Shortcut | Action |
|----------|--------|
| `Tab` | Switch to next panel |
| `Shift+Tab` | Switch to previous panel |
| `Ctrl+1` | Switch to panel 1 |
| `Ctrl+2` | Switch to panel 2 |
| `Ctrl+3` | Switch to panel 3 |

## Tips and Tricks

### 1. Use Natural Language

agent-tui เข้าใจ natural language commands:

```bash
# Instead of:
"Read file src/main.rs"

# You can say:
"Show me the main file"
"What's in src/main.rs?"
"Display the contents of main.rs"
```

### 2. Chain Commands

```bash
# Chain multiple operations:
"Read src/main.rs, then explain the logic, then suggest improvements"
```

### 3. Use Context

```bash
# Reference previous messages:
"Based on the previous explanation, implement the fix"
"Use the same pattern for the next function"
```

### 4. Save Useful Responses

```bash
# Save responses for later:
"Save this explanation as a note"
"Bookmark this code snippet"
```

## Troubleshooting

### TUI Not Rendering

```bash
# Check terminal support
echo $TERM

# Try different terminal
# agent-tui works best with modern terminals:
# - iTerm2 (macOS)
# - Windows Terminal
# - GNOME Terminal
# - Alacritty
```

### Session Not Saving

```bash
# Check database path
# Verify write permissions
ls -la ~/.agent-tui/

# Check database integrity
sqlite3 ~/.agent-tui/sessions.db ".tables"
```

### AI Not Responding

```bash
# Check API key
echo $OPENAI_API_KEY

# Test connection
curl https://api.openai.com/v1/models

# Check logs
RUST_LOG=debug agent-tui run
```
