---
title: Environment Variables Reference
description: Environment variables ทั้งหมด
---

# Environment Variables Reference

Environment variables สำหรับ configuration agent-tui

## AI Provider Configuration

### OpenAI

| Variable | Description | Required | Default |
|----------|-------------|----------|---------|
| `OPENAI_API_KEY` | OpenAI API key | Yes | - |
| `OPENAI_ORGANIZATION` | OpenAI organization ID | No | - |
| `OPENAI_BASE_URL` | Custom OpenAI base URL | No | https://api.openai.com/v1 |

**Example**:
```bash
export OPENAI_API_KEY=sk-...
export OPENAI_ORGANIZATION=org-...
```

### Anthropic

| Variable | Description | Required | Default |
|----------|-------------|----------|---------|
| `ANTHROPIC_API_KEY` | Anthropic API key | Yes | - |
| `ANTHROPIC_BASE_URL` | Custom Anthropic base URL | No | https://api.anthropic.com |

**Example**:
```bash
export ANTHROPIC_API_KEY=sk-ant-...
```

### Google

| Variable | Description | Required | Default |
|----------|-------------|----------|---------|
| `GOOGLE_API_KEY` | Google API key | Yes | - |
| `GOOGLE_PROJECT_ID` | Google Cloud project ID | No | - |

**Example**:
```bash
export GOOGLE_API_KEY=...
export GOOGLE_PROJECT_ID=my-project
```

## Database Configuration

| Variable | Description | Required | Default |
|----------|-------------|----------|---------|
| `DATABASE_URL` | SQLite database path | No | ~/.agent-tui/sessions.db |
| `DATABASE_POOL_SIZE` | Connection pool size | No | 5 |
| `DATABASE_TIMEOUT` | Query timeout (seconds) | No | 30 |

**Example**:
```bash
export DATABASE_URL=sqlite:///path/to/sessions.db
export DATABASE_POOL_SIZE=10
```

## Git Configuration

| Variable | Description | Required | Default |
|----------|-------------|----------|---------|
| `GIT_ENABLED` | Enable Git integration | No | true |
| `GIT_AUTO_COMMIT` | Auto-commit changes | No | false |
| `GIT_COMMIT_TEMPLATE` | Commit message template | No | "feat: %s" |
| `GIT_SSH_KEY_PATH` | Path to SSH private key | No | ~/.ssh/id_rsa |

**Example**:
```bash
export GIT_ENABLED=true
export GIT_AUTO_COMMIT=false
export GIT_COMMIT_TEMPLATE="chore: %s"
```

## LSP Configuration

| Variable | Description | Required | Default |
|----------|-------------|----------|---------|
| `LSP_ENABLED` | Enable LSP support | No | true |
| `LSP_SERVER_PATH` | Path to LSP server | No | - |
| `LSP_HOVER_DELAY_MS` | Hover delay in milliseconds | No | 300 |
| `LSP_COMPLETION_TRIGGER_CHARS` | Completion trigger characters | No | ".:(" |

**Example**:
```bash
export LSP_ENABLED=true
export LSP_SERVER_PATH=/usr/local/bin/rust-analyzer
export LSP_HOVER_DELAY_MS=300
```

## MCP Configuration

| Variable | Description | Required | Default |
|----------|-------------|----------|---------|
| `MCP_ENABLED` | Enable MCP integration | No | true |
| `MCP_SERVER_URL` | MCP server URL | No | http://localhost:3000 |
| `MCP_TIMEOUT_SECONDS` | Request timeout (seconds) | No | 30 |
| `MCP_API_KEY` | MCP server API key | No | - |

**Example**:
```bash
export MCP_ENABLED=true
export MCP_SERVER_URL=http://localhost:3000
export MCP_TIMEOUT_SECONDS=30
```

## Application Configuration

| Variable | Description | Required | Default |
|----------|-------------|----------|---------|
| `AGENT_TUI_CONFIG` | Path to config file | No | ~/.agent-tui/config.toml |
| `AGENT_TUI_LOG_LEVEL` | Log level | No | info |
| `AGENT_TUI_LOG_FILE` | Path to log file | No | ~/.agent-tui/agent-tui.log |
| `AGENT_TUI_THEME` | UI theme | No | default |
| `AGENT_TUI_DATA_DIR` | Data directory | No | ~/.agent-tui |

