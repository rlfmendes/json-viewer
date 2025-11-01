# 🎉 JSON Viewer - DevContainer Ready!

Your JSON Viewer project is now fully configured with a DevContainer for easy testing and development.

## 🚀 Quick Start (3 Steps)

### 1. Open in DevContainer

```bash
# In VS Code, open the project
code /Users/ricardo/json-viewer

# Click "Reopen in Container" when prompted
# Or: Cmd+Shift+P → "Dev Containers: Reopen in Container"
```

### 2. Wait for Setup (First Time Only)

The container will automatically:
- ✅ Pull the Rust development image (~2-3 min)
- ✅ Install Rust components (clippy, rustfmt, rust-src)
- ✅ Add cross-compilation targets (x86_64, ARM64)
- ✅ Fetch all project dependencies

### 3. Run the Application

```bash
# Inside the container terminal:
cargo run -- examples/

# Or use the Makefile:
make run
```

## 📋 What's Included in the DevContainer

### Development Environment
- 🦀 **Rust toolchain** (stable, latest)
- 🔧 **Cargo** (package manager)
- 🎯 **Cross-compilation** support (x86_64 + ARM64)
- 🐛 **Debugging** support (CodeLLDB)

### VS Code Extensions
- **rust-analyzer** - Code intelligence, autocomplete, go-to-definition
- **Even Better TOML** - Cargo.toml syntax support
- **crates** - Dependency version management
- **CodeLLDB** - Debugger integration

### Pre-configured
- ✅ Build tasks (Ctrl+Shift+B)
- ✅ Debug configurations (F5)
- ✅ Code formatting (cargo fmt)
- ✅ Linting (cargo clippy)
- ✅ Persistent cargo cache

## 🎮 Using the Application

### Keyboard Controls
- `↑/↓` - Navigate through files
- `Enter` - Select and open a file
- `Tab` - Toggle between plain text and hierarchical view
- `/` - Enter query mode
- `Esc` - Exit query mode
- `q` or `Ctrl+C` - Quit application

### Example Queries (Press `/` first)
```
".[0].name"              # First person's name
".[].age"                # All ages
".[].address.city"       # All cities
".employees[0]"          # First employee
".projects.active"       # Active projects
```

## 🛠️ Development Commands

### Quick Commands (Using Makefile)
```bash
make run        # Run with examples
make build      # Debug build
make release    # Release build
make test       # Run tests
make clippy     # Lint code
make fmt        # Format code
make package    # Create .deb package
make clean      # Clean artifacts
```

### Direct Cargo Commands
```bash
# Development
cargo build                    # Debug build
cargo build --release          # Optimized build
cargo run -- examples/         # Run with examples
cargo run -- /path/to/files/   # Run with custom path

# Code Quality
cargo test                     # Run tests
cargo clippy                   # Lint with suggestions
cargo fmt                      # Auto-format code
cargo check                    # Quick compile check

# Documentation
cargo doc --open              # Generate and view docs
```

## 📦 Creating Installation Package

### Build .deb Package
```bash
# Inside container:
./build.sh

# Package created at:
# target/json-viewer_0.1.0_amd64.deb (or arm64)
```

### Cross-compile for Both Architectures
```bash
./build-cross.sh

# Binaries at:
# target/x86_64-unknown-linux-gnu/release/json-viewer
# target/aarch64-unknown-linux-gnu/release/json-viewer
```

## 📚 Documentation

| Document | Purpose |
|----------|---------|
| `README.md` | Main documentation and features |
| `QUICKSTART.md` | Getting started guide |
| `TESTING.md` | Comprehensive testing guide |
| `DEVELOPMENT.md` | Developer guide and architecture |
| `.devcontainer/README.md` | DevContainer details |

## 🔍 Verification

Run the environment check:
```bash
./check-env.sh
```

This verifies:
- ✅ Rust and Cargo installed
- ✅ Required components available
- ✅ Targets installed
- ✅ Project structure correct
- ✅ Dependencies fetchable
- ✅ Project builds successfully

## 🐛 Debugging

### Using VS Code Debugger
1. Set breakpoints (click left of line numbers)
2. Press `F5` to start debugging
3. Application runs with debugger attached
4. Use step over/into/out controls

### Viewing Logs
```bash
# Run with backtrace
RUST_BACKTRACE=1 cargo run -- examples/

# Save debug output
cargo run -- examples/ 2> debug.log
```

## 🎯 Project Structure

```
json-viewer/
├── src/                      # Source code
│   ├── main.rs              # Entry point & event loop
│   ├── app.rs               # Application state
│   ├── file_browser.rs      # File navigation
│   ├── json_viewer.rs       # JSON parsing & viewing
│   └── ui.rs                # Terminal UI rendering
├── examples/                 # Sample JSON files
│   ├── sample-data.json     # Simple array example
│   └── nested-data.json     # Complex nested example
├── debian/                   # Packaging files
│   ├── control              # Package metadata
│   ├── changelog            # Version history
│   └── rules                # Build rules
├── .devcontainer/           # DevContainer config
│   ├── devcontainer.json   # Main configuration
│   ├── post-create.sh      # Setup script
│   └── README.md           # Container docs
├── .vscode/                 # VS Code settings
│   ├── tasks.json          # Build tasks
│   └── launch.json         # Debug configs
├── .github/                 # CI/CD
│   └── workflows/
│       └── build.yml       # Build workflow
├── Cargo.toml              # Rust dependencies
├── build.sh                # Build & package script
├── check-env.sh            # Environment check
└── Makefile                # Convenience commands
```

## 🧪 Testing Checklist

Basic functionality:
- [ ] Opens and lists JSON files
- [ ] Navigates with arrow keys
- [ ] Displays file content
- [ ] Switches between view modes
- [ ] Executes queries
- [ ] Exits cleanly

See `TESTING.md` for detailed testing procedures.

## 💡 Tips

### First Run
The first build will take a few minutes as it downloads and compiles dependencies. Subsequent builds are much faster thanks to caching.

### Terminal UI
The TUI requires a proper interactive terminal:
- ✅ Use VS Code integrated terminal (Ctrl+\`)
- ✅ Use external terminal emulator
- ❌ Don't use VS Code debug console

### Performance
The target directory is cached in a Docker volume for faster builds. If you need to clean it:
```bash
make clean
# or
cargo clean
```

## 🆘 Troubleshooting

### Container won't start
- Ensure Docker Desktop is running
- Try: "Dev Containers: Rebuild Container"

### Build errors
```bash
cargo clean
cargo build
```

### Dependencies issue
```bash
cargo update
cargo fetch
```

### Permission errors
```bash
chmod +x *.sh .devcontainer/*.sh
```

## 🎓 Learning Resources

- **Rust Book**: https://doc.rust-lang.org/book/
- **Ratatui Tutorial**: https://ratatui.rs/tutorials/
- **JQL Documentation**: https://github.com/yamafaktory/jql
- **Crossterm Docs**: https://docs.rs/crossterm/

## 🤝 Contributing

1. Create a feature branch
2. Make your changes
3. Run `cargo fmt` and `cargo clippy`
4. Test thoroughly
5. Submit a pull request

## ⚡ Next Steps

1. **Open in DevContainer** (see step 1 above)
2. **Run the application** (`make run`)
3. **Explore the code** (start with `src/main.rs`)
4. **Read the docs** (especially `DEVELOPMENT.md`)
5. **Start building!** 🚀

## 📬 Support

- Issues: Use GitHub Issues
- Questions: See documentation in repo
- Contributions: Pull requests welcome!

---

Built with ❤️ using Rust, Ratatui, and DevContainers

**Happy Coding!** 🎉
