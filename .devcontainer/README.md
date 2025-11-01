# Dev Container for JSON Viewer

This devcontainer provides a complete Rust development environment for building and testing the JSON viewer application.

## What's Included

- **Rust toolchain** (latest stable)
- **Cargo** package manager
- **Cross-compilation targets** (x86_64 and aarch64)
- **VS Code extensions**:
  - rust-analyzer (code intelligence)
  - Even Better TOML (Cargo.toml support)
  - crates (dependency management)
  - CodeLLDB (debugging support)

## Getting Started

1. Install [Docker Desktop](https://www.docker.com/products/docker-desktop)
2. Install the [Dev Containers extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers) for VS Code
3. Open this project in VS Code
4. Click "Reopen in Container" when prompted (or use Command Palette: "Dev Containers: Reopen in Container")

## Building and Running

Once inside the container:

```bash
# Build the project
cargo build --release

# Run with examples
cargo run -- examples/

# Run tests
cargo test

# Check code with clippy
cargo clippy

# Format code
cargo fmt
```

## Creating .deb Package

```bash
# Install debian packaging tools
sudo apt-get update
sudo apt-get install -y dpkg-dev debhelper

# Make build script executable and run it
chmod +x build.sh
./build.sh
```

## Features

- Persistent target directory (mounted as volume for faster builds)
- Pre-configured Rust tools and formatters
- Cross-compilation support for both x86_64 and ARM64
- Full debugging capabilities