**Example**:
```bash
export AGENT_TUI_CONFIG=/path/to/config.toml
export AGENT_TUI_LOG_LEVEL=debug
export AGENT_TUI_THEME=dark
```

## Network Configuration

| Variable | Description | Required | Default |
|----------|-------------|----------|---------|
| `HTTP_PROXY` | HTTP proxy URL | No | - |
| `HTTPS_PROXY` | HTTPS proxy URL | No | - |
| `NO_PROXY` | No proxy hosts | No | - |
| `REQUEST_TIMEOUT_SECONDS` | Request timeout (seconds) | No | 60 |

**Example**:
```bash
export HTTP_PROXY=http://proxy:8080
export HTTPS_PROXY=http://proxy:8080
export NO_PROXY=localhost,127.0.0.1
```

## Security Configuration

| Variable | Description | Required | Default |
|----------|-------------|----------|---------|
| `ALLOWED_DIRS` | Allowed directories (comma-separated) | No | - |
| `BLOCKED_DIRS` | Blocked directories (comma-separated) | No | - |
| `MAX_FILE_SIZE_MB` | Maximum file size (MB) | No | 10 |
| `ENABLE_COMMAND_EXECUTION` | Enable command execution | No | true |

**Example**:
```bash
export ALLOWED_DIRS=/home/user/projects,/tmp
export BLOCKED_DIRS=/etc,/usr
export MAX_FILE_SIZE_MB=50
export ENABLE_COMMAND_EXECUTION=true
```

## Cache Configuration

| Variable | Description | Required | Default |
|----------|-------------|----------|---------|
| `CACHE_ENABLED` | Enable cache | No | true |
| `CACHE_DIR` | Cache directory | No | ~/.agent-tui/cache |
| `CACHE_TTL_SECONDS` | Cache TTL (seconds) | No | 3600 |
| `CACHE_MAX_SIZE_MB` | Maximum cache size (MB) | No | 1000 |

**Example**:
```bash
export CACHE_ENABLED=true
export CACHE_DIR=/tmp/agent-tui-cache
export CACHE_TTL_SECONDS=7200
```

## Memory Configuration

| Variable | Description | Required | Default |
|----------|-------------|----------|---------|
| `MEMORY_ENABLED` | Enable memory system | No | true |
| `MEMORY_DB_PATH` | Memory database path | No | ~/.agent-tui/memory.db |
| `MEMORY_VECTOR_SIZE` | Vector dimension | No | 1536 |
| `MEMORY_MAX_RESULTS` | Maximum search results | No | 10 |

**Example**:
```bash
export MEMORY_ENABLED=true
export MEMORY_DB_PATH=/path/to/memory.db
export MEMORY_MAX_RESULTS=20
```

## Example .env File

```bash
# AI Providers
OPENAI_API_KEY=sk-...
ANTHROPIC_API_KEY=sk-ant-...
GOOGLE_API_KEY=...

# Database
DATABASE_URL=sqlite:///path/to/sessions.db

# Git
GIT_ENABLED=true
GIT_AUTO_COMMIT=false

# LSP
LSP_ENABLED=true
LSP_SERVER_PATH=/usr/local/bin/rust-analyzer

# MCP
MCP_ENABLED=true
MCP_SERVER_URL=http://localhost:3000

# Application
AGENT_TUI_LOG_LEVEL=info
AGENT_TUI_THEME=default

# Network
HTTP_PROXY=http://proxy:8080
HTTPS_PROXY=http://proxy:8080

# Security
ALLOWED_DIRS=/home/user/projects
MAX_FILE_SIZE_MB=50

# Cache
CACHE_ENABLED=true
CACHE_TTL_SECONDS=7200

# Memory
MEMORY_ENABLED=true
MEMORY_MAX_RESULTS=20
```

## Priority

Configuration priority (high to low):
1. Environment variables
2. Config file (`~/.agent-tui/config.toml`)
3. Default values

## Security Notes

- ไม่ควร commit `.env` file ไปยัง version control
- ใช้ `.env.example` เป็น template
- Set proper file permissions: `chmod 600 .env`
- ใช้ secret management tools สำหรับ production
