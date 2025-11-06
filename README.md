# JSON Viewer

A fast, terminal-based JSON viewer with a TUI, now supporting both filesystem browsing and live MQTT streams.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)

## Features

- 📁 **Filesystem Browser**: Navigate JSON files in a directory using arrow keys
   - 🔼 Parent navigation via a ".." entry
   - 🔄 Auto-refresh when files are added/removed (watches .json and .txt)
- � **MQTT Mode (New)**: Subscribe to a topic and treat each incoming message as a "virtual file"
   - Live updates while connected; configurable max in-memory messages
   - Wildcard topics supported (e.g., sensors/# or +/temperature)
   - Optional username/password and custom client ID
   - Status bar shows connection state, retries, and last error time
- 📄 **Dual View Modes**:
   - Plain text view for raw JSON
   - Hierarchical tree view with collapse/expand via Space
- 🔍 **Query Support**: Simple dot/bracket queries to navigate and extract JSON data
- 📜 **Scrolling & Cursor**: Highlighted cursor line; auto-scroll
- 🔄 **Line Wrapping**: Toggle wrapping on/off for readability
- ⚡ **Fast & Lightweight**: Native Rust performance
- 🖥️ **Cross-Platform**: x86_64 and ARM64
- 📦 **Easy Installation**: Install via `.deb` or build from source

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
   # Filesystem mode (browse a directory)
   cargo run -- examples/

   # MQTT mode (subscribe and view messages)
   cargo run -- --mqtt mqtt://test.mosquitto.org:1883 --mqtt-topic sensors/#
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
git clone https://github.com/rlfmendes/json-viewer.git
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

### Filesystem mode

```bash
# Open current directory
json-viewer

# Open specific directory
json-viewer /path/to/json/files
```

### MQTT mode

```bash
# Minimum: broker + topic
json-viewer --mqtt mqtt://localhost:1883 --mqtt-topic sensors/#

# With auth and custom client ID
json-viewer \
   --mqtt mqtt://broker.example.com:1883 \
   --mqtt-topic devices/+/telemetry \
   --mqtt-user myuser \
   --mqtt-pass mypass \
   --mqtt-client-id json-viewer-dev \
   --max-messages 2000
```

Notes:
- In MQTT mode, each received message appears in the left pane as a row. Press Enter to open it in the viewer.
- Directory navigation (Enter/Backspace on folders) is disabled in MQTT mode.

### Keyboard Controls

#### Normal Mode
- `Tab` / `Shift+Tab`: Cycle focus forward/backward between Query input, File list, and JSON display
- `↑/↓`: Navigate through files (when file list is focused) OR move cursor through JSON content (when JSON display is focused)
- `Enter`: Open selected file; in filesystem mode, Enter on a directory navigates into it; in MQTT mode, it opens the selected message
- `←` or `Backspace`: Go to parent directory (filesystem mode only)
- `/`: Enter query mode
- `v`: Toggle between plain text and hierarchical view
- `w`: Toggle line wrapping in JSON display
- `Space`: Collapse/expand the node at cursor in hierarchical view (works at any level)
- `q` or `Ctrl+C`: Quit the application

**File Browser Features:**
- The top entry "📁 .." allows quick navigation to the parent directory
- Files are automatically refreshed when new files are added or removed from the current directory
- In MQTT mode, the list auto-updates as messages arrive; navigation to directories is disabled
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
│   ├── main.rs              # Entry point and CLI handling
│   ├── app.rs               # Application state management
│   ├── data_source.rs       # DataSource trait abstraction
│   ├── filesystem_source.rs # Filesystem implementation of DataSource
│   ├── mqtt_source.rs       # MQTT implementation of DataSource
│   ├── file_browser.rs      # Legacy helper (kept for compatibility)
│   ├── json_viewer.rs       # JSON parsing and viewing
│   └── ui.rs                # Terminal UI rendering
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
- **notify**: Filesystem watching for auto-refresh
- **rumqttc**: MQTT client
- **uuid**: Unique IDs for messages
- **chrono**: Timestamps and formatting

### CLI Reference

```
json-viewer [PATH]

Options:
   --mqtt <url>             MQTT broker URL (e.g., mqtt://localhost:1883)
   --mqtt-topic <topic>     Topic to subscribe to (supports + and #)
   --mqtt-user <user>       Username for MQTT auth
   --mqtt-pass <pass>       Password for MQTT auth
   --mqtt-client-id <id>    Custom client ID (auto-generated if omitted)
   --max-messages <n>       Max messages kept in memory (default: 1000)
```

When --mqtt is provided, PATH is ignored and the app runs in MQTT mode.

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

### MQTT: connection or no messages

- Ensure the broker URL is reachable (host/port) and correct (mqtt://host:port)
- Verify the topic matches published messages; try a wildcard like `#` to test
- If using auth, confirm username/password and permissions
- The status bar shows: `Connected | Broker: ... | Topic: ... | Messages: N | Retries: X | Last error: HH:MM:SS`
- Non-UTF8 payloads are ignored; non-JSON payloads will open as plain text but hierarchical view may show "No valid JSON file selected"

## Acknowledgments

Built with:
- [Ratatui](https://github.com/ratatui-org/ratatui) - Terminal UI framework

---

Made with ❤️ and Rust
