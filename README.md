# JSON Viewer

A fast, terminal-based JSON file viewer with a character-based UI, built in Rust.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)

## Features

- 📁 **File Browser**: Navigate through JSON files in a directory using arrow keys
  - 🔼 **Parent Directory Navigation**: Quick access to parent folders with ".." entry at top of list
  - 🔄 **Auto-Refresh**: Automatically detects and displays new/removed files in real-time
- 📄 **Dual View Modes**: 
  - Plain text view for raw JSON
  - Hierarchical tree view for structured browsing
- 🔍 **Query Support**: Simple dot/bracket queries to navigate and extract JSON data
- 📜 **Scrolling & Cursor**: Navigate through long JSON files with a highlighted cursor line
- 🔄 **Line Wrapping**: Toggle line wrapping on/off for better readability
- ⚡ **Fast & Lightweight**: Built with Rust for maximum performance
- 🖥️ **Cross-Platform**: Supports both x86_64 and ARM64 architectures
- 📦 **Easy Installation**: Install via `.deb` package or build from source

## Getting Started

### Development with DevContainer (Recommended)

The easiest way to develop and test this project is using the provided DevContainer:

1. **Prerequisites**:
   - [Docker Desktop](https://www.docker.com/products/docker-desktop)
   - [VS Code](https://code.visualstudio.com/)
   - [Dev Containers extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)

2. **Open in DevContainer**:
   ```bash
   # Open the project in VS Code
   code .
   
   # VS Code will prompt you to "Reopen in Container"
   # Or use Command Palette (Cmd+Shift+P): "Dev Containers: Reopen in Container"
   ```

3. **Build and Run**:
   ```bash
   # Inside the container
   cargo run -- examples/
   ```

The DevContainer includes:
- Complete Rust toolchain with rust-analyzer
- Cross-compilation targets (x86_64 and ARM64)
- All necessary VS Code extensions
- Pre-configured build tasks and debugging

## Installation

### Option 1: Install from .deb Package (Debian/Ubuntu)

```bash
# Download the appropriate package for your architecture
# For x86_64:
sudo apt-get install -f ./json-viewer_0.1.0_amd64.deb

# For ARM64:
sudo apt-get install -f ./json-viewer_0.1.0_arm64.deb
```

### Option 2: Build from Source

#### Prerequisites

- Rust 1.70 or later ([Install Rust](https://rustup.rs/))
- Cargo (comes with Rust)

#### Build and Install

```bash
# Clone the repository
git clone https://github.com/yourusername/json-viewer.git
cd json-viewer

# Build the project
cargo build --release

# The binary will be at target/release/json-viewer
# Optionally, install it to your system
cargo install --path .
```

### Option 3: Build .deb Package

```bash
# Make the build script executable
chmod +x build.sh

# Run the build script (detects architecture automatically)
./build.sh

# Install the generated package
sudo dpkg -i target/json-viewer_0.1.0_*.deb
```

## Usage

### Basic Usage

```bash
# Open current directory
json-viewer

# Open specific directory
json-viewer /path/to/json/files
```

### Keyboard Controls

#### Normal Mode
- `Tab` / `Shift+Tab`: Cycle focus forward/backward between Query input, File list, and JSON display
- `↑/↓`: Navigate through files (when file list is focused) OR move cursor through JSON content (when JSON display is focused)
- `Enter`: Select and open a file, or navigate to parent directory when ".." is selected (when file list is focused)
- `←` or `Backspace`: Go to parent directory (when file list is focused)
- `/`: Enter query mode
- `v`: Toggle between plain text and hierarchical view
- `w`: Toggle line wrapping in JSON display
- `Space`: Collapse/expand the node at cursor in hierarchical view (works at any level)
- `q` or `Ctrl+C`: Quit the application

**File Browser Features:**
- The top entry "📁 .." allows quick navigation to the parent directory
- Files are automatically refreshed when new files are added or removed from the current directory
- 📄 icons indicate individual files

#### Query Mode
- Type your query
- `Enter`: Execute the query
- `Esc`: Cancel and return to normal mode

## Query Syntax (dot/bracket)

Use simple dot/bracket notation to navigate JSON. Examples:

```bash
# Select a specific field
name

# Select nested field
user.address.city

# Select array element
users[0]

# Filter array
users[].name
users[1].address.city
```

Notes:
- `users[0]` selects the first element of the `users` array
- `users[].name` selects the `name` field for all users and returns an array
- Unsupported (for now): filters, projections, arithmetic, boolean expressions

## Architecture Support

This tool is built to support both:
- **x86_64** (AMD64): Standard Intel/AMD 64-bit processors
- **ARM64** (aarch64): ARM 64-bit processors (e.g., Apple Silicon, AWS Graviton)

## Cross-Compilation

To build for both architectures:

```bash
# Install cross-compilation targets
rustup target add x86_64-unknown-linux-gnu
rustup target add aarch64-unknown-linux-gnu

# Run the cross-compilation script
chmod +x build-cross.sh
./build-cross.sh
```

## Development

### Project Structure

```
json-viewer/
├── src/
│   ├── main.rs           # Entry point and CLI handling
│   ├── app.rs            # Application state management
│   ├── file_browser.rs   # File browsing logic
│   ├── json_viewer.rs    # JSON parsing and viewing
│   └── ui.rs             # Terminal UI rendering
├── debian/               # Debian package configuration
│   ├── control
│   ├── changelog
│   └── rules
├── Cargo.toml           # Rust dependencies
├── build.sh             # Build script for .deb packages
└── README.md
```

### Dependencies

- **ratatui**: Terminal UI framework
- **crossterm**: Cross-platform terminal manipulation
- **serde/serde_json**: JSON serialization and parsing
 
- **clap**: Command-line argument parsing
- **walkdir**: Directory traversal

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Troubleshooting

### Binary not found after installation

Make sure `/usr/bin` is in your `PATH`:
```bash
echo $PATH
```

### Permission denied when running build.sh

Make the script executable:
```bash
chmod +x build.sh
```

### Cross-compilation errors

Install the required target:
```bash
rustup target add <target-triple>
```

For cross-compilation, you may also need the appropriate linker:
```bash
# For ARM64 on x86_64 systems
sudo apt-get install gcc-aarch64-linux-gnu
```

## Acknowledgments

Built with:
- [Ratatui](https://github.com/ratatui-org/ratatui) - Terminal UI framework

---

Made with ❤️ and Rust
