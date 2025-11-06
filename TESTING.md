# Testing the JSON Viewer

## Getting Started with DevContainer

### Step 1: Open in DevContainer

1. Open VS Code in the project directory
2. Click "Reopen in Container" notification, or
3. Press `Cmd/Ctrl+Shift+P` → "Dev Containers: Reopen in Container"

### Step 2: Wait for Setup

The container will automatically:
- Install Rust toolchain
- Add clippy and rustfmt
- Install cross-compilation targets
- Fetch all dependencies

This takes 2-5 minutes on first run.

### Step 3: Test the Application

#### Quick Test with Examples

```bash
# Run with provided example files
make run

# Or directly with cargo
cargo run -- examples/
```

#### Quick Test with MQTT (Live Messages)

```bash
# Run in MQTT mode (public broker example)
cargo run -- --mqtt mqtt://test.mosquitto.org:1883 --mqtt-topic sensors/#

# Or your own broker and topic
cargo run -- --mqtt mqtt://localhost:1883 --mqtt-topic devices/+/telemetry
```

What to expect in MQTT mode:
- Left pane lists incoming messages as rows (newest first)
- Press Enter on a row to open the message in the JSON viewer
- Status bar shows connection state, message count, retries, and last error time

#### What to Try

1. **Navigate Files**:
   - Use `↑` and `↓` arrow keys to move between files
   - You should see `sample-data.json` and `nested-data.json`

2. **View JSON**:
   - Press `Enter` to select a file
   - Right panel shows the JSON content
  - Press `v` to toggle between plain text and hierarchical view
  - Press `w` to toggle line wrapping
  - Press `Space` to collapse/expand the current node (hierarchical view)

3. **Run Queries**:
   - Press `/` to enter query mode
  - Type: `[0].name` and press `Enter`
   - Should show: "John Doe" from the first entry
  - Note: Queries work the same on opened MQTT messages

4. **Exit**:
   - Press `q` or `Ctrl+C` to quit

5. **Focus**:
  - Press `Tab` / `Shift+Tab` to cycle focus between Query, Files, and JSON panes

## Testing with Your Own Data

### Create Test Directory

```bash
# Create a test directory with JSON files
mkdir -p test-data
cd test-data

# Create some test JSON files
cat > users.json << 'EOF'
{
  "users": [
    {"id": 1, "name": "Alice", "active": true},
    {"id": 2, "name": "Bob", "active": false}
  ]
}
EOF

cat > config.json << 'EOF'
{
  "app": {
    "name": "MyApp",
    "version": "1.0.0",
    "settings": {
      "debug": true,
      "port": 8080
    }
  }
}
EOF

cd ..

# Run the viewer on your test data
cargo run -- test-data/
```

## Query Examples to Test

### On sample-data.json

```bash
# Get first person's name
[0].name
→ "John Doe"

# Get all names
[].name
→ ["John Doe", "Jane Smith", "Bob Johnson"]

# Get all cities
[].address.city
→ ["New York", "Los Angeles", "Chicago"]
 
# Get second person
[1]
→ {"id": 2, "name": "Jane Smith", ...}
```

### On nested-data.json

```bash
# Get company name
company
→ "TechCorp"

# Get first employee
employees[0]
→ {name: "Alice", role: "Developer", ...}

# Get all employee names
employees[].name
→ ["Alice", "Bob"]

# Get active projects
projects.active
→ [list of active projects]

# Get first active project name
projects.active[0].name
→ "Project Alpha"
```

## Build Tests

### Test Debug Build

```bash
make build
# or
cargo build

# Should complete without errors
# Binary at: target/debug/json-viewer
```

### Test Release Build

```bash
make release
# or
cargo build --release

# Should complete without errors
# Optimized binary at: target/release/json-viewer
```

### Test Package Creation

```bash
make package
# or
chmod +x build.sh && ./build.sh

# Should create: target/json-viewer_0.1.0_amd64.deb
```

## Code Quality Tests

### Run Linter

```bash
make clippy
# or
cargo clippy -- -D warnings

# Should pass without warnings
```

### Check Formatting

```bash
make fmt
# or
cargo fmt --check
```

## Manual Testing Checklist

