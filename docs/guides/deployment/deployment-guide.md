---
title: Deployment Guide
description: Deployment strategies และ CI/CD
---

# Deployment Guide

## Build for Release

### Using Cargo

```bash
# Build release binary
cargo build --release

# Binary location
# target/release/agent-tui
```

### Using Moon

```bash
# Build release
moon run agent-tui:build

# Binary location
# target/release/agent-tui
```

## Cross-Platform Builds

### Linux

```bash
# Build for Linux
cargo build --release --target x86_64-unknown-linux-gnu

# Install target
rustup target add x86_64-unknown-linux-gnu
```

### macOS

```bash
# Build for macOS (Intel)
cargo build --release --target x86_64-apple-darwin

# Build for macOS (Apple Silicon)
cargo build --release --target aarch64-apple-darwin

# Install targets
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
```

### Windows

```bash
# Build for Windows
cargo build --release --target x86_64-pc-windows-msvc

# Install target
rustup target add x86_64-pc-windows-msvc
```

## Packaging

### Linux

```bash
# Create package
mkdir -p agent-tui-linux
cp target/release/agent-tui agent-tui-linux/
cp README.md agent-tui-linux/
cp LICENSE agent-tui-linux/

# Create tarball
tar -czf agent-tui-linux-x86_64.tar.gz agent-tui-linux/
```

### macOS

```bash
# Create package
mkdir -p agent-tui-macos
cp target/release/agent-tui agent-tui-macos/
cp README.md agent-tui-macos/
cp LICENSE agent-tui-macos/

# Create tarball
tar -czf agent-tui-macos-x86_64.tar.gz agent-tui-macos/
```

### Windows

```bash
# Create package
mkdir agent-tui-windows
copy target\release\agent-tui.exe agent-tui-windows\
copy README.md agent-tui-windows\
copy LICENSE agent-tui-windows\

# Create zip
powershell Compress-Archive -Path agent-tui-windows\* -DestinationPath agent-tui-windows-x86_64.zip
```

## CI/CD

### GitHub Actions

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
        target: [x86_64-unknown-linux-gnu, x86_64-apple-darwin, x86_64-pc-windows-msvc]
    
    runs-on: ${{ matrix.os }}
    
    steps:
      - uses: actions/checkout@v2
      
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: ${{ matrix.target }}
      
      - name: Build
        run: cargo build --release --target ${{ matrix.target }}
      
      - name: Upload artifact
        uses: actions/upload-artifact@v2
        with:
          name: agent-tui-${{ matrix.target }}
          path: target/${{ matrix.target }}/release/agent-tui*
```

### Release Automation

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  release:
    runs-on: ubuntu-latest
    
    steps:
      - uses: actions/checkout@v2
      
      - name: Build all targets
        run: |
          cargo build --release --target x86_64-unknown-linux-gnu
          cargo build --release --target x86_64-apple-darwin
          cargo build --release --target aarch64-apple-darwin
          cargo build --release --target x86_64-pc-windows-msvc
      
      - name: Create release
        uses: softprops/action-gh-release@v1
        with:
          files: |
            target/x86_64-unknown-linux-gnu/release/agent-tui
            target/x86_64-apple-darwin/release/agent-tui
            target/aarch64-apple-darwin/release/agent-tui
            target/x86_64-pc-windows-msvc/release/agent-tui.exe
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

## Distribution

### Crates.io

```bash
# Login to crates.io
cargo login

# Publish
cargo publish
```

### Homebrew (macOS)

Create tap formula:

```ruby
# Formula/agent-tui.rb
class AgentTui < Formula
  desc "Terminal User Interface AI coding assistant"
  homepage "https://github.com/your-org/agent-tui"
  url "https://github.com/your-org/agent-tui/archive/v0.1.0.tar.gz"
  sha256 "..."
  license "MIT"

  depends_on "rust"

  def install
    system "cargo", "install", "--locked", "--root", prefix, "--path", "."
  end

  test do
    system "#{bin}/agent-tui", "--version"
  end
end
```

### Scoop (Windows)

Create bucket manifest:

```json
{
  "version": "0.1.0",
  "description": "Terminal User Interface AI coding assistant",
  "homepage": "https://github.com/your-org/agent-tui",
  "license": "MIT",
  "url": "https://github.com/your-org/agent-tui/releases/download/v0.1.0/agent-tui-windows-x86_64.zip",
  "hash": "...",
  "bin": "agent-tui.exe",
  "checkver": "github",
  "autoupdate": {
    "url": "https://github.com/your-org/agent-tui/releases/download/v$version/agent-tui-windows-x86_64.zip"
  }
}
```

### AUR (Arch Linux)

Create PKGBUILD:

```bash
pkgname=agent-tui
pkgver=0.1.0
pkgrel=1
pkgdesc="Terminal User Interface AI coding assistant"
arch=('x86_64')
url="https://github.com/your-org/agent-tui"
license=('MIT')
depends=('sqlite')
makedepends=('rust' 'git')

build() {
  cd "$srcdir/agent-tui-$pkgver"
  cargo build --release
}

package() {
  cd "$srcdir/agent-tui-$pkgver"
  install -Dm755 target/release/agent-tui "$pkgdir/usr/bin/agent-tui"
}
```

## Installation Methods

### Binary Installation

```bash
# Download binary
wget https://github.com/your-org/agent-tui/releases/download/v0.1.0/agent-tui-linux-x86_64.tar.gz

# Extract
tar -xzf agent-tui-linux-x86_64.tar.gz

# Install
sudo cp agent-tui-linux/agent-tui /usr/local/bin/
```

### Package Manager Installation

```bash
# Homebrew (macOS)
brew install agent-tui

# Scoop (Windows)
scoop install agent-tui

# AUR (Arch Linux)
yay -S agent-tui
```

### Cargo Installation

```bash
cargo install agent-tui
```

## Post-Installation

### Verification

```bash
# Check version
agent-tui --version

# Show help
agent-tui --help

# Run (will prompt for configuration)
agent-tui run
```

### Configuration

```bash
# Create config directory
mkdir -p ~/.agent-tui

# Create config file
cat > ~/.agent-tui/config.toml << EOF
[general]
log_level = "info"

[ai]
default_provider = "openai"
model = "gpt-4"
EOF

# Set environment variables
export OPENAI_API_KEY=your_api_key
```

## Troubleshooting

### Binary Not Found

```bash
# Add to PATH
export PATH="$PATH:/usr/local/bin"

# Or use full path
/usr/local/bin/agent-tui --version
```

### Permission Denied

```bash
# Make executable
chmod +x agent-tui

# Or reinstall with correct permissions
sudo cp agent-tui /usr/local/bin/
```

### Missing Dependencies

```bash
# Linux
sudo apt-get install sqlite3

# macOS
brew install sqlite3

# Windows
# Download from https://www.sqlite.org/download.html
```
