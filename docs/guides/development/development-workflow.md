---
title: Development Workflow
description: Development workflow, testing, และ debugging
---

# Development Workflow

## Setup

### Prerequisites

- Rust 1.70+
- Moon build system (optional, for monorepo)
- Git

### Initial Setup

```bash
# Clone repository
git clone https://github.com/your-org/rust-packages.git
cd rust-packages/apps/agent-tui

# Install dependencies
cargo fetch

# (Optional) Install Moon
npm install -g @moonrepo/cli
```

## Development Tasks

### Using Moon (Recommended)

```bash
# Development mode
moon run agent-tui:dev

# Watch mode
moon run agent-tui:watch

# Build
moon run agent-tui:build

# Build debug
moon run agent-tui:build_dev
```

### Using Cargo Directly

```bash
# Development mode
cargo run --bin agent-tui

# Watch mode (requires cargo-watch)
cargo watch --bin agent-tui

# Build release
cargo build --release

# Build debug
cargo build
```

## Code Quality

### Linting

```bash
# Run Clippy
cargo clippy --all-targets --all-features

# Fix Clippy warnings
cargo clippy --fix --allow-dirty

# Check for specific lints
cargo clippy -- -W clippy::all
```

### Formatting

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt --check
```

### Type Checking

```bash
# Type check
cargo check

# Type check with all features
cargo check --all-features
```

## Testing

### Unit Tests

```bash
# Run all tests
cargo test

# Run unit tests only
cargo test --lib

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture
```

### Integration Tests

```bash
# Run integration tests
cargo test --test '*'

# Run specific integration test
cargo test --test integration_test_name
```

### Test Coverage

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html
```

## Debugging

### Debug Builds

```bash
# Build with debug symbols
cargo build

# Run with debug symbols
cargo run --bin agent-tui
```

### Logging

```bash
# Enable debug logging
RUST_LOG=debug cargo run --bin agent-tui

# Enable trace logging
RUST_LOG=trace cargo run --bin agent-tui

# Enable specific module logging
RUST_LOG=agent_tui::modules::session=debug cargo run --bin agent-tui
```

### Debugging with IDE

**VS Code**:
1. Install Rust Analyzer extension
2. Set breakpoints in code
3. Press F5 to start debugging
4. Use debug console for inspection

**IntelliJ IDEA**:
1. Install Rust plugin
2. Set breakpoints
3. Click debug button
4. Use debugger tools

## Common Development Tasks

### Adding a New Module

```bash
# Create module structure
mkdir -p src/modules/new_module/{domain,application,ports}

# Create module files
touch src/modules/new_module/mod.rs
touch src/modules/new_module/domain/models.rs
touch src/modules/new_module/domain/operations.rs
touch src/modules/new_module/application/usecases.rs
touch src/modules/new_module/ports.rs

# Add to lib.rs
echo "pub mod new_module;" >> src/lib.rs
```

### Adding a New Use Case

```rust
// 1. Define domain operation
pub fn operation_name(input: Input) -> Result<Output, Error> {
    // Pure logic
}

// 2. Create use case
pub struct UseCaseName {
    dependency: Arc<dyn Dependency>,
}

impl UseCaseName {
    pub async fn execute(&self, input: Input) -> Result<Output> {
        let output = operation_name(input)?;
        self.dependency.save(output.clone()).await?;
        Ok(output)
    }
}

// 3. Add to DI container
impl DIContainer {
    pub fn use_case_name(&self) -> UseCaseName {
        UseCaseName {
            dependency: self.dependency.clone(),
        }
    }
}
```

### Adding a New Adapter

```rust
// 1. Implement port
pub struct NewAdapter {
    // Fields
}

impl PortTrait for NewAdapter {
    async fn method(&self) -> Result<()> {
        // Implementation
    }
}

// 2. Register in DI container
impl DIContainer {
    pub fn new() -> Self {
        let adapter = Arc::new(NewAdapter::new());
        Self { adapter }
    }
}
```

## Git Workflow

### Branch Strategy

```bash
# Create feature branch
git checkout -b feature/new-feature

# Make changes
git add .
git commit -m "feat: add new feature"

# Push to remote
git push origin feature/new-feature

# Create PR
# (via GitHub UI)
```

### Commit Convention

```bash
# Features
git commit -m "feat: add session persistence"

# Fixes
git commit -m "fix: resolve memory leak in message handling"

# Documentation
git commit -m "docs: update architecture guide"

# Refactoring
git commit -m "refactor: simplify domain operations"

# Tests
git commit -m "test: add integration tests for session module"
```

## Performance Profiling

### Using Criterion

```bash
# Install criterion
cargo install cargo-criterion

# Run benchmarks
cargo criterion
```

### Using Flamegraph

```bash
# Install flamegraph
cargo install flamegraph

# Generate flamegraph
cargo flamegraph --bin agent-tui
```

## CI/CD

### GitHub Actions

```yaml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo test
      - run: cargo clippy
      - run: cargo fmt --check
```

## Troubleshooting

### Build Errors

```bash
# Clean build artifacts
cargo clean

# Update dependencies
cargo update

# Check for outdated dependencies
cargo outdated
```

### Test Failures

```bash
# Run tests with backtrace
RUST_BACKTRACE=1 cargo test

# Run specific test file
cargo test --test test_file_name

# Run tests in single thread
cargo test -- --test-threads=1
```

### Dependency Issues

```bash
# Update Cargo.lock
cargo update

# Remove Cargo.lock and regenerate
rm Cargo.lock
cargo fetch
```

## Best Practices

1. **Write Tests First**: TDD approach for domain logic
2. **Keep Functions Small**: Single responsibility
3. **Use Type Safety**: Leverage Rust's type system
4. **Document Public APIs**: Use `///` for documentation
5. **Handle Errors Properly**: Use Result types
6. **Avoid Unwraps**: Use proper error handling
7. **Use Clippy**: Follow linter suggestions
8. **Format Code**: Use rustfmt consistently
9. **Review PRs**: Code review before merge
10. **Update Documentation**: Keep docs in sync
