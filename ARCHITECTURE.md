# JSON Viewer - Architecture Overview

## Application Flow

```
┌─────────────────────────────────────────────────────────────────┐
│                         User Terminal                            │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │  Query Box (Top Panel)                                     │  │
│  │  [JQL Query: .[].name_________________________]            │  │
│  └───────────────────────────────────────────────────────────┘  │
│  ┌──────────────────┬────────────────────────────────────────┐  │
│  │ File Browser     │ JSON Viewer                            │  │
│  │ (Left Panel)     │ (Right Panel)                          │  │
│  │                  │                                        │  │
│  │ > sample.json    │ {                                      │  │
│  │   nested.json    │   "name": "John Doe",                 │  │
│  │   config.json    │   "age": 30,                          │  │
│  │                  │   "address": {                         │  │
│  │                  │     "city": "New York"                 │  │
│  │                  │   }                                    │  │
│  │                  │ }                                      │  │
│  └──────────────────┴────────────────────────────────────────┘  │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │  Status Bar (Bottom Panel)                                 │  │
│  │  File: /path/to/sample.json | ↑↓: navigate | /: query    │  │
│  └───────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

## Component Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                           main.rs                                │
│  • CLI argument parsing (clap)                                   │
│  • Terminal setup (crossterm)                                    │
│  • Event loop (keyboard input)                                   │
│  • Application lifecycle                                         │
└──────────────────┬──────────────────────────────────────────────┘
                   │
                   ├─────────────────────┐
                   │                     │
         ┌─────────▼─────────┐  ┌────────▼────────┐
         │     app.rs        │  │     ui.rs       │
         │                   │  │                 │
         │ • App state       │◄─┤ • Rendering     │
         │ • Mode management │  │ • Layout        │
         │ • Input handling  │  │ • Styling       │
         │ • Coordination    │  │ • Components    │
         └─────┬────────┬────┘  └─────────────────┘
               │        │
      ┌────────▼──┐  ┌──▼─────────────┐
      │file_browser│  │json_viewer.rs  │
      │.rs         │  │                │
      │            │  │ • Load files   │
      │ • Scan dir │  │ • Parse JSON   │
      │ • Navigate │  │ • Format views │
      │ • Select   │  │ • Run queries  │
      └────────────┘  └────────────────┘
```

## Data Flow

```
User Input (Keyboard)
       │
       ▼
┌──────────────┐
│ crossterm    │ Capture key events
│ event::read()│
└──────┬───────┘
       │
       ▼
┌──────────────────────┐
│ main.rs event loop   │ Match key codes
│ match key.code       │
└──────┬───────────────┘
       │
       ├─── Up/Down ──────────┐
       ├─── Enter ────────────┤
       ├─── Tab ──────────────┤
       ├─── / ────────────────┤
       └─── q ────────────────┤
                               ▼
                    ┌─────────────────┐
                    │ App state update│
                    └────────┬────────┘
                             │
                    ┌────────▼────────┐
                    │  ui::draw()     │
                    │  Render changes │
                    └────────┬────────┘
                             │
                             ▼
                    ┌────────────────┐
                    │ Terminal output│
                    └────────────────┘
```

## File Browser Flow

```
┌────────────────────┐
│ User opens folder  │
└─────────┬──────────┘
          │
          ▼
┌──────────────────────┐
│ FileBrowser::new()   │
│ • walkdir scan       │
│ • Filter .json files │
│ • Sort alphabetically│
└─────────┬────────────┘
          │
          ▼
┌──────────────────────┐
│ Display file list    │
│ with cursor          │
└─────────┬────────────┘
          │
    ┌─────┴─────┐
    │           │
    ▼           ▼
┌───────┐   ┌───────┐
│ Up/   │   │Enter  │
│ Down  │   │select │
└───┬───┘   └───┬───┘
    │           │
    ▼           ▼
┌───────┐   ┌────────────────┐
│Move   │   │Load file into  │
│cursor │   │json_viewer     │
└───────┘   └────────────────┘
```

## JSON Viewer Flow

```
┌─────────────────────┐
│ File selected       │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────────┐
│ JsonViewer::load_file() │
│ • Read file content     │
│ • Parse JSON (serde)    │
│ • Store raw + parsed    │
└──────────┬──────────────┘
           │
     ┌─────┴─────┐
     │           │
     ▼           ▼
┌─────────┐ ┌──────────────┐
│Plain    │ │Hierarchical  │
│text view│ │tree view     │
└─────────┘ └──────────────┘
           │
           │ (Query mode)
           ▼
┌─────────────────────┐
│ JsonViewer::query() │
│ • Use JQL library   │
│ • Filter/transform  │
│ • Display result    │
└─────────────────────┘
```

## Query Processing

