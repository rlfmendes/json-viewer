# 📚 JSON Viewer - Documentation Index

Welcome to the JSON Viewer documentation! This index will help you find the information you need.

## 🚀 Getting Started

Start here if you're new to the project:

1. **[DEVCONTAINER_SETUP.md](DEVCONTAINER_SETUP.md)** ⭐ START HERE
   - Complete setup guide for DevContainer
   - Quick 3-step start process
   - All keyboard controls
   - Example queries
   - Troubleshooting tips

2. **[QUICKSTART.md](QUICKSTART.md)**
   - Minimal quick start guide
   - Basic usage examples
   - Query syntax examples

3. **[README.md](README.md)**
   - Project overview and features
   - Installation options (DevContainer, .deb, source)
   - Architecture support (x86_64, ARM64)
   - Basic usage and commands

## 🧪 Testing & Quality

4. **[TESTING.md](TESTING.md)**
   - Comprehensive testing guide
   - Manual testing procedures
   - Query examples
   - Build tests
   - Performance testing
   - Troubleshooting

5. **[check-env.sh](check-env.sh)** (Script)
   - Automated environment verification
   - Checks tools and dependencies
   - Quick health check

## 👨‍💻 Development

6. **[DEVELOPMENT.md](DEVELOPMENT.md)**
   - Complete developer guide
   - Project structure explanation
   - Architecture overview
   - Adding features
   - Code style guidelines
   - Debugging techniques
   - Release process

7. **[ARCHITECTURE.md](ARCHITECTURE.md)**
   - Visual diagrams of system architecture
   - Component interactions
   - Data flow diagrams
   - Build process
   - Technology stack

## 🏗️ Building & Packaging

8. **[build.sh](build.sh)** (Script)
   - Build and create .deb package
   - Auto-detects architecture
   - Creates installable package

9. **[build-cross.sh](build-cross.sh)** (Script)
   - Cross-compile for multiple architectures
   - Builds for x86_64 and ARM64

10. **[Makefile](Makefile)**
    - Convenient build commands
    - Common development tasks

## 🐳 DevContainer

11. **[.devcontainer/devcontainer.json](.devcontainer/devcontainer.json)**
    - DevContainer configuration
    - VS Code extensions
    - Container settings

12. **[.devcontainer/README.md](.devcontainer/README.md)**
    - DevContainer details
    - What's included
    - Setup process

13. **[.devcontainer/post-create.sh](.devcontainer/post-create.sh)** (Script)
    - Automatic setup script
    - Installs components and dependencies

## 🔧 Configuration Files

14. **[Cargo.toml](Cargo.toml)**
    - Rust project manifest
    - Dependencies
    - Build configuration

15. **[debian/](debian/)**
    - `control` - Package metadata
    - `changelog` - Version history
    - `rules` - Build rules for .deb creation

16. **[.vscode/](.vscode/)**
    - `tasks.json` - Build tasks (Ctrl+Shift+B)
    - `launch.json` - Debug configurations (F5)

17. **[.github/workflows/build.yml](.github/workflows/build.yml)**
    - CI/CD pipeline
    - Automated builds and tests

## 📋 Legal & License

18. **[LICENSE](LICENSE)**
    - MIT License
    - Usage terms

## 📊 Quick Reference

### File Organization by Purpose

#### 📘 User Documentation
- DEVCONTAINER_SETUP.md - Complete setup guide
- QUICKSTART.md - Quick start
- README.md - Main documentation
- TESTING.md - Testing guide

#### 💻 Developer Documentation
- DEVELOPMENT.md - Developer guide
- ARCHITECTURE.md - System architecture
- .devcontainer/README.md - Container details

#### 🔨 Build & Scripts
- build.sh - Build package
- build-cross.sh - Cross-compile
- check-env.sh - Environment check
- Makefile - Build commands
- .devcontainer/post-create.sh - Setup script

