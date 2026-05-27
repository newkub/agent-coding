---
title: Contribution Guidelines
description: Contribution guidelines สำหรับ project
---

# Contribution Guidelines

## Getting Started

### Prerequisites

- Rust 1.70+
- Git
- GitHub account

### Setup Development Environment

```bash
# Fork repository
# Click "Fork" on GitHub

# Clone your fork
git clone https://github.com/your-username/rust-packages.git
cd rust-packages/apps/agent-tui

# Add upstream remote
git remote add upstream https://github.com/original-org/rust-packages.git

# Install dependencies
cargo fetch
```

## Development Workflow

### 1. Create Branch

```bash
# Sync with upstream
git fetch upstream
git checkout main
git merge upstream/main

# Create feature branch
git checkout -b feature/your-feature-name
```

### 2. Make Changes

```bash
# Make your changes
# Follow code style guidelines
# Write tests
# Update documentation
```

### 3. Test Changes

```bash
# Run tests
cargo test

# Run linter
cargo clippy --all-targets --all-features

# Format code
cargo fmt

# Type check
cargo check
```

### 4. Commit Changes

```bash
# Stage changes
git add .

# Commit with conventional commits
git commit -m "feat: add new feature"

# Or
git commit -m "fix: resolve bug in session handling"
```

### 5. Push and Create PR

```bash
# Push to your fork
git push origin feature/your-feature-name

# Create PR on GitHub
# Provide clear description
# Link to relevant issues
```

## Code Style

### Rust Guidelines

- Follow Rust API Guidelines
- Use `rustfmt` for formatting
- Use `clippy` for linting
- Document public APIs
- Use meaningful names

### Clean Architecture Guidelines

- Keep domain layer pure
- Use traits for abstractions
- Inject dependencies
- Avoid circular dependencies
- Test domain logic first

## Commit Convention

Use [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

### Types

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks
- `perf`: Performance improvements
- `ci`: CI/CD changes

## Pull Request Guidelines

### PR Title

Use conventional commits format

### PR Description

Include:
- **What**: What does this PR do?
- **Why**: Why is this change needed?
- **How**: How does it work?
- **Testing**: How was it tested?
- **Breaking Changes**: Any breaking changes?
- **Related Issues**: Link to relevant issues

### PR Checklist

- [ ] Tests pass
- [ ] Clippy passes
- [ ] Code formatted
- [ ] Documentation updated
- [ ] Commit messages follow convention
- [ ] PR description is clear
- [ ] No merge conflicts
