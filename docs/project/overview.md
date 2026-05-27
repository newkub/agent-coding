---
title: Project Overview
description: แนะนำโปรเจกต์ agent-tui, วัตถุประสงค์, และ use cases
---

# Project Overview

## What is agent-tui?

agent-tui เป็น Terminal User Interface (TUI) AI coding assistant ที่สร้างด้วย Clean Architecture (FP-style) สำหรับนักพัฒนาที่ต้องการใช้ AI ช่วยเขียนโค้ดโดยไม่ต้องออกจาก terminal

## Purpose

| Problem | Solution |
|---------|----------|
| TUI solutions ที่มีอยู่ขาด features, architecture ไม่ดี, หรือไม่ integrate กับ modern AI agents | agent-tui ให้ TUI ที่ feature-rich ด้วย Clean Architecture (FP-style) ที่เทียบได้กับ GUI tools เช่น Claude Code และ OpenCode |
| นักพัฒนาต้องการ AI coding assistant แบบ terminal-based โดยไม่ต้องออกจาก workflow | Integration กับ Git, LSP, MCP และ AI providers หลายตัวใน terminal interface เดียว |
| จัดการ coding sessions หลายตัวและ maintain context ข้าม restart ยาก | Session persistence ด้วย SQLite database และ context management ที่ครบถ้วน |
| จำกัด AI provider เดียวและ extensibility ต่ำ | Provider-agnostic architecture ที่รองรับ 30+ LLM providers พร้อม extensible plugin system |

## Use Cases

### Primary Use Cases

1. **AI Coding Assistant** - รับความช่วยเหลือจาก AI ในการเขียนโค้ดโดยตรงใน terminal
2. **Session Management** - จัดการ coding sessions หลายตัวด้วย persistence
3. **File Operations** - อ่าน, เขียน, และจัดการไฟล์จาก terminal
4. **Git Integration** - Git operations เต็มรูปแบบโดยไม่ต้องออกจาก terminal
5. **LSP Support** - Code intelligence ผ่าน Language Server Protocol
6. **MCP Integration** - Model Context Protocol สำหรับ external tools
7. **Terminal Interface** - Development environment แบบ terminal-based ที่ lightweight
8. **Productivity Boost** - Coding workflow ที่เร็วขึ้นด้วย AI assistance

## Target Audience

- Rust developers ที่ต้องการ AI coding assistant แบบ terminal-based
- นักพัฒนาที่ชอบใช้ terminal และต้องการเครื่องมือที่ integrate กับ workflow ปัจจุบัน
- Teams ที่ต้องการ maintainable และ testable codebase ด้วย Clean Architecture
- นักพัฒนาที่ต้องการ flexibility ในการเลือก AI providers

## Key Differentiators

- **Clean Architecture (FP-style)** - Domain logic แยกจาก side effects, ทำให้ testable และ maintainable
- **Multi-Provider Support** - รองรับ 30+ LLM providers ไม่ lock-in กับ provider เดียว
- **Extensible Plugin System** - WASM-based plugins สำหรับ custom functionality
- **Vector Memory** - Context ข้าม sessions ด้วย semantic search
- **Comprehensive Integration** - Git, LSP, MCP, และ external tools ทั้งหมดใน interface เดียว

## Project Status

**Current Phase**: Feature Development

Project has:
- ✅ Cargo.toml พร้อม workspace dependencies
- ✅ Source code structure (src/)
- ✅ Moon build system configuration
- ✅ README.md พร้อม comprehensive documentation

## Related Projects

- [wrikka/agent](../../packages/agent) - Core agent package ด้วย Clean Architecture
- [wrikka/canvas](../../packages/canvas) - Canvas integration สำหรับ visual editing
