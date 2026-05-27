<div align="center">

# agent-tui

**Terminal User Interface (TUI) AI coding assistant with Clean Architecture (FP-style)**

A modern, feature-rich terminal-based AI assistant that integrates with 30+ AI providers, Git, LSP, and MCP for seamless development workflows.

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://img.shields.io/badge/Build-Passing-green.svg)](https://github.com/your-org/rust-packages)

[Features](#features) • [Quick Start](#installation) • [Usage](#usage) • [Architecture](#architecture) • [Reference](#reference)

</div>

<!-- Banner Image 16:9 -->
<div align="center">
  <img src="https://placehold.co/1280x720/60a5fa/ffffff?text=agent-tui+Terminal+AI+Assistant" alt="agent-tui Banner" width="1280" height="720">
</div>

---

> [!NOTE]
> <details>
> <summary><b>Key Concepts</b></summary>
>
> Core concepts that help you understand how the application works and what benefits it provides.
>
> | Icon | Concept | Benefit |
> |-------|---------|---------|
> | <center><img src="https://api.iconify.design/mdi:layers.svg?color=%2360a5fa" width="24" height="24"></center> | Clean Architecture (FP-style) | Reliable and maintainable code that ensures consistent behavior across all features |
> | <center><img src="https://api.iconify.design/mdi:view-module.svg?color=%2334d399" width="24" height="24"></center> | Modular Design | Independent features that work together seamlessly without conflicts |
> | <center><img src="https://api.iconify.design/mdi:dependency.svg?color=%23f59e0b" width="24" height="24"></center> | Dependency Injection | Easy to test and extend with new capabilities without breaking existing functionality |
> | <center><img src="https://api.iconify.design/mdi:flash.svg?color=%23ef4444" width="24" height="24"></center> | Event-Driven | Real-time updates and smooth interactions between different parts of the application |
> | <center><img src="https://api.iconify.design/mdi:cloud-outline.svg?color=%238b5cf6" width="24" height="24"></center> | Provider-Agnostic | Switch between AI providers (OpenAI, Anthropic, Google, etc.) without changing your workflow |
> </details>
>
> <details>
> <summary><b>Principles</b></summary>
>
> Design principles that ensure the application works reliably and efficiently for you.
>
> | Icon | Principle | User Impact |
> |-------|-----------|-------------|
> | <center><img src="https://api.iconify.design/mdi:water.svg?color=%230ea5e9" width="24" height="24"></center> | Domain Purity | Predictable behavior with consistent results every time you use the application |
> | <center><img src="https://api.iconify.design/mdi:call-split.svg?color=%23e11d48" width="24" height="24"></center> | Separation of Concerns | Clear and focused features that are easy to understand and use |
> | <center><img src="https://api.iconify.design/mdi:arrow-u-down-left.svg?color=%2385430d" width="24" height="24"></center> | Dependency Inversion | Flexible integration with new tools and services without breaking existing workflows |
> | <center><img src="https://api.iconify.design/mdi:lock.svg?color=%230f172a" width="24" height="24"></center> | Immutability | Safe operations that prevent accidental data loss or corruption |
> | <center><img src="https://api.iconify.design/mdi:merge.svg?color=%23663399" width="24" height="24"></center> | Composition over Inheritance | Easy to customize and extend with new features without complex configurations |
> </details>
>
> <details>
> <summary><b>FAQs</b></summary>
>
> Frequently asked questions and answers about using the application.
>
> | Question | Answer |
> |----------|--------|
> | How do I switch between AI providers? | Set `default_provider` in `agent-tui.toml` or use environment variables |
> | Can I use agent-tui without Git? | Yes, Git integration is optional and can be disabled |
> | Is my session data encrypted? | Session data is stored in SQLite without encryption by default |
> | How do I customize keyboard shortcuts? | Edit the `[keyboard]` section in `agent-tui.toml` |
> | Can I run agent-tui in headless mode? | Yes, use the CLI commands for programmatic access |
> </details>
>
> <details>
> <summary><b>Release</b></summary>
>
> Version history and release notes showing the evolution of the project.
>
> ### v0.5.x - May 2026
>
> | ID | Version | Title | Description | Type |
> |----|---------|-------|-------------|------|
> | 1 | [v0.5.0](https://github.com/your-org/rust-packages/releases/tag/v0.5.0) | WASM Plugin System | Add WASM plugin support with wasmtime for extensible tool ecosystem | Major |
>
> ### v0.4.x - April 2026
>
> | ID | Version | Title | Description | Type |
> |----|---------|-------|-------------|------|
> | 2 | [v0.4.0](https://github.com/your-org/rust-packages/releases/tag/v0.4.0) | Vector Memory | Implement vector-based persistent memory with semantic search | Major |
>
> ### v0.3.x - March 2026
>
> | ID | Version | Title | Description | Type |
> |----|---------|-------|-------------|------|
> | 3 | [v0.3.2](https://github.com/your-org/rust-packages/releases/tag/v0.3.2) | Bug Fixes | Fix session persistence issues and improve error handling | Patch |
> | 4 | [v0.3.1](https://github.com/your-org/rust-packages/releases/tag/v0.3.1) | Performance | Optimize TUI rendering and reduce memory usage | Patch |
> | 5 | **[v0.3.0](https://github.com/your-org/rust-packages/releases/tag/v0.3.0)** | Skills System | Add reusable AI skills with parameter injection | Minor |
>
> ### v0.2.x - February 2026
>
> | ID | Version | Title | Description | Type |
> |----|---------|-------|-------------|------|
> | 6 | [v0.2.5](https://github.com/your-org/rust-packages/releases/tag/v0.2.5) | LSP Integration | Improve LSP hover and completion support | Patch |
> | 7 | [v0.2.4](https://github.com/your-org/rust-packages/releases/tag/v0.2.4) | Git Operations | Fix git diff display and branch management | Patch |
> | 8 | **[v0.2.0](https://github.com/your-org/rust-packages/releases/tag/v0.2.0)** | MCP Integration | Add Model Context Protocol for external tools | Minor |
>
> ### v0.1.x - January 2026
>
> | ID | Version | Title | Description | Type |
> |----|---------|-------|-------------|------|
> | 9 | [v0.1.3](https://github.com/your-org/rust-packages/releases/tag/v0.1.3) | File Operations | Improve file search and directory traversal | Patch |
> | 10 | [v0.1.2](https://github.com/your-org/rust-packages/releases/tag/v0.1.2) | CLI Commands | Add session list and agent list commands | Patch |
> </details>
>
> <details>
> <summary><b>Best Practices</b></summary>
>
> Recommended practices for optimal usage.
>
> - Start with code onboarding to understand project structure before making changes
> - Use subagents for specialized tasks (code review, bug hunting, refactoring)
> - Enable guardrails for security, quality, and performance validation
> - Monitor performance metrics regularly to identify bottlenecks
> </details>

## Features

Complete list of features and capabilities provided by the application.

| Icon | Feature | Description | Benefit | Usage |
|-------|---------|-------------|---------|-------|
| <center><img src="https://api.iconify.design/mdi:monitor-dashboard.svg?color=%234285f4" width="18" height="18"></center> | TUI Interface | Beautiful terminal UI built with ratatui and crossterm | Intuitive interface for efficient terminal-based workflows | Daily coding sessions |
| <center><img src="https://api.iconify.design/mdi:database.svg?color=%23795548" width="18" height="18"></center> | Session Persistence | SQLite-based session storage with full CRUD operations | Never lose your conversation history | Long-term projects |
| <center><img src="https://api.iconify.design/mdi:git.svg?color=%23f44336" width="18" height="18"></center> | Git Integration | Full Git operations: status, log, diff, commit, branch management | Seamless Git workflow within the terminal | Version control tasks |
| <center><img src="https://api.iconify.design/mdi:rocket-launch.svg?color=%2310b981" width="18" height="18"></center> | Code Onboarding | Agentic codebase analysis with structure detection and tech stack inference | Quick understanding of new codebases | Starting new projects |
| <center><img src="https://api.iconify.design/mdi:auto-fix.svg?color=%23f59e0b" width="18" height="18"></center> | Automation | Automated GitHub issue to pull request workflow with smart branch naming | Streamlined issue-to-PR workflow | GitHub management |
| <center><img src="https://api.iconify.design/mdi:console.svg?color=%236366f1" width="18" height="18"></center> | Headless Mode | CLI mode for automation and scripting with multiple output formats | Programmatic access for CI/CD pipelines | Automation scripts |
| <center><img src="https://api.iconify.design/mdi:robot.svg?color=%238b5cf6" width="18" height="18"></center> | Subagents System | Specialized AI agents for code review, bug hunting, refactoring, testing, security | Specialized expertise for specific tasks | Code quality assurance |
| <center><img src="https://api.iconify.design/mdi:shield-check.svg?color=%23ef4444" width="18" height="18"></center> | Guardrails System | Security, quality, and performance validation with configurable rules | Prevent security issues and performance problems | Production deployments |
| <center><img src="https://api.iconify.design/mdi:speedometer.svg?color=%2310b981" width="18" height="18"></center> | Performance Monitoring | Real-time metrics, response time tracking, and performance analysis | Identify bottlenecks and optimize performance | Performance tuning |
| <center><img src="https://api.iconify.design/mdi:clipboard-text.svg?color=%2360a5fa" width="18" height="18"></center> | Audit System | Comprehensive audit logging for all operations | Track and review all system activities | Security compliance |
| <center><img src="https://api.iconify.design/mdi:account-group.svg?color=%239c27b0" width="18" height="18"></center> | Collaboration | Multi-user session sharing and real-time collaboration | Work together with team members on the same session | Team projects |
| <center><img src="https://api.iconify.design/mdi:compare.svg?color=%23ff9800" width="18" height="18"></center> | Diff System | Advanced diff viewing and comparison capabilities | Visualize changes between code versions | Code review |
| <center><img src="https://api.iconify.design/mdi:code-braces.svg?color=%234caf50" width="18" height="18"></center> | Macros System | Reusable code macros and templates | Automate repetitive code patterns | Code generation |
| <center><img src="https://api.iconify.design/mdi:shield.svg?color=%23e91e63" width="18" height="18"></center> | Sandbox | Isolated execution environment for safe command running | Run untrusted code safely | Testing and debugging |
| <center><img src="https://api.iconify.design/mdi:share-variant.svg?color=%23795548" width="18" height="18"></center> | Share System | Share sessions and snippets with others | Distribute knowledge and solutions | Knowledge sharing |
| <center><img src="https://api.iconify.design/mdi:file-code.svg?color=%232196f3" width="18" height="18"></center> | Snippet System | Code snippet management and reuse | Save and reuse code snippets | Code productivity |

## Quick Start

Get started with agent-tui in a few simple steps.

1. **Clone the repository**
   ```bash
   git clone https://github.com/your-org/rust-packages.git
   cd rust-packages/apps/agent-tui
   ```

2. **Build the project**
   ```bash
   cargo build --release
   ```

3. **Run the application**
   ```bash
   cargo run -- run
   ```

4. **Configure AI providers** (optional)
   ```bash
   # Set environment variables
   export OPENAI_API_KEY=your_key
   ```

5. **Start coding** with AI assistance in your terminal

## Usage

Detailed usage examples and instructions for using the application.

### Method 1: Usage via CLI

#### Basic Commands

```bash
# Run TUI interface (default)
cargo run

# Run TUI explicitly
cargo run tui

# Show version
cargo run -- version
```

#### Code Onboarding

```bash
# Analyze codebase structure and dependencies
cargo run onboarding /path/to/project

# Output includes:
# - Project type detection
# - File structure analysis
# - Tech stack inference
# - Entry point identification
# - Dependency analysis
```

#### Issue-to-PR Automation

```bash
# Automate GitHub issue to PR workflow
cargo run automate owner/repo 123

# Requires GITHUB_TOKEN environment variable
export GITHUB_TOKEN=your_github_token

# Workflow includes:
# - Issue analysis
# - Branch creation
# - Commit generation
# - PR creation with auto-generated title and body
```

#### Headless Mode

```bash
# Run in headless mode for automation
cargo run headless "/chat explain this code" --directory /path/to/project --format text

# Output formats: text, json, markdown
cargo run headless "/read file.txt" --format json
cargo run headless "/write new.rs" --format markdown

# Commands available:
# - /chat: Chat with AI
# - /read: Read file
# - /write: Write file
# - /search: Search codebase
# - /execute: Run command
```

#### Subagents System

```bash
# List available subagents
cargo run subagent list

# Execute subagent task
cargo run subagent execute code-reviewer "Review this code"

# Available subagents:
# - code-reviewer: Code quality review
# - bug-hunter: Bug detection
# - refactorer: Refactoring suggestions
# - documenter: Documentation generation
# - tester: Test case generation
# - security-auditor: Security analysis
# - performance-optimizer: Performance optimization
```

#### Guardrails System

```bash
# Run guardrails check
cargo run guardrail "input to check" --guardrail-type security

# Guardrail types: security, quality, performance
cargo run guardrail "SELECT * FROM users" --guardrail-type security
cargo run guardrail "TODO: fix this" --guardrail-type quality
```

#### Performance Monitoring

```bash
# Analyze performance metrics
cargo run performance analyze

# Create performance snapshot
cargo run performance snapshot

# Generate performance report
cargo run performance report
```

#### TUI Interface Controls

Once in TUI mode:

- **Type**: Enter message in input field
- **Enter**: Send message to AI
- **Ctrl+C**: Exit application
- **Ctrl+S**: Save current session
- **Ctrl+N**: Create new session
- **Ctrl+L**: List sessions
- **Tab**: Switch between panels
- **Esc**: Cancel current operation or go back
- **?**: Show help menu

#### Session Management

```bash
# CLI: Create session
cargo run -- create-session --name "project-review"

# CLI: List sessions
cargo run -- list-sessions

# TUI: Use Ctrl+N to create new session
# TUI: Use Ctrl+L to list and switch sessions
```

#### File Operations

```bash
# In TUI chat, use natural language:
"Read the file src/main.rs"
"Write hello world to test.txt"
"List all files in src/"
"Search for function names in src/"
```

#### Git Operations

```bash
# In TUI chat, use natural language:
"Show git status"
"Commit changes with message 'fix bug'"
"View diff of main.rs"
"Create new branch feature/login"
```

### Method 2: Usage via Programmatic API

```rust
use agent_tui::presentation::di::DIContainer;
use agent_tui::modules::session::domain::models::Session;

// Create DI container
let container = DIContainer::new();

// Create session use case
let create_session = container.create_session_use_case();
let session = create_session.execute("my-session").await?;

// Send message use case
let send_message = container.send_message_use_case();
let response = send_message.execute(session_id, message).await?;
```

## Reference

Comprehensive reference documentation for configuration options, commands, and external resources.

### Configuration Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `log_level` | string | `info` | Logging level (debug, info, warn, error) |
| `theme` | string | `default` | UI theme name |
| `default_provider` | string | `openai` | Default AI provider |
| `model` | string | `gpt-4` | Default AI model |
| `temperature` | float | `0.7` | AI response temperature (0.0-1.0) |
| `max_tokens` | integer | `4096` | Maximum tokens per response |
| `database_path` | string | `~/.agent-tui/sessions.db` | SQLite database path |
| `auto_backup` | boolean | `true` | Enable automatic database backups |
| `backup_interval` | string | `24h` | Backup interval duration |
| `git_enabled` | boolean | `true` | Enable Git integration |
| `git_auto_stage` | boolean | `false` | Auto-stage files before commit |
| `lsp_enabled` | boolean | `true` | Enable LSP integration |
| `mcp_enabled` | boolean | `true` | Enable MCP integration |
| `mcp_server_url` | string | `http://localhost:3000` | MCP server URL |

### CLI Commands

Complete list of available CLI commands with descriptions and examples.

| Command | Description | Example |
|---------|-------------|---------|
| `run` | Start TUI interface | `cargo run -- run` |
| `run --session <id>` | Run with specific session | `cargo run -- run --session abc123` |
| `run --agent <id>` | Run with specific agent | `cargo run -- run --agent default` |
| `list-sessions` | List all sessions | `cargo run -- list-sessions` |
| `list-agents` | List all agents | `cargo run -- list-agents` |
| `create-session --name <name>` | Create new session | `cargo run -- create-session --name my-session` |
| `version` | Show version info | `cargo run -- version` |

### Documentation

Links to detailed project documentation and guides.

**Project**
- [Project Overview](docs/project/overview.md) - Project overview and purpose
- [Features](docs/project/features.md) - Complete feature list
- [Key Concepts](docs/project/key-concept.md) - Core concepts and terminology
- [Principles](docs/project/principles.md) - Architecture principles and best practices
- [Roadmap](docs/project/roadmap.md) - Development roadmap and milestones
- [Changelog](docs/project/changelog.md) - Version history and changes
- [FAQ](docs/project/faq.md) - Frequently asked questions
- [Vision](docs/project/vision.md) - Project vision and mission

**Getting Started**
- [Installation](docs/getting-started/installation.md) - Installation guide
- [Configuration](docs/getting-started/configuration.md) - Configuration options
- [Usage](docs/getting-started/usage.md) - Basic usage examples
- [Quick Start](docs/getting-started/quick-start.md) - Get started in 5 minutes
- [Troubleshooting](docs/getting-started/troubleshooting.md) - Common issues and solutions

**Guides**
- [Clean Architecture](docs/guides/architecture/clean-architecture.md) - Architecture guide
- [Development Workflow](docs/guides/development/development-workflow.md) - Development guide
- [Testing Strategies](docs/guides/testing/testing-strategies.md) - Testing guide
- [Deployment Guide](docs/guides/deployment/deployment-guide.md) - Deployment guide
- [Contribution Guidelines](docs/guides/contributing/contribution-guidelines.md) - Contribution guide

**API**
- [Session Module API](docs/api/modules/session-module.md) - Session module documentation
- [Agent Module API](docs/api/modules/agent-module.md) - Agent module documentation
- [TUI Components API](docs/api/components/tui-components.md) - TUI components documentation
- [Helper Functions](docs/api/utilities/helper-functions.md) - Utility functions documentation

**Examples**
- [Basic Examples](docs/examples/basic/) - Basic usage examples
- [Advanced Examples](docs/examples/advanced/) - Advanced usage examples

**Reference**
- [Domain Types](docs/reference/types/domain-types.md) - Domain type definitions
- [Domain Functions](docs/reference/functions/domain-functions.md) - Domain function signatures
- [Configuration Options](docs/reference/config/configuration-options.md) - Configuration reference
- [CLI Commands](docs/reference/cli.md) - CLI commands reference
- [Environment Variables](docs/reference/env-vars.md) - Environment variables reference
- [Constants](docs/reference/constants.md) - Constants and enums
- [Error Types](docs/reference/errors.md) - Error codes and handling

**Planning**
- [Idea Features](idea-features.md) - Feature ideas for markdown rendering, streaming animation, and UX/UI improvements
- [Test Specification](spec/index.md) - Complete test specification with all test suites and test cases

### Related Projects

- [wrikka/agent](../../packages/agent) - Core agent package with Clean Architecture
- [wrikka/canvas](../../packages/canvas) - Canvas integration for visual editing

### External Resources

- [Ratatui Documentation](https://docs.rs/ratatui/)
- [Crossterm Documentation](https://docs.rs/crossterm/)
- [Clean Architecture](https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html)
- [Functional Programming in Rust](https://rust-lang.github.io/what-is-ownership/)

## Notes

Additional tips, warnings, and important information for using the application effectively.

> [!TIP]
> - Save sessions frequently with `Ctrl+S` to prevent data loss
> - Configure multiple AI providers for fallback support and reliability
> - Use plan mode to review AI suggestions before execution
> - Enable Git integration for automatic commit suggestions

> [!IMPORTANT]
> - The project supports 30+ AI providers including OpenAI, Anthropic, Google, and more
> - Session data is stored in SQLite by default for persistence
> - Git integration is optional and can be disabled in configuration
> - LSP and MCP integrations enhance development capabilities

> [!WARNING]
> - Ensure API keys are stored securely in environment variables
> - File operations require explicit confirmation for destructive actions
> - Command execution is sandboxed but still requires caution
> - Review all AI-generated commands before running them

> [!CAUTION]
> - Never run untrusted commands without reviewing them first
> - Command execution has timeout protection for safety
> - Always verify file paths before destructive operations
> - Database backups are recommended before major changes

## License

This project is licensed under the MIT License - see [LICENSE](../../LICENSE) for details.

## History

Star history chart showing the project's popularity over time.

[![Star History Chart](https://api.star-history.com/svg?repos=your-org/rust-packages&type=Date)](https://star-history.com/#your-org/rust-packages&Date)
