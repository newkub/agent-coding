---
title: Quick Start
description: เริ่มต้นใช้งาน agent-tui อย่างรวดเร็ว
---

# Quick Start

เริ่มต้นใช้งาน agent-tui ภายใน 5 นาที

## Prerequisites

- Rust 1.70 หรือใหม่กว่า
- Git (optional, สำหรับ Git integration)

## Installation

```bash
# Clone repository
git clone https://github.com/your-org/rust-packages.git
cd rust-packages/apps/agent-tui

# Build และ run
cargo run -- run
```

## First Run

เมื่อ run ครั้งแรก จะมีการ:

1. **สร้าง configuration file** ที่ `~/.agent-tui/config.toml`
2. **สร้าง database** ที่ `~/.agent-tui/sessions.db`
3. **ขอ API keys** สำหรับ AI providers

## Setup API Keys

สร้าง `.env` file ใน project root:

```bash
OPENAI_API_KEY=your_openai_api_key
ANTHROPIC_API_KEY=your_anthropic_api_key
```

## Basic Usage

### 1. Start TUI

```bash
cargo run -- run
```

### 2. Create Session

กด `Ctrl+N` ใน TUI เพื่อสร้าง session ใหม่

### 3. Chat with AI

พิมพ์ข้อความและกด `Enter` เพื่อส่ง:

```
Hello! I need help with Rust code.
```

### 4. File Operations

ใช้ natural language เพื่อจัดการไฟล์:

```
Read src/main.rs
Write hello world to test.txt
```

### 5. Git Operations

```
Show git status
Commit changes with message "fix bug"
```

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl+C` | Exit |
| `Ctrl+S` | Save session |
| `Ctrl+N` | New session |
| `Ctrl+L` | List sessions |
| `Tab` | Switch panels |
| `Esc` | Cancel/Back |
| `?` | Help |

## Next Steps

- [Configuration](./configuration.md) - ตั้งค่าเพิ่มเติม
- [Usage](./usage.md) - วิธีใช้งานเชิงลึก
- [Architecture](../guides/architecture/clean-architecture.md) - เรียนรู้ architecture
