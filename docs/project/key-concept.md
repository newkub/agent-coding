---
title: Key Concepts
description: Concepts สำคัญและ terminology
---

# Key Concepts

## Architecture Concepts

### Clean Architecture (FP-style)

**Mental Model**: Pure functions ใน domain layer, side effects isolated ใน adapters, dependency inversion ทั่วทั้งระบบ

- **Domain Layer** - Pure business logic, ไม่มี IO, ไม่มี state mutation, ไม่มี external dependencies
- **Application Layer** - Orchestration และ workflows, pipeline-style composition
- **Adapters Layer** - Side effects เท่านั้น: state management, IO, external services
- **Presentation Layer** - Entry points เท่านั้น: CLI, TUI initialization

### Modular Design

**Mental Model**: Vertical slice architecture พร้อม independent modules

แต่ละ module เป็น vertical slice ที่มี:
- Domain (business logic)
- Application (orchestration)
- Ports (interfaces)
- Adapters (implementations)

Modules ที่มีอยู่:
- `agent` - AI agent management
- `session` - Session persistence และ management
- `message` - Message handling แล AI responses
- `git` - Git operations
- `lsp` - Language Server Protocol
- `mcp` - Model Context Protocol

### Dependency Injection

**Mental Model**: Centralized DI container ใน presentation layer สำหรับ testability และ flexibility

- DI container ใน `presentation/di.rs`
- Pure dependency injection ด้วย composition
- Testable ด้วย mock implementations
- Flexible ด้วย runtime configuration

### Event-Driven

**Mental Model**: Domain events propagate changes ข้าม modules โดยไม่มี tight coupling

- Domain events ในแต่ละ module
- Event handlers ใน application layer
- Loose coupling ระหว่าง modules
- Async event propagation

### Provider-Agnostic

**Mental Model**: Abstract interfaces สำหรับ AI providers enable switching ระหว่าง providers

- AI provider trait ใน domain layer
- Implementations ใน adapters layer
- Runtime provider selection
- Easy provider switching

## Domain Concepts

### Session

**Definition**: Container สำหรับ conversation history และ context

Session ประกอบด้วย:
- Session ID และ name
- Creation และ modification timestamps
- Agent configuration
- Message history
- File references
- Git state (ถ้ามี)

### Agent

**Definition**: AI entity ที่ process messages และ generate responses

Agent ประกอบด้วย:
- Provider configuration (OpenAI, Anthropic, etc.)
- Model selection
- Temperature และ generation parameters
- System prompt
- Capabilities และ constraints

### Message

**Definition**: Unit ของ communication ระหว่าง user และ AI

Message ประกอบด้วย:
- Role (user, assistant, system)
- Content
- Timestamp
- Metadata (tokens, model, etc.)
- File references

### Skill

**Definition**: Reusable AI pattern พร้อม parameter injection

Skill ประกอบด้วย:
- Name และ description
- Template prompt
- Parameter definitions
- Execution logic

### Hook

**Definition**: Event-driven automation trigger

Hook ประกอบด้วย:
- Trigger event (pre-commit, post-message, etc.)
- Action definition
- Condition logic
- Execution context

## Technical Concepts

### Port & Adapter Pattern

**Definition**: Interfaces (ports) ใน domain, implementations (adapters) ใน adapters layer

- Ports กำหนด contracts
- Adapters implement ports
- Dependency inversion
- Easy testing ด้วย mocks

### Vertical Slice Architecture

**Definition**: Each feature เป็น independent slice พร้อม domain, application, และ adapters

- Feature-focused organization
- Independent deployment
- Clear boundaries
- Easy maintenance

### Functional Core, Imperative Shell

**Definition**: Pure functions ใน core, side effects ใน shell

- Core = pure, testable
- Shell = impure, IO
- Clear separation
- Predictable behavior

### Pipeline Composition

**Definition**: Compose operations ใน pipeline style

- Chain operations
- Error propagation
- Async composition
- Clear data flow

## Integration Concepts

### LSP (Language Server Protocol)

**Definition**: Protocol สำหรับ code intelligence

Features:
- Autocomplete
- Diagnostics
- Hover information
- Go to definition
- References

### MCP (Model Context Protocol)

**Definition**: Protocol สำหรับ external tool integration

Features:
- Tool discovery
- Tool execution
- Result handling
- Error management

### Git Integration

**Definition**: Git operations ผ่าน git2 library

Features:
- Status, log, diff
- Commit, branch management
- Merge, rebase
- Remote operations

## Data Concepts

### Vector Memory

**Definition**: Semantic search สำหรับ context retrieval

Features:
- Embedding generation
- Vector similarity search
- Context ranking
- Efficient retrieval

### SQLite Persistence

**Definition**: Relational database สำหรับ session storage

Features:
- ACID transactions
- Full-text search
- Backup/restore
- Migration support
