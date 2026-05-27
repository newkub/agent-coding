---
title: FAQ
description: คำถามที่พบบ่อย
---

# FAQ

คำถามที่พบบ่อยเกี่ยวกับ agent-tui

## General

### What is agent-tui?

agent-tui เป็น Terminal User Interface (TUI) AI coding assistant ที่ใช้ Clean Architecture (FP-style) ช่วยให้คุณทำงานกับ AI ได้โดยไม่ต้องออกจาก terminal

### Why use agent-tui instead of GUI tools?

- **Stay in terminal**: ไม่ต้องสลับ context ระหว่าง editor และ browser
- **Keyboard-first**: ทำงานได้เร็วด้วย keyboard shortcuts
- **Lightweight**: ใช้ทรัพยากรน้อยกว่า GUI tools
- **Clean Architecture**: Code ที่ maintainable และ testable
- **Extensible**: Plugin system และ multi-provider support

### Is agent-tui open source?

ใช่, agent-tui เป็น open source ภายใต้ MIT License

## Installation

### How do I install agent-tui?

```bash
git clone https://github.com/your-org/rust-packages.git
cd rust-packages/apps/agent-tui
cargo build --release
```

ดู [Installation Guide](../getting-started/installation.md) สำหรับรายละเอียดเพิ่มเติม

### What are the prerequisites?

- Rust 1.70 หรือใหม่กว่า
- SQLite 3
- Git (optional, สำหรับ Git integration)

## Usage

### How do I start the TUI?

```bash
cargo run -- run
```

### How do I create a new session?

กด `Ctrl+N` ใน TUI หรือใช้ CLI:

```bash
cargo run -- create-session --name "my-session"
```

### How do I switch between sessions?

กด `Ctrl+L` ใน TUI เพื่อ list และ switch sessions

### Can I use multiple AI providers?

ใช่, agent-tui รองรับ 30+ LLM providers รวมถึง OpenAI, Anthropic, Google และอื่นๆ

## Features

### Does agent-tui support Git operations?

ใช่, รองรับ Git operations ทั้งหมด: status, log, diff, commit, branch management

### Can I execute shell commands?

ใช่, สามารถ execute shell commands ผ่าน AI interface พร้อม output capture และ error handling

### Does agent-tui support LSP?

ใช่, รองรับ Language Server Protocol สำหรับ code intelligence

### What is MCP integration?

MCP (Model Context Protocol) ช่วยให้ integrate กับ external tools และ services ได้อย่างยืดหยุ่น

## Configuration

### Where is the config file?

Config file อยู่ที่ `~/.agent-tui/config.toml`

### How do I set API keys?

สร้าง `.env` file ใน project root:

```bash
OPENAI_API_KEY=your_key
ANTHROPIC_API_KEY=your_key
```

ดู [Configuration Guide](../getting-started/configuration.md) สำหรับรายละเอียด

## Troubleshooting

### TUI rendering is broken

ตรวจสอบว่า terminal รองรับ true color:
- แนะนำ: iTerm2, Kitty, Alacritty, Windows Terminal

### Database is locked

```bash
rm ~/.agent-tui/sessions.db
cargo run -- run
```

### API key not found

ตรวจสอบ `.env` file หรือ environment variables

ดู [Troubleshooting Guide](../getting-started/troubleshooting.md) สำหรับรายละเอียดเพิ่มเติม

## Development

### How do I contribute?

ดู [Contribution Guidelines](../guides/contributing/contribution-guidelines.md)

### What is the architecture?

agent-tui ใช้ Clean Architecture (FP-style):
- **Domain**: Pure business logic
- **Application**: Orchestration
- **Adapters**: Side effects
- **Presentation**: Entry points

ดู [Architecture Guide](../guides/architecture/clean-architecture.md) สำหรับรายละเอียด

## Security

### Is my data safe?

- Session data เก็บใน SQLite database ในเครื่อง
- API keys เก็บใน environment variables หรือ config file
- File operations มี security controls และ path validation

### Can agent-tui access any file?

agent-tui มี sandbox restrictions และ path validation เพื่อป้องกัน unauthorized access

## Performance

### Is agent-tui fast?

ใช่, ออกแบบมาเพื่อ performance:
- Async/await สำหรับ I/O operations
- Database query optimization
- Streaming AI responses
- Cache สำหรับ offline mode

### How much memory does it use?

ขึ้นอยู่กับจำนวน sessions และ messages แต่โดยทั่วไปใช้ ~50-100MB

## Still have questions?

- [GitHub Issues](https://github.com/your-org/rust-packages/issues)
- [Discord Community](https://discord.gg/your-server)
- [Documentation](../)
