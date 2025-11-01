# Development Guide

## Overview

This guide will help you set up the development environment and start contributing to the JSON Viewer project.

## Quick Start with DevContainer

The **easiest and recommended** way to develop is using the DevContainer:

```bash
# 1. Open in VS Code
code .

# 2. Reopen in Container
# Click the notification or use Command Palette:
# "Dev Containers: Reopen in Container"

# 3. Wait for setup (2-5 minutes first time)

# 4. Start coding!
```

Everything is pre-configured:
- ✅ Rust toolchain
- ✅ Code intelligence (rust-analyzer)
- ✅ Debugging support
- ✅ Cross-compilation targets
- ✅ Build tasks
- ✅ Code formatting and linting

## Project Structure

```
json-viewer/
├── src/
│   ├── main.rs           # Entry point, CLI parsing, event loop
│   ├── app.rs            # Application state and logic
│   ├── file_browser.rs   # File navigation component
│   ├── json_viewer.rs    # JSON parsing and viewing
│   └── ui.rs             # Terminal UI rendering
├── examples/             # Sample JSON files for testing
├── debian/               # Debian packaging files
├── .devcontainer/        # Dev container configuration
├── .github/              # CI/CD workflows
└── Cargo.toml           # Rust dependencies
```

## Architecture

### Main Components

1. **main.rs**: Application entry point
   - Parses CLI arguments
   - Sets up terminal
   - Runs event loop
   - Handles keyboard input

2. **app.rs**: Application state
   - Manages file browser state
   - Manages JSON viewer state
   - Handles mode switching (Normal/Query)
   - Handles view switching (Plain/Hierarchical)

3. **file_browser.rs**: File navigation
   - Scans directory for JSON files
   - Handles cursor navigation
   - Provides file selection

4. **json_viewer.rs**: JSON handling
   - Loads and parses JSON files
   - Formats hierarchical view
   - Executes JQL queries

5. **ui.rs**: Terminal UI
   - Renders 3-panel layout
   - Handles styling and colors
   - Updates display

### Data Flow

```
User Input → main.rs → app.rs → file_browser/json_viewer
                 ↓
             ui.rs (render)
                 ↓
            Terminal Display
```

## Making Changes

### Adding a New Feature

1. **Identify the component** where the feature belongs
2. **Update the relevant module** (app, file_browser, json_viewer, or ui)
3. **Test your changes** with `cargo run -- examples/`
4. **Format code** with `cargo fmt`
5. **Check lints** with `cargo clippy`

### Example: Adding a New Keyboard Shortcut

Edit `src/main.rs`:

```rust
// In run_app function, Normal mode match
KeyCode::Char('h') => {
    // Show help dialog
    app.show_help();
}
```

Then implement in `src/app.rs`:

```rust
impl App {
    pub fn show_help(&mut self) {
        // Implementation
    }
}
```

Update UI in `src/ui.rs` to display help.

### Example: Adding a New View Mode

1. Edit `src/app.rs`:
```rust
pub enum ViewMode {
    PlainText,
    Hierarchical,
    CompactView,  // New mode
}
```

2. Implement formatting in `src/json_viewer.rs`:
```rust
pub fn get_compact_view(&self) -> Option<String> {
    // Implementation
}
```

3. Update UI in `src/ui.rs`:
```rust
ViewMode::CompactView => {
    app.json_viewer.get_compact_view()
        .unwrap_or_else(|| "...".to_string())
}
```

## Development Workflow

### 1. Daily Development

```bash
# Pull latest changes
git pull

# Create feature branch
git checkout -b feature/my-feature

# Make changes, then test
cargo run -- examples/

# Check for errors
cargo clippy

# Format code
cargo fmt

# Commit changes
git add .
git commit -m "feat: add my feature"

# Push
git push origin feature/my-feature
```

### 2. Testing

```bash
# Run with examples
make run

# Test with your own data
cargo run -- /path/to/json/files

# Run unit tests (when added)
cargo test

# Check code quality
make clippy
```

### 3. Building

```bash
# Debug build (fast, for development)
make build

# Release build (optimized)
make release

# Create .deb package
make package
```

## Debugging

### Using VS Code Debugger

1. Set breakpoints by clicking left of line numbers
2. Press `F5` to start debugging
3. Use debug controls to step through code

### Using Print Debugging

```rust
// Add to your code
eprintln!("Debug: value = {:?}", some_value);
```

Run with:
```bash
cargo run -- examples/ 2> debug.log
```

### Using Rust Backtrace

```bash
RUST_BACKTRACE=1 cargo run -- examples/
```

## Code Style

### Rust Conventions

- Use `snake_case` for functions and variables
- Use `CamelCase` for types and structs
- Maximum line length: 100 characters
- Use `rustfmt` for consistent formatting

### Formatting

```bash
# Format all code
cargo fmt

# Check formatting without changing files
cargo fmt -- --check
```

### Linting

```bash
# Run clippy with all warnings
cargo clippy -- -D warnings
```

## Adding Dependencies

Edit `Cargo.toml`:

```toml
[dependencies]
new-crate = "1.0"
```

Then:
```bash
cargo fetch
cargo build
```

## Testing

### Manual Testing

See `TESTING.md` for comprehensive testing guide.

### Writing Unit Tests

Add tests to your modules:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_browser_navigation() {
        let browser = FileBrowser::new(Path::new("examples/")).unwrap();
        assert!(browser.files.len() > 0);
    }
}
```

Run tests:
```bash
cargo test
```

## Performance Optimization

### Profiling

```bash
# Build with debug info
cargo build --release --profile=release-with-debug

# Use a profiler (e.g., perf, flamegraph)
```

### Benchmarking

Add benchmarks in `benches/` directory using criterion.

## Documentation

### Code Documentation

```rust
/// Loads a JSON file and parses its content.
///
/// # Arguments
/// * `path` - Path to the JSON file
///
/// # Returns
/// Result with parsed JSON or error
pub fn load_file(&mut self, path: &PathBuf) -> Result<()> {
    // Implementation
}
```

Generate docs:
```bash
cargo doc --open
```

## Release Process

### Version Bump

1. Update version in `Cargo.toml`
2. Update version in `debian/changelog`
3. Update version in `debian/control`

### Create Release

```bash
# Build for all architectures
./build-cross.sh

# Create packages
./build.sh

# Tag release
git tag v0.1.0
git push --tags
```

## Troubleshooting

### Compilation Errors

```bash
# Clean and rebuild
cargo clean
cargo build
```

### Dependency Issues

```bash
# Update dependencies
cargo update

# Check for outdated deps
cargo outdated
```

### Terminal Issues

The TUI requires a proper terminal. Use:
- VS Code integrated terminal ✅
- Standard terminal emulator ✅
- Not VS Code debug console ❌

## Resources

- **Rust Book**: https://doc.rust-lang.org/book/
- **Ratatui Docs**: https://ratatui.rs/
- **JQL Syntax**: https://github.com/yamafaktory/jql
- **Crossterm**: https://docs.rs/crossterm/

## Getting Help

- Read `README.md` for user documentation
- Read `QUICKSTART.md` for basic usage
- Read `TESTING.md` for testing guide
- Check existing issues on GitHub
- Ask questions in discussions

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Ensure `cargo clippy` and `cargo fmt` pass
6. Submit a pull request

## License

MIT License - see LICENSE file for details
