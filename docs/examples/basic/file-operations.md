---
title: File Operations Example
description: File operations examples
---

# File Operations Example

## Overview

ตัวอย่างการจัดการ files ใน agent-tui

## Read File

### TUI

ใน TUI chat, พิมพ์:

```
Read the file src/main.rs
```

### Programmatic

```rust
use agent_tui::adapters::external::file_operations;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = file_operations::read_file("src/main.rs").await?;
    println!("{}", content);
    
    Ok(())
}
```

## Write File

### TUI

ใน TUI chat, พิมพ์:

```
Write "Hello World" to test.txt
```

### Programmatic

```rust
use agent_tui::adapters::external::file_operations;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    file_operations::write_file("test.txt", "Hello World").await?;
    println!("File written");
    
    Ok(())
}
```

## List Files

### TUI

ใน TUI chat, พิมพ์:

```
List all files in src/
```

### Programmatic

```rust
use agent_tui::adapters::external::file_operations;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let files = file_operations::list_files("src/").await?;
    
    for file in files {
        println!("{}", file);
    }
    
    Ok(())
}
```

## Search Files

### TUI

ใน TUI chat, พิมพ์:

```
Search for function names in src/
```

### Programmatic

```rust
use agent_tui::adapters::external::file_operations;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let results = file_operations::search_files("fn ", "src/").await?;
    
    for result in results {
        println!("{}: {}", result.path, result.line);
    }
    
    Ok(())
}
```

## Delete File

### TUI

ใน TUI chat, พิมพ์:

```
Delete test.txt
```

### Programmatic

```rust
use agent_tui::adapters::external::file_operations;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    file_operations::delete_file("test.txt").await?;
    println!("File deleted");
    
    Ok(())
}
```

## Security Notes

- Path validation ถูกใช้โดย default
- Directory traversal ถูก block
- File operations ถูก log

## Expected Output

```
File written: test.txt
Files in src/:
  main.rs
  lib.rs
  mod.rs
```
