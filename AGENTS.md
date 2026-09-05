# AGENTS.md

## Project Overview

agent-tui: Terminal User Interface (TUI) AI coding assistant with Clean Architecture (FP-style)

## Development Phase

**Current Phase**: Feature Development

Project has:
- ✅ Cargo.toml with workspace dependencies and Standard scripts (dev, build, typecheck, lint, format, test, verify, ci, test_watch, test_coverage, deps_analyze, deps_update, clean)
- ✅ Source code structure (src/) with 48 Rust files
- ✅ Moon build system configuration
- ✅ README.md with comprehensive documentation
- ✅ Clean Architecture (FP-style) implementation
- ✅ idea-features.md with 40 features for markdown rendering, streaming animation, and UX/UI improvements
- ✅ Placeholder execution paths converted to real implementations (headless filesystem/shell/OpenAI, GitHub issue lookup, persistent automation workflows)
- ✅ Share links use `AGENT_TUI_SHARE_BASE_URL`; optional real notifications use `AGENT_TUI_SHARE_WEBHOOK_URL`
- ✅ Lib/bin architecture unified (main.rs uses agent_tui lib)
- ✅ Clippy clean (0 warnings) with wired session, audit, share, subagent, guardrail, and performance CLI features
- ✅ SQLite persistence layer with migrations for session, audit, collaboration, macro, headless, share, performance, automation, subagent, guardrail, notes, and snippets
- ✅ Production repositories (SqliteSessionRepository, SqliteAuditRepository, SqliteCollaborationRepository, SqliteMacroRepository, SqliteHeadlessSessionManager, SqliteSnapshotManager, SqliteOptimizationManager, SqliteAutomationWorkflowRepository, SqliteSubagentManager, SqliteGuardrailManager, SqliteUiContentRepository)
- ✅ Environment validation (OPENAI_API_KEY, GITHUB_TOKEN, share-link URL/webhook configuration, AGENT_TUI_DB_PATH) with tracing/observability
- ✅ SQLite backup/restore/verify CLI via `database` subcommands using online `VACUUM INTO` and `PRAGMA integrity_check`
- ✅ Bounded exponential HTTP retries with backoff for GitHub and share webhook requests
- ✅ x-correlation-id tracing and structured response-time logging for external HTTP and OpenAI calls
- ✅ Token-bucket rate limiting for GitHub and OpenAI API calls
- ✅ SQLite indexes added for common lookup paths in sessions, macros, headless, share, subagents, guardrails, and UI content
- ✅ End-to-end CLI test coverage for database backup/restore lifecycle

## Workflows

### Foundation Workflows (Execute First)

1. `/follow-rust` - Rust development best practices
2. `/follow-clean-architecture` - Clean Architecture (FP-style) implementation
3. `/follow-functional-programming` - Functional programming principles
4. `/setup-tasks` - Setup package.json/Cargo.toml scripts (adapted for Rust)

### Development Workflows

5. `/follow-clippy` - Clippy lint rules and error handling
6. `/follow-cargo` - Cargo lint rules and workspace lint configuration
7. `/follow-test-function` - Unit testing with Vitest (adapted for Rust)
8. `/follow-git` - Git best practices for software development

### Architecture Workflows

9. `/follow-feature-module-rules` - Rules for implementing feature modules in Clean Architecture 2
10. `/follow-ddd` - Domain-Driven Design for complex business logic

### Quality Workflows

11. `/follow-code-quality` - Code quality with separation of concerns, type safety, error handling
12. `/check-correctness` - Check correctness according to principles
13. `/check-architecture` - Check project architecture with tree view
14. `/check-consistency` - Check consistency of naming, style, and structure

### Documentation Workflows

15. `/update-readme-singlerepo` - Update README.md for single repo

## Skills

### Core Skills

1. `rust` - Rust development best practices (43 supporting files)
2. `testing` - Best practices for writing and maintaining tests (6 supporting files)
3. `git` - Git best practices (19 supporting files)
4. `moonrepo` - Build system and monorepo management (4 supporting files)

### Notes

- Skills `clean-architecture` and `functional-programming` are not available in the system
- Apply Clean Architecture (FP-style) principles manually following README.md architecture section
- Apply functional programming principles manually using Rust's functional features

## Execution Order

1. Foundation workflows first (1-4)
2. Development workflows (5-8)
3. Architecture workflows (9-10)
4. Quality workflows (11-14)
5. Documentation workflows (15)

## Notes

- This is a Rust project, skip JavaScript/TypeScript-specific workflows
- Use Cargo instead of npm/yarn
- Use Clippy instead of ESLint
- Use rustfmt instead of Prettier/Biome
- Focus on Clean Architecture (FP-style) principles
- Moon build system is used for task management
