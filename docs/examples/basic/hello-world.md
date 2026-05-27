---
title: Hello World Example
description: Basic example สำหรับเริ่มต้นใช้งาน
---

# Hello World Example

## Overview

Basic example สำหรับเริ่มต้นใช้งาน agent-tui

## Prerequisites

- Rust 1.70+
- agent-tui installed

## Step 1: Run TUI

```bash
agent-tui run
```

## Step 2: Send First Message

ใน TUI interface, พิมพ์:

```
Hello, can you help me with Rust programming?
```

กด Enter เพื่อส่ง

## Step 3: View Response

AI จะตอบกลับด้วย streaming response

## Step 4: Save Session

กด `Ctrl+S` เพื่อบันทึก session

## Step 5: Exit

กด `Ctrl+C` เพื่อออกจาก application

## Code Example

หากต้องการใช้งานผ่าน code:

```rust
use agent_tui::presentation::di::DIContainer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create DI container
    let container = DIContainer::new();
    
    // Create session
    let create_session = container.create_session_use_case();
    let session = create_session.execute("hello-world".to_string()).await?;
    
    println!("Created session: {}", session.name);
    
    Ok(())
}
```

## Expected Output

```
Created session: hello-world
Session ID: 550e8400-e29b-41d4-a716-446655440000
```

## Next Steps

- ดู [Session Management Example](./session-management.md)
- ดู [File Operations Example](./file-operations.md)
- ดู [Git Integration Example](./git-integration.md)
