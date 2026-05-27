---
title: CLI Commands Reference
description: CLI commands และ options ทั้งหมด
---

# CLI Commands Reference

คำสั่ง CLI ทั้งหมดสำหรับ agent-tui

## Global Options

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--config` | `-c` | Path to config file | `~/.agent-tui/config.toml` |
| `--verbose` | `-v` | Enable verbose logging | `false` |
| `--quiet` | `-q` | Suppress output | `false` |
| `--help` | `-h` | Show help | - |
| `--version` | `-V` | Show version | - |

## Commands

### run

Run TUI interface

```bash
agent-tui run [OPTIONS]
```

**Options**:
- `--session <ID>` - Load specific session
- `--agent <ID>` - Use specific agent
- `--theme <NAME>` - Set theme (default, dark, light)

**Example**:
```bash
agent-tui run --session abc123 --agent gpt4
```

### create-session

Create new session

```bash
agent-tui create-session [OPTIONS] --name <NAME>
```

**Options**:
- `--name <NAME>` - Session name (required)
- `--agent <ID>` - Agent ID (default: from config)
- `--description <TEXT>` - Session description

**Example**:
```bash
agent-tui create-session --name "project-review" --agent gpt4
```

### list-sessions

List all sessions

```bash
agent-tui list-sessions [OPTIONS]
```

**Options**:
- `--format <FORMAT>` - Output format (table, json) - default: table
- `--limit <N>` - Limit results - default: 20

**Example**:
```bash
agent-tui list-sessions --format json
```

### delete-session

Delete session

```bash
agent-tui delete-session <ID>
```

**Example**:
```bash
agent-tui delete-session abc123
```

### list-agents

List available agents

```bash
agent-tui list-agents [OPTIONS]
```

**Options**:
- `--format <FORMAT>` - Output format (table, json) - default: table

**Example**:
```bash
agent-tui list-agents
```

### export-session

Export session to file

```bash
agent-tui export-session <ID> --output <PATH>
```

**Options**:
- `--output <PATH>` - Output file path (required)
- `--format <FORMAT>` - Export format (json, markdown) - default: json

**Example**:
```bash
agent-tui export-session abc123 --output session.json
```

### import-session

Import session from file

```bash
agent-tui import-session --input <PATH>
```

**Options**:
- `--input <PATH>` - Input file path (required)

**Example**:
```bash
agent-tui import-session --input session.json
```

### config

Manage configuration

```bash
agent-tui config <SUBCOMMAND>
```

**Subcommands**:
- `get <KEY>` - Get config value
- `set <KEY> <VALUE>` - Set config value
- `list` - List all config values
- `reset` - Reset to defaults

**Example**:
```bash
agent-tui config get ai.model
agent-tui config set ai.model gpt-4
agent-tui config list
```

### cleanup

Cleanup old data

```bash
agent-tui cleanup [OPTIONS]
```

**Options**:
- `--sessions <DAYS>` - Delete sessions older than N days
- `--cache` - Clear cache
- `--logs` - Clear logs

**Example**:
```bash
agent-tui cleanup --sessions 30 --cache
```

### doctor

Check system health

```bash
agent-tui doctor
```

**Checks**:
- Rust version
- Database connectivity
- API key configuration
- Git installation
- LSP server availability

**Example**:
```bash
agent-tui doctor
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | General error |
| 2 | Invalid arguments |
| 3 | Configuration error |
| 4 | Database error |
| 5 | Network error |
| 6 | Authentication error |

## Environment Variables

| Variable | Description |
|----------|-------------|
| `OPENAI_API_KEY` | OpenAI API key |
| `ANTHROPIC_API_KEY` | Anthropic API key |
| `GOOGLE_API_KEY` | Google API key |
| `DATABASE_URL` | SQLite database path |
| `AGENT_TUI_CONFIG` | Config file path |
| `AGENT_TUI_LOG_LEVEL` | Log level (debug, info, warn, error) |
