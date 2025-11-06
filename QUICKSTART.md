# Quick Start Guide

## Using the DevContainer

### First Time Setup

1. **Install Prerequisites**:
   - Docker Desktop: https://www.docker.com/products/docker-desktop
   - VS Code: https://code.visualstudio.com/
   - Dev Containers Extension: Install from VS Code marketplace

2. **Open Project**:
   ```bash
   cd /path/to/json-viewer
   code .
   ```

3. **Start DevContainer**:
   - Click "Reopen in Container" when prompted
   - Or press `Cmd+Shift+P` (macOS) / `Ctrl+Shift+P` (Linux/Windows)
   - Type "Dev Containers: Reopen in Container"

4. **Wait for Setup** (first time only):
   - Container will download (~2-3 minutes)
   - Dependencies will be fetched
   - Cross-compilation targets will be installed

### Testing the Application

Once inside the container:

```bash
# Filesystem mode — browse a directory of JSON files
cargo run -- examples/

# Or specify a different directory
cargo run -- /path/to/your/json/files

# MQTT mode — subscribe to a topic and view messages as files
cargo run -- --mqtt mqtt://test.mosquitto.org:1883 --mqtt-topic sensors/#
```

### Keyboard Controls

- `↑/↓`: Navigate files/messages (left) or move cursor in JSON view (right)
- `Enter`: Open selected file/message; Enter on a directory navigates into it (filesystem mode)
- `←` or `Backspace`: Navigate to parent directory (filesystem mode)
- `/`: Enter query mode
- `v`: Toggle between plain text and hierarchical view
- `w`: Toggle line wrapping
- `Space`: Collapse/expand node at cursor (hierarchical view)
- `Esc`: Exit query mode
- `q` or `Ctrl+C`: Quit

### Example Queries

When in query mode (press `/`), try these dot/bracket queries:

```
# On examples/sample-data.json:
[0].name           # Get first person's name
[].age             # Get all ages
[].address.city    # Get all cities

# On examples/nested-data.json:
company            # Get company name
employees[0]       # Get first employee
projects.active    # Get active projects
```

### Building

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Check code quality
cargo clippy -- -D warnings
```

### Creating .deb Package

```bash
# Make build script executable
chmod +x build.sh

# Build and create package
./build.sh

# Package will be created at:
# target/json-viewer_0.1.0_amd64.deb (or arm64)
```

### Useful VS Code Commands

- `Ctrl+Shift+B`: Show build tasks
- `F5`: Start debugging
- `Ctrl+` ` : Open terminal

## Troubleshooting

### Container won't start
- Make sure Docker Desktop is running
- Try: "Dev Containers: Rebuild Container"

### Permission errors
```bash
# Inside container, make scripts executable:
chmod +x build.sh build-cross.sh
```

### Slow first build
- First build fetches all dependencies (normal)
- Subsequent builds will be much faster
- The `target/` directory is cached in a volume

### Terminal UI not working
- The TUI requires a proper terminal
- Use VS Code's integrated terminal (not the debug console)
- Make sure you're running in interactive mode

### MQTT: No messages appear

- Verify the broker is reachable and URL is correct: `mqtt://host:port`
- Try a broad topic first to validate: `--mqtt-topic #`
- If your broker requires auth, pass `--mqtt-user` and `--mqtt-pass`
- The status bar shows connection state, retry count, and last error time
- Non-UTF8 payloads are ignored; non-JSON opens as plain text

## Need Help?

Check the main README.md for detailed documentation, including the new MQTT mode, or:
- Review `.devcontainer/README.md` for container details
- Look at the example JSON files in `examples/`
 
