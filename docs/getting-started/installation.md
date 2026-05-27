---
title: Installation
description: วิธีติดตั้งและ dependencies
---

# Installation

## Prerequisites

- Rust 1.70 หรือใหม่กว่า
- SQLite 3 (สำหรับ session persistence)
- Git (optional, สำหรับ Git integration features)

## Build from Source

### Step 1: Clone Repository

```bash
git clone https://github.com/your-org/rust-packages.git
cd rust-packages/apps/agent-tui
```

### Step 2: Build Release Binary

```bash
cargo build --release
```

Binary จะอยู่ที่: `target/release/agent-tui`

### Step 3: (Optional) Add to PATH

```bash
# Linux/macOS
export PATH="$PATH:$PWD/target/release"

# Windows (PowerShell)
$env:PATH += ";$PWD\target\release"
```

## Using Moon (Recommended for Monorepo)

### Development Mode

```bash
# Run in development mode
moon run agent-tui:dev

# Run with watch mode
moon run agent-tui:watch
```

### Build Mode

```bash
# Build release binary
moon run agent-tui:build
```

## Using Cargo Directly

### Development Mode

```bash
# Run TUI interface
cargo run --bin agent-tui

# Run with specific arguments
cargo run --bin agent-tui -- run --session <session-id>
```

### Build Mode

```bash
# Build debug binary
cargo build

# Build release binary
cargo build --release
```

## Installation via Cargo (Future)

เมื่อ publish ไปยัง crates.io:

```bash
cargo install agent-tui
```

## Verify Installation

```bash
# Check version
agent-tui --version

# Show help
agent-tui --help
```

## Dependencies

### Runtime Dependencies

- **Rust** - 1.70+ (สำหรับ build)
- **SQLite** - 3.x (สำหรับ session storage)
- **Git** - 2.x (optional, สำหรับ Git features)

### Development Dependencies

- **Cargo** - Rust package manager
- **Moon** - Build system (สำหรับ monorepo)
- **Clippy** - Rust linter
- **rustfmt** - Rust formatter

## Platform-Specific Notes

### Linux

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install SQLite
sudo apt-get install sqlite3  # Debian/Ubuntu
sudo yum install sqlite3      # CentOS/RHEL
```

### macOS

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# SQLite มาพร้อมกับ macOS
```

### Windows

```bash
# Install Rust จาก https://rustup.rs/
# หรือใช้ winget
winget install Rustlang.Rustup

# SQLite ดาวน์โหลดจาก https://www.sqlite.org/download.html
```

## Troubleshooting

### Rust Not Found

```bash
# Ensure Rust is in PATH
source $HOME/.cargo/env  # Linux/macOS
# หรือ restart terminal
```

### SQLite Not Found

```bash
# Linux
sudo apt-get install sqlite3

# macOS
brew install sqlite3

# Windows
# Download จาก https://www.sqlite.org/download.html
```

### Build Errors

```bash
# Update Rust toolchain
rustup update

# Clean build artifacts
cargo clean

# Rebuild
cargo build --release
```

### Moon Not Found

```bash
# Install Moon
npm install -g @moonrepo/cli

# หรือใช้ cargo
cargo install moon
```