```
User presses '/'
       │
       ▼
┌──────────────────┐
│ Enter query mode │
│ input_mode =     │
│ InputMode::Query │
└────────┬─────────┘
         │
         ▼
┌─────────────────┐
│ User types      │
│ query string    │
└────────┬────────┘
         │
         ▼ (Enter pressed)
┌──────────────────────┐
│ app.execute_query()  │
└──────────┬───────────┘
           │
           ▼
┌───────────────────────┐
│ json_viewer.query()   │
│ • Parse JQL syntax    │
│ • Apply to JSON data  │
│ • Return result       │
└──────────┬────────────┘
           │
           ▼
┌───────────────────────┐
│ Display query result  │
│ in JSON viewer panel  │
└───────────────────────┘
```

## Build Process

```
┌──────────────┐
│ cargo build  │
└──────┬───────┘
       │
       ▼
┌────────────────────┐
│ Fetch dependencies │
│ • ratatui          │
│ • crossterm        │
│ • serde_json       │
│ • jql              │
│ • clap             │
└────────┬───────────┘
         │
         ▼
┌────────────────────┐
│ Compile modules    │
│ in dependency order│
└────────┬───────────┘
         │
         ▼
┌────────────────────┐
│ Link binary        │
│ json-viewer        │
└────────┬───────────┘
         │
         ▼
┌────────────────────┐
│ Output to target/  │
│ debug/ or release/ │
└────────────────────┘
```

## Package Creation

```
┌──────────────┐
│ ./build.sh   │
└──────┬───────┘
       │
       ▼
┌──────────────────┐
│ Detect arch      │
│ x86_64 or arm64  │
└──────┬───────────┘
       │
       ▼
┌──────────────────┐
│ cargo build      │
│ --release        │
│ --target ARCH    │
└──────┬───────────┘
       │
       ▼
┌──────────────────┐
│ Create pkg dir   │
│ target/debian-   │
│ package/         │
└──────┬───────────┘
       │
       ▼
┌──────────────────┐
│ Copy files       │
│ • Binary         │
│ • Control        │
│ • Metadata       │
└──────┬───────────┘
       │
       ▼
┌──────────────────┐
│ dpkg-deb --build │
└──────┬───────────┘
       │
       ▼
┌──────────────────────────┐
│ json-viewer_0.1.0_*.deb  │
└──────────────────────────┘
```

## DevContainer Setup

```
┌──────────────────────┐
│ Open in VS Code      │
└──────────┬───────────┘
           │
           ▼
┌──────────────────────┐
│ Read devcontainer.   │
│ json configuration   │
└──────────┬───────────┘
           │
           ▼
┌──────────────────────┐
│ Pull Docker image    │
│ rust:1-1-bookworm    │
└──────────┬───────────┘
           │
           ▼
┌──────────────────────┐
│ Install features     │
│ • common-utils       │
│ • git                │
└──────────┬───────────┘
           │
           ▼
┌──────────────────────┐
│ Install VS Code      │
│ extensions           │
│ • rust-analyzer      │
│ • TOML support       │
│ • debugger           │
└──────────┬───────────┘
           │
           ▼
┌──────────────────────┐
│ Run post-create.sh   │
│ • Add targets        │
│ • Fetch deps         │
└──────────┬───────────┘
           │
           ▼
┌──────────────────────┐
│ Ready to develop! 🚀 │
└──────────────────────┘
```

## Key Technologies

```
┌─────────────────────────────────────────┐
│             Application Layer            │
│  ┌──────────┐  ┌──────────┐  ┌────────┐│
│  │  Ratatui │  │Crossterm │  │  JQL   ││
│  │   (TUI)  │  │(Terminal)│  │(Query) ││
│  └──────────┘  └──────────┘  └────────┘│
└─────────────────────────────────────────┘
┌─────────────────────────────────────────┐
│            Data Layer                    │
│  ┌──────────┐  ┌──────────┐  ┌────────┐│
│  │  Serde   │  │ Walkdir  │  │  Clap  ││
│  │  (JSON)  │  │  (Files) │  │  (CLI) ││
│  └──────────┘  └──────────┘  └────────┘│
└─────────────────────────────────────────┘
┌─────────────────────────────────────────┐
│            System Layer                  │
│  ┌──────────┐  ┌──────────┐  ┌────────┐│
│  │   Rust   │  │  Cargo   │  │  LLVM  ││
│  │ Compiler │  │  Build   │  │Codegen ││
│  └──────────┘  └──────────┘  └────────┘│
└─────────────────────────────────────────┘
```

---

This architecture provides:
- 🎯 **Separation of concerns** - Each module has a single responsibility
- 🔄 **Reactive UI** - State changes trigger UI updates
- 🚀 **Performance** - Rust's zero-cost abstractions
- 🛡️ **Safety** - Type-safe, memory-safe code
- 🧩 **Modularity** - Easy to extend and maintain