#### ⚙️ Configuration
- Cargo.toml - Rust config
- .devcontainer/devcontainer.json - Container config
- .vscode/tasks.json - VS Code tasks
- .vscode/launch.json - Debug config
- .github/workflows/build.yml - CI/CD
- debian/* - Package config

#### 📝 Source Code
- src/main.rs - Entry point
- src/app.rs - App state
- src/file_browser.rs - File navigation
- src/json_viewer.rs - JSON handling
- src/ui.rs - UI rendering

#### 🧪 Examples
- examples/sample-data.json - Simple example
- examples/nested-data.json - Complex example

## 🎯 Common Tasks → Documentation

| I want to... | Read this... |
|-------------|--------------|
| Get started quickly | [DEVCONTAINER_SETUP.md](DEVCONTAINER_SETUP.md) |
| Install and use the tool | [README.md](README.md) |
| Test the application | [TESTING.md](TESTING.md) |
| Contribute code | [DEVELOPMENT.md](DEVELOPMENT.md) |
| Understand the architecture | [ARCHITECTURE.md](ARCHITECTURE.md) |
| Build a .deb package | [build.sh](build.sh) |
| Set up development environment | [.devcontainer/README.md](.devcontainer/README.md) |
| Debug an issue | [DEVELOPMENT.md](DEVELOPMENT.md) (Debugging section) |
| Add a new feature | [DEVELOPMENT.md](DEVELOPMENT.md) (Making Changes) |
| Run queries | [QUICKSTART.md](QUICKSTART.md) (Example Queries) |

## 🆘 Troubleshooting → Documentation

| Problem | Check this... |
|---------|---------------|
| Can't start DevContainer | [DEVCONTAINER_SETUP.md](DEVCONTAINER_SETUP.md) (Troubleshooting) |
| Build errors | [DEVELOPMENT.md](DEVELOPMENT.md) (Troubleshooting) |
| Application won't run | [TESTING.md](TESTING.md) (Troubleshooting Tests) |
| Environment issues | Run [check-env.sh](check-env.sh) |
| Query not working | [TESTING.md](TESTING.md) (Query Examples) |
| Terminal UI issues | [TESTING.md](TESTING.md) (Terminal Not Interactive) |

## 📖 Reading Order by Experience Level

### Complete Beginner
1. DEVCONTAINER_SETUP.md
2. QUICKSTART.md
3. TESTING.md (basic tests)

### Regular User
1. README.md
2. QUICKSTART.md
3. TESTING.md

### Developer (New to Project)
1. README.md
2. DEVCONTAINER_SETUP.md
3. ARCHITECTURE.md
4. DEVELOPMENT.md
5. Source code (start with src/main.rs)

### Experienced Developer
1. ARCHITECTURE.md
2. DEVELOPMENT.md
3. Source code
4. Build scripts

### Package Maintainer
1. README.md
2. build.sh
3. debian/ directory
4. .github/workflows/

## 🔗 External Resources

- **Rust Programming**: https://doc.rust-lang.org/book/
- **Ratatui (TUI)**: https://ratatui.rs/
- **JQL Query Language**: https://github.com/yamafaktory/jql
- **Crossterm**: https://docs.rs/crossterm/
- **Cargo Book**: https://doc.rust-lang.org/cargo/

## 📌 Quick Command Reference

```bash
# Environment
./check-env.sh              # Check setup

# Running
make run                    # Run with examples
cargo run -- examples/      # Run directly
cargo run -- /path/to/dir   # Run on custom directory

# Building
make build                  # Debug build
make release                # Release build
make package                # Create .deb package

# Quality
make test                   # Run tests
make clippy                 # Lint code
make fmt                    # Format code

# Cleaning
make clean                  # Clean build artifacts
```

## 💡 Tips for Using This Documentation

1. **Start with DEVCONTAINER_SETUP.md** - It has everything you need to get running
2. **Use the index** - Jump to specific topics using the links
3. **Check troubleshooting sections** - Most common issues are documented
4. **Run check-env.sh** - Quick way to verify your setup
5. **Read the comments** - Source code has helpful comments

## 🎓 Learning Path

### Week 1: Getting Started
- [ ] Read DEVCONTAINER_SETUP.md
- [ ] Open project in DevContainer
- [ ] Run the application
- [ ] Try example queries
- [ ] Read QUICKSTART.md

### Week 2: Understanding the Code
- [ ] Read ARCHITECTURE.md
- [ ] Explore src/main.rs
- [ ] Understand the event loop
- [ ] Review other source files
- [ ] Read DEVELOPMENT.md

### Week 3: Contributing
- [ ] Read DEVELOPMENT.md thoroughly
- [ ] Make a small change
- [ ] Run tests and linting
- [ ] Build a package
- [ ] Understand CI/CD workflow

## 📬 Getting Help

If you can't find what you need:
1. Check the troubleshooting sections in relevant docs
2. Run `./check-env.sh` to verify your setup
3. Review error messages carefully
4. Check GitHub Issues
5. Read the source code (it's well-commented!)

---

**Happy exploring! 🚀**

For the fastest start, go to: **[DEVCONTAINER_SETUP.md](DEVCONTAINER_SETUP.md)**
