mod app;
mod data_source;
mod file_browser;
mod filesystem_source;
mod json_viewer;
mod mqtt_source;
mod ui;

use anyhow::Result;
use clap::Parser;
use crossterm::{
    event::{self, poll, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::path::PathBuf;

use app::App;

#[derive(Parser)]
#[command(name = "json-viewer")]
#[command(about = "A terminal-based JSON file viewer with query capabilities", long_about = None)]
struct Cli {
    /// Directory to browse (defaults to current directory)
    #[arg(default_value = ".", conflicts_with = "mqtt")]
    path: PathBuf,

    /// MQTT broker URL (e.g., mqtt://localhost:1883)
    #[arg(long)]
    mqtt: Option<String>,

    /// MQTT topic to subscribe to (supports wildcards like sensors/# or +/temperature)
    #[arg(long, requires = "mqtt")]
    mqtt_topic: Option<String>,

    /// MQTT username for authentication
    #[arg(long, requires = "mqtt")]
    mqtt_user: Option<String>,

    /// MQTT password for authentication
    #[arg(long, requires = "mqtt")]
    mqtt_pass: Option<String>,

    /// MQTT client ID (auto-generated if not provided)
    #[arg(long, requires = "mqtt")]
    mqtt_client_id: Option<String>,

    /// Maximum number of messages to keep in memory
    #[arg(long, default_value = "1000")]
    max_messages: usize,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app based on mode
    let mut app = if let Some(mqtt_url) = cli.mqtt {
        // MQTT mode
        let topic = cli.mqtt_topic.unwrap_or_else(|| "#".to_string());
        App::new_mqtt(
            mqtt_url,
            topic,
            cli.mqtt_user,
            cli.mqtt_pass,
            cli.mqtt_client_id,
            cli.max_messages,
        )?
    } else {
        // Filesystem mode
        App::new(cli.path)?
    };

    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {err}");
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    loop {
        // Check for file system changes
        app.check_file_changes();

        terminal.draw(|f| ui::draw(f, app))?;

        // Poll for events with a timeout to allow file watching updates
        if poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match app.input_mode {
                    app::InputMode::Normal => match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            return Ok(())
                        }
                        KeyCode::Up => match app.focused_area {
                            app::FocusedArea::FileList => {
                                app.data_source.previous();
                                // Calculate visible height for file browser (30% of screen width, minus borders)
                                let visible_height = terminal.size()?.height.saturating_sub(8) as usize;
                                app.data_source.ensure_visible(visible_height);
                            }
                            app::FocusedArea::JsonDisplay => {
                                app.cursor_up();
                            }
                            _ => {}
                        },
                        KeyCode::Down => {
                            match app.focused_area {
                                app::FocusedArea::FileList => {
                                    app.data_source.next();
                                    // Calculate visible height for file browser
                                    let visible_height = terminal.size()?.height.saturating_sub(8) as usize;
                                    app.data_source.ensure_visible(visible_height);
                                }
                                app::FocusedArea::JsonDisplay => {
                                    // Calculate total lines from current content
                                    let content = if let Some(result) = &app.query_result {
                                        result.clone()
                                    } else {
                                        match app.view_mode {
                                            app::ViewMode::PlainText => app
                                                .json_viewer
                                                .get_plain_text()
                                                .unwrap_or("No file selected")
                                                .to_string(),
                                            app::ViewMode::Hierarchical => app
                                                .json_viewer
                                                .get_hierarchical_view_with_depth(app.fold_depth)
                                                .unwrap_or_else(|| {
                                                    "No valid JSON file selected".to_string()
                                                }),
                                        }
                                    };
                                    let total_lines = content.lines().count();
                                    let visible_height =
                                        terminal.size()?.height.saturating_sub(9) as usize;
                                    app.cursor_down(total_lines, visible_height);
                                }
                                _ => {}
                            }
                        }
                        KeyCode::Enter => {
                            if let app::FocusedArea::FileList = app.focused_area {
                                if let Some(entry) = app.data_source.get_selected_entry() {
                                    if entry.id == ".." {
                                        // Navigate to parent
                                        app.navigate_parent_dir()?;
                                    } else if entry.is_navigable && app.data_source.supports_navigation() {
                                        // Navigate into directory
                                        app.navigate_into_directory()?;
                                    } else {
                                        // Open the selected file/message
                                        app.select_file()?;
                                    }
                                }
                            }
                        }
                        KeyCode::Left | KeyCode::Backspace => {
                            if let app::FocusedArea::FileList = app.focused_area {
                                if app.data_source.supports_navigation() && app.data_source.has_parent() {
                                    app.navigate_parent_dir()?;
                                }
                            }
                        }
                        KeyCode::Char('/') => app.enter_query_mode(),
                        KeyCode::Tab => app.cycle_focus(),
                        KeyCode::BackTab => app.cycle_focus_reverse(),
                        KeyCode::Char('v') => app.toggle_view(),
                        KeyCode::Char('w') => app.toggle_wrap(),
                        KeyCode::Char(' ') => {
                            if let app::FocusedArea::JsonDisplay = app.focused_area {
                                if let app::ViewMode::Hierarchical = app.view_mode {
                                    app.toggle_collapse();
                                }
                            }
                        }
                        _ => {}
                    },
                    app::InputMode::Query => match key.code {
                        KeyCode::Enter => {
                            app.execute_query()?;
                            app.exit_query_mode();
                        }
                        KeyCode::Esc => app.exit_query_mode(),
                        KeyCode::Char(c) => app.query_input.push(c),
                        KeyCode::Backspace => {
                            app.query_input.pop();
                        }
                        _ => {}
                    },
                }
            }
        }
    }
}
