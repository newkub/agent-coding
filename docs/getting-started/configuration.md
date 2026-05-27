---
title: Configuration
description: การตั้งค่า environment และ config
---

# Configuration

## Environment Configuration

สร้าง `.env` file ใน project root หรือ set environment variables:

### AI Provider Configuration

```bash
# OpenAI
OPENAI_API_KEY=your_openai_api_key

# Anthropic
ANTHROPIC_API_KEY=your_anthropic_api_key

# Google
GOOGLE_API_KEY=your_google_api_key

# Default provider
AI_DEFAULT_PROVIDER=openai
```

### Database Configuration

```bash
# Database path
DATABASE_URL=sqlite:///path/to/sessions.db

# Auto backup
DATABASE_AUTO_BACKUP=true
DATABASE_BACKUP_INTERVAL=24h
```

### Git Configuration

```bash
# Enable Git integration
GIT_ENABLED=true

# Auto commit
GIT_AUTO_COMMIT=false

# Commit template
GIT_COMMIT_TEMPLATE="feat: %s"
```

### LSP Configuration

```bash
# Enable LSP
LSP_ENABLED=true

# LSP server path
LSP_SERVER_PATH=/path/to/lsp-server

# Hover delay
LSP_HOVER_DELAY_MS=300

# Completion trigger chars
LSP_COMPLETION_TRIGGER_CHARS=".:("
```

### MCP Configuration

```bash
# Enable MCP
MCP_ENABLED=true

# MCP server URL
MCP_SERVER_URL=http://localhost:3000

# Timeout
MCP_TIMEOUT_SECONDS=30
```

### General Configuration

```bash
# Log level
RUST_LOG=info

# Backtrace
RUST_BACKTRACE=1

# Theme
THEME=default
```

## File Configuration

สร้าง `agent-tui.toml` ใน directory เดียวกับ binary:

### General Settings

```toml
[general]
log_level = "info"
theme = "default"
```

### AI Settings

```toml
[ai]
default_provider = "openai"
model = "gpt-4"
temperature = 0.7
max_tokens = 4096
```

### Database Settings

```toml
[database]
path = "~/.agent-tui/sessions.db"
auto_backup = true
backup_interval = "24h"
```

### Git Settings

```toml
[git]
enabled = true
auto_stage = false
commit_template = "feat: %s"
```

### LSP Settings

```toml
[lsp]
enabled = true
hover_delay_ms = 300
completion_trigger_chars = [".", ":", "("]
```

### MCP Settings

```toml
[mcp]
enabled = true
server_url = "http://localhost:3000"
timeout_seconds = 30
```

## Configuration Priority

Configuration จะถูกโหลดตามลำดับนี้ (priority สูงสุดก่อน):

1. Environment variables
2. `agent-tui.toml` file
3. Default values

## Example Configuration

### Minimal Configuration

```bash
# .env
OPENAI_API_KEY=sk-...
DATABASE_URL=sqlite:///sessions.db
```

### Full Configuration

```bash
# .env
OPENAI_API_KEY=sk-...
ANTHROPIC_API_KEY=sk-...
DATABASE_URL=sqlite:///sessions.db
GIT_ENABLED=true
LSP_ENABLED=true
MCP_ENABLED=true
RUST_LOG=debug
```

```toml
# agent-tui.toml
[general]
log_level = "debug"
theme = "dark"

[ai]
default_provider = "openai"
model = "gpt-4"
temperature = 0.7
max_tokens = 4096

[database]
path = "~/.agent-tui/sessions.db"
auto_backup = true
backup_interval = "24h"

[git]
enabled = true
auto_stage = false
commit_template = "feat: %s"

[lsp]
enabled = true
hover_delay_ms = 300
completion_trigger_chars = [".", ":", "("]

[mcp]
enabled = true
server_url = "http://localhost:3000"
timeout_seconds = 30
```

## Configuration Validation

agent-tui จะ validate configuration เมื่อ start:

- Check required environment variables
- Validate configuration file syntax
- Verify database path accessibility
- Check API key format

ถ้า configuration ไม่ valid, agent-tui จะแสดง error message และ exit

## Reloading Configuration

Configuration จะถูก reload เมื่อ:

- Restart application
- หรือใช้ command ใน TUI: `:reload-config`

## Secure Configuration

### API Keys

- ไม่เคย commit API keys ไปยัง git
- ใช้ environment variables สำหรับ secrets
- ใช้ `.env` file และ add ไปยัง `.gitignore`
- Rotate keys regularly

### Database Path

- ใช้ absolute paths หรือ expand `~`
- Ensure directory มี write permissions
- Backup database regularly

### Git Configuration

- ตรวจสอบ git remote URLs
- ใช้ SSH keys สำหรับ private repos
- Configure git user info
