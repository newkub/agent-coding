---
title: Configuration Options
description: Configuration options แล settings
---

# Configuration Options

## Environment Variables

### AI Provider Configuration

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `OPENAI_API_KEY` | String | - | OpenAI API key |
| `ANTHROPIC_API_KEY` | String | - | Anthropic API key |
| `GOOGLE_API_KEY` | String | - | Google API key |
| `AI_DEFAULT_PROVIDER` | String | `openai` | Default AI provider |

### Database Configuration

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `DATABASE_URL` | String | `sqlite:///sessions.db` | Database connection string |
| `DATABASE_AUTO_BACKUP` | Boolean | `true` | Enable automatic backups |
| `DATABASE_BACKUP_INTERVAL` | String | `24h` | Backup interval |

### Git Configuration

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `GIT_ENABLED` | Boolean | `true` | Enable Git integration |
| `GIT_AUTO_COMMIT` | Boolean | `false` | Auto-commit changes |
| `GIT_COMMIT_TEMPLATE` | String | `feat: %s` | Commit message template |

### LSP Configuration

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `LSP_ENABLED` | Boolean | `true` | Enable LSP integration |
| `LSP_SERVER_PATH` | String | - | LSP server path |
| `LSP_HOVER_DELAY_MS` | Integer | `300` | Hover delay in milliseconds |
| `LSP_COMPLETION_TRIGGER_CHARS` | String | `.:(` | Completion trigger characters |

### MCP Configuration

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `MCP_ENABLED` | Boolean | `true` | Enable MCP integration |
| `MCP_SERVER_URL` | String | `http://localhost:3000` | MCP server URL |
| `MCP_TIMEOUT_SECONDS` | Integer | `30` | Request timeout |

### General Configuration

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `RUST_LOG` | String | `info` | Log level |
| `RUST_BACKTRACE` | Integer | `1` | Backtrace level |
| `THEME` | String | `default` | UI theme |

## File Configuration

### agent-tui.toml

```toml
[general]
log_level = "info"           # Log level: trace, debug, info, warn, error
theme = "default"            # UI theme: default, dark, light

[ai]
default_provider = "openai"  # Default AI provider
model = "gpt-4"              # Default model
temperature = 0.7            # Temperature (0.0 - 2.0)
max_tokens = 4096            # Max tokens (1 - 128000)

[database]
path = "~/.agent-tui/sessions.db"  # Database path
auto_backup = true                 # Enable auto backup
backup_interval = "24h"            # Backup interval

[git]
enabled = true               # Enable Git integration
auto_stage = false           # Auto-stage changes
commit_template = "feat: %s" # Commit template

[lsp]
enabled = true               # Enable LSP integration
hover_delay_ms = 300        # Hover delay
completion_trigger_chars = [".", "(", ":"]  # Trigger chars

[mcp]
enabled = true               # Enable MCP integration
server_url = "http://localhost:3000"  # Server URL
timeout_seconds = 30         # Request timeout
```

## Configuration Priority

1. Environment variables (highest priority)
2. `agent-tui.toml` file
3. Default values (lowest priority)

## Supported AI Providers

| Provider | Models | Notes |
|----------|--------|-------|
| `openai` | gpt-4, gpt-3.5-turbo | Requires API key |
| `anthropic` | claude-3-opus, claude-3-sonnet | Requires API key |
| `google` | gemini-pro, gemini-ultra | Requires API key |
| `cohere` | command, command-light | Requires API key |
| `huggingface` | Various | Requires API key |

## Supported Themes

| Theme | Description |
|-------|-------------|
| `default` | Default theme |
| `dark` | Dark theme |
| `light` | Light theme |
| `solarized` | Solarized theme |
| `nord` | Nord theme |

## Log Levels

| Level | Description |
|-------|-------------|
| `trace` | Most verbose |
| `debug` | Debug information |
| `info` | General information |
| `warn` | Warnings |
| `error` | Errors only |