- [ ] File browser shows all .json files in directory
- [ ] Arrow keys navigate up/down through file list
- [ ] Enter key opens selected file
- [ ] Plain text view shows raw JSON
- [ ] Hierarchical view shows formatted JSON tree
- [ ] Tab/Shift+Tab cycles focus across panes
- [ ] 'v' toggles between views (plain/hierarchical)
- [ ] 'w' toggles line wrapping
- [ ] 'Space' collapses/expands nodes in hierarchical view
- [ ] `/` key activates query input
- [ ] Query executes on Enter
- [ ] Esc exits query mode
- [ ] `q` exits application
- [ ] Ctrl+C exits application
- [ ] Works with nested JSON structures
- [ ] Works with array JSON structures
- [ ] Handles invalid JSON gracefully
- [ ] File list scrolls when many entries (selection stays visible)

### MQTT Mode Checklist

- [ ] App starts in MQTT mode with provided broker and topic
- [ ] Status bar shows connection state, broker, topic, Messages: N
- [ ] Messages appear in the left pane as new publishes arrive
- [ ] Enter opens the selected message in the viewer
- [ ] Directory navigation is disabled (no parent/back navigation)
- [ ] Non-JSON payloads open as plain text; hierarchical view indicates invalid JSON
- [ ] Non-UTF8 payloads are ignored (do not crash)
- [ ] Status bar increments retries and shows last error when connection issues occur

## Performance Testing

### Large File Test

```bash
# Create a large JSON file
python3 << 'EOF'
import json

data = [
    {
        "id": i,
        "name": f"User {i}",
        "email": f"user{i}@example.com",
        "data": {"value": i * 2, "nested": {"deep": i * 3}}
    }
    for i in range(1000)
]

with open("test-data/large.json", "w") as f:
    json.dump(data, f)
EOF

# Test with large file
cargo run -- test-data/
```

The viewer should:
- Load quickly
- Navigate smoothly
- Display content without lag

## Troubleshooting Tests

### Terminal Not Interactive

If the TUI doesn't work:
```bash
# Make sure you're in an interactive terminal
# In VS Code, use the integrated terminal (Ctrl+`)
# Not the debug console
```

### No Files Shown

```bash
# Make sure there are .json files in the directory
ls -la examples/
# Should show sample-data.json and nested-data.json
```

### Query Not Working

```bash
# Make sure you're pressing '/' first
# Then type query without quotes in the app
# The query box should turn yellow when active
```

### MQTT: No Messages

```bash
# Try a wildcard topic to broaden matches
cargo run -- --mqtt mqtt://localhost:1883 --mqtt-topic '#'

# If auth is required
cargo run -- --mqtt mqtt://broker:1883 --mqtt-topic sensors/# --mqtt-user USER --mqtt-pass PASS
```

Tips:
- Verify the broker host/port are correct and reachable
- Publish a test message to your topic and confirm it appears
- Status bar shows retries and last error; use this to diagnose

#### Quick Ways to Publish Test Messages

Using mosquitto_pub (if available):
```bash
mosquitto_pub -h test.mosquitto.org -p 1883 -t sensors/temp -m '{"temp":22, "unit":"C"}'
```

Using Python (paho-mqtt):
```bash
python3 - << 'PY'
import json, time
import paho.mqtt.client as mqtt
c = mqtt.Client()
c.connect('test.mosquitto.org', 1883, 60)
for i in range(3):
  payload = json.dumps({"seq": i, "ok": True})
  c.publish('sensors/demo', payload)
  time.sleep(0.5)
c.disconnect()
PY
```

## Automated Testing (Future)

To add unit tests, create test modules in the source files:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_file_browser() {
        // Add test code
    }
}
```

Run tests with:
```bash
cargo test
```

## Success Criteria

✅ Application starts without errors
✅ Can navigate files with arrow keys
✅ Can view JSON in both modes
✅ Can execute queries successfully
✅ Can exit cleanly
✅ Builds successfully for release
✅ Creates .deb package successfully

## Need Help?

- Check logs: `RUST_BACKTRACE=1 cargo run -- examples/`
- Review error messages in the terminal
- See QUICKSTART.md for basic usage (filesystem and MQTT)
- See README.md for full documentation and CLI flags
