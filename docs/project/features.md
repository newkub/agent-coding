---
title: Features
description: รายการ features ทั้งหมดพร้อมคำอธิบาย
---

# Features

## Core Features

| Feature | Description | Benefit |
|---------|-------------|---------|
| **TUI Interface** | Beautiful terminal UI built with ratatui and crossterm | Modern, responsive terminal experience พร้อม keyboard shortcuts |
| **Session Persistence** | SQLite-based session storage พร้อม full CRUD operations | ไม่เสีย conversation history, switch ระหว่าง sessions ได้อย่างราบรื่น |
| **Multi-Model Support** | รองรับ OpenAI, Anthropic, Google, และ 30+ LLM providers | ใช้ AI model ที่ดีที่สุดสำหรับแต่ละ task, หลีกเลี่ยง vendor lock-in |
| **Git Integration** | Full Git operations: status, log, diff, commit, branch management | อยู่ใน terminal สำหรับทุก Git operations, ไม่ต้อง context switch |
| **File Operations** | Read, write, search, และ manage files พร้อม security controls | File manipulation โดยตรงจาก AI chat, safe directory traversal |
| **Command Execution** | Run shell commands พร้อม output capture และ error handling | Execute terminal commands ผ่าน AI interface |
| **LSP Support** | Language Server Protocol integration สำหรับ code intelligence | รับ autocomplete, diagnostics, และ hover information |
| **MCP Integration** | Model Context Protocol สำหรับ external tool integration | Extensible tool ecosystem, custom tool support |
| **Real-time Chat** | Streaming AI responses พร้อม loading indicators | Immediate feedback, conversational experience |
| **Error Handling** | Comprehensive error types พร้อม clear messages | Debugging ง่าย, user-friendly error display |

## Advanced Features

| Feature | Description | Benefit |
|---------|-------------|---------|
| **Plan Mode** | Review AI suggestions ก่อน execution | ป้องกัน unwanted changes, maintain control |
| **Skills System** | Reusable AI skills พร้อม parameter injection | Automate repetitive tasks, share prompt patterns |
| **Hooks System** | Event-driven automation บน specific triggers | Automate workflows ตาม events |
| **Memory System** | Vector-based persistent memory พร้อม semantic search | Context ข้าม sessions, intelligent retrieval |
| **Clean Architecture** | FP-style architecture พร้อม pure domain logic | Maintainable, testable, scalable codebase |
| **WASM Plugins** | Extensible plugin system ด้วย WebAssembly | Custom functionality โดยไม่ต้อง recompile |
| **Vector Memory** | Semantic search สำหรับ context retrieval | Intelligent context management ข้าม sessions |
| **Cron Scheduling** | Scheduled tasks และ automation | Automate recurring tasks |

## Integration Features

| Feature | Description | Benefit |
|---------|-------------|---------|
| **Git Integration** | Full Git operations ผ่าน git2 library | Git workflow ที่ครบถ้วนใน terminal |
| **LSP Support** | Language Server Protocol ผ่าน tower-lsp | Code intelligence สำหรับหลายภาษา |
| **MCP Integration** | Model Context Protocol สำหรับ external tools | Extensible tool ecosystem |
| **Database** | SQLite สำหรับ session persistence | Reliable data storage พร้อม backup |
| **Configuration** | TOML-based configuration พร้อม environment variables | Flexible configuration management |

## UI/UX Features

| Feature | Description | Benefit |
|---------|-------------|---------|
| **Keyboard Shortcuts** | Comprehensive keyboard navigation | Fast workflow สำหรับ power users |
| **Responsive UI** | Adaptive terminal size handling | Works บน terminal sizes ต่างๆ |
| **Theme Support** | Customizable color themes | Personalized experience |
| **Multi-panel Layout** | Split view สำหรับ different contexts | Efficient information display |
| **Toast Notifications** | Non-intrusive status updates | Clear feedback โดยไม่ interrupt workflow |

## Security Features

| Feature | Description | Benefit |
|---------|-------------|---------|
| **Path Validation** | Directory traversal protection | Secure file operations |
| **Command Validation** | Shell command safety checks | Prevent malicious command execution |
| **API Key Management** | Secure storage สำหรับ API keys | Protect sensitive credentials |
| **Session Isolation** | Separate session contexts | Privacy และ security ระหว่าง sessions |

## Performance Features

| Feature | Description | Benefit |
|---------|-------------|---------|
| **Streaming Responses** | Real-time AI response streaming | Immediate feedback |
| **Caching** | Response caching สำหรับ common queries | Reduced API calls, faster responses |
| **Async Operations** | Non-blocking I/O พร้อม tokio | Responsive UI แม้กับ heavy operations |
| **Connection Pooling** | Efficient HTTP connection management | Better resource utilization |
