---
title: Troubleshooting
description: แก้ปัญหาที่พบบ่อย
---

# Troubleshooting

ปัญหาที่พบบ่อยและวิธีแก้ไข

## Installation Issues

### Rust version too old

**Problem**: `error: package agent-tui requires rust version 1.70 or later`

**Solution**:
```bash
rustup update stable
rustup default stable
```

### Build fails with dependencies error

**Problem**: `error: failed to compile agent-tui`

**Solution**:
```bash
cargo clean
cargo update
cargo build
```

## Runtime Issues

### Database locked

**Problem**: `database is locked` error

**Solution**:
```bash
# ลบ database และสร้างใหม่
rm ~/.agent-tui/sessions.db
cargo run -- run
```

### API key not found

**Problem**: `OPENAI_API_KEY not found` error

**Solution**:
```bash
# สร้าง .env file
echo "OPENAI_API_KEY=your_key" > .env

# หรือ set environment variable
export OPENAI_API_KEY=your_key
```

### TUI rendering issues

**Problem**: Terminal rendering broken or garbled

**Solution**:
```bash
# ตรวจสอบ terminal size
echo $COLUMNS $LINES

# ใช้ terminal ที่รองรับ true color
# แนะนำ: iTerm2, Kitty, Alacritty, Windows Terminal
```

## Performance Issues

### Slow AI responses

**Problem**: AI responses take too long

**Solution**:
1. เปลี่ยน model ที่เร็วกว่าใน config:
```toml
[ai]
model = "gpt-3.5-turbo"  # แทน gpt-4
```

2. เปิด cache:
```toml
[database]
auto_backup = false
```

### High memory usage

**Problem**: Application uses too much memory

**Solution**:
```bash
# ลบ old sessions
cargo run -- cleanup-sessions --older-than 30d
```

## Git Integration Issues

### Git not found

**Problem**: `git command not found`

**Solution**:
```bash
# Install Git
# Ubuntu/Debian: sudo apt install git
# macOS: brew install git
# Windows: https://git-scm.com/download/win
```

### Git authentication failed

**Problem**: Git operations fail with auth error

**Solution**:
```bash
# Setup Git credentials
git config --global user.name "Your Name"
git config --global user.email "your@email.com"

# หรือใช้ SSH key
ssh-keygen -t ed25519
```

## LSP Integration Issues

### LSP server not starting

**Problem**: LSP features not working

**Solution**:
```toml
[lsp]
enabled = false  # ปิด LSP ชั่วคราว
```

### LSP server path incorrect

**Problem**: `LSP server not found at path`

**Solution**:
```toml
[lsp]
server_path = "/usr/local/bin/rust-analyzer"
```

## Network Issues

### Connection timeout

**Problem**: `connection timeout` when calling AI API

**Solution**:
```bash
# ตรวจสอบ internet connection
ping api.openai.com

# หรือใช้ proxy
export HTTP_PROXY=http://proxy:port
export HTTPS_PROXY=http://proxy:port
```

### SSL certificate error

**Problem**: `SSL certificate verify failed`

**Solution**:
```bash
# Update CA certificates
# Ubuntu/Debian: sudo apt install ca-certificates
# macOS: brew install ca-certificates
```

## Getting Help

ถ้าไม่พบวิธีแก้ไขที่นี่:

1. Check [GitHub Issues](https://github.com/your-org/rust-packages/issues)
2. Read [Architecture Guide](../guides/architecture/clean-architecture.md)
3. Join [Discord Community](https://discord.gg/your-server)
