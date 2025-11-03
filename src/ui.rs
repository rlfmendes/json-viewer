use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, FocusedArea, InputMode, ViewMode};
use crate::data_source::DataSource;

pub fn draw(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Query box
            Constraint::Min(0),    // Main content
            Constraint::Length(3), // Status bar
        ])
        .split(f.area());

    // Draw query box
    draw_query_box(f, app, chunks[0]);

    // Split main content into left (file browser) and right (JSON viewer)
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30), // File browser
            Constraint::Percentage(70), // JSON viewer
        ])
        .split(chunks[1]);

    // Draw file browser
    draw_file_browser(f, app, main_chunks[0]);

    // Draw JSON viewer
    draw_json_viewer(f, app, main_chunks[1]);

    // Draw status bar
    draw_status_bar(f, app, chunks[2]);
}

fn draw_query_box(f: &mut Frame, app: &App, area: Rect) {
    let is_focused = matches!(app.focused_area, FocusedArea::Query);

    let style = match app.input_mode {
        InputMode::Query => Style::default().fg(Color::Yellow),
        InputMode::Normal => {
            if is_focused {
                Style::default().fg(Color::Cyan)
            } else {
                Style::default().fg(Color::Gray)
            }
        }
    };

    let text = if matches!(app.input_mode, InputMode::Query) {
        format!("Query: {}_", app.query_input)
    } else {
        format!("Query: {} (Press '/' to edit)", app.query_input)
    };

    let title = if is_focused && !matches!(app.input_mode, InputMode::Query) {
        "JQL Query [FOCUSED]"
    } else {
        "JQL Query"
    };

    let paragraph = Paragraph::new(text).block(
        Block::default()
            .borders(Borders::ALL)
            .title(title)
            .style(style),
    );

    f.render_widget(paragraph, area);
}

fn draw_file_browser(f: &mut Frame, app: &App, area: Rect) {
    let is_focused = matches!(app.focused_area, FocusedArea::FileList);

    // Get entries from data source
    let entries = app.data_source.get_entries();
    let selected_idx = app.data_source.selected_index();
    
    let items: Vec<ListItem> = entries
        .iter()
        .enumerate()
        .map(|(idx, entry)| {
            let style = if idx == selected_idx {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else if entry.is_navigable {
                Style::default().fg(Color::Cyan)
            } else {
                Style::default()
            };
            
            let display = if let Some(metadata) = &entry.metadata {
                format!("{} ({})", entry.display_name, metadata)
            } else {
                entry.display_name.clone()
            };
            
            ListItem::new(display).style(style)
        })
        .collect();

    // Calculate visible portion based on scroll offset
    let visible_height = area.height.saturating_sub(2) as usize; // Subtract borders
    let scroll_offset = app.data_source.scroll_offset();
    let visible_items: Vec<ListItem> = items
        .into_iter()
        .skip(scroll_offset)
        .take(visible_height)
        .collect();

    let nav_hint = if app.data_source.supports_navigation() {
        "(↑↓: navigate, Enter: open/parent)"
    } else {
        "(↑↓: navigate, Enter: select)"
    };

    let title = if is_focused {
        format!("Files [FOCUSED] {}", nav_hint)
    } else {
        "Files (Tab to focus)".to_string()
    };

    let border_style = if is_focused {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default()
    };

    let list = List::new(visible_items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(border_style),
        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    f.render_widget(list, area);
}

fn draw_json_viewer(f: &mut Frame, app: &mut App, area: Rect) {
    let is_focused = matches!(app.focused_area, FocusedArea::JsonDisplay);

    let view_mode_text = match app.view_mode {
        ViewMode::PlainText => "Plain Text",
        ViewMode::Hierarchical => "Hierarchical",
    };

    let wrap_text = if app.wrap_lines { "ON" } else { "OFF" };

    let title = if is_focused {
        format!("JSON View [FOCUSED] - {view_mode_text} | Wrap: {wrap_text} (↑↓: navigate, v: view, w: wrap, Space: collapse)")
    } else {
        format!("JSON View - {view_mode_text} | Wrap: {wrap_text} (Tab to focus)")
    };

    let border_style = if is_focused {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default()
    };

    let content = if let Some(result) = &app.query_result {
        result.clone()
    } else {
        match app.view_mode {
            ViewMode::PlainText => app
                .json_viewer
                .get_plain_text()
                .unwrap_or("No file selected")
                .to_string(),
            ViewMode::Hierarchical => {
                // Use collapse-aware rendering for hierarchical view
                if let Some((content, paths)) = app
                    .json_viewer
                    .get_hierarchical_view_with_collapse(&app.collapsed_paths)
                {
                    // Update line_to_path mapping
                    app.line_to_path = paths;
                    content
                } else {
                    "No valid JSON file selected".to_string()
                }
            }
        }
    };

    // Render with cursor highlight when focused
    let lines: Vec<&str> = content.lines().collect();
    let total_lines = lines.len();

    if is_focused && total_lines > 0 {
        // Render line by line with cursor highlight
        let visible_lines: Vec<Line> = lines
            .iter()
            .enumerate()
            .skip(app.scroll_offset)
            .map(|(idx, line_text)| {
                if idx == app.cursor_line {
                    // Highlight cursor line
                    Line::from(Span::styled(
                        *line_text,
                        Style::default()
                            .bg(Color::DarkGray)
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    ))
                } else {
                    Line::from(*line_text)
                }
            })
            .collect();

        let paragraph = Paragraph::new(visible_lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(border_style),
        );

        f.render_widget(paragraph, area);
    } else {
        // Fallback to simple rendering when not focused
        let visible_content = if app.scroll_offset < total_lines {
            lines[app.scroll_offset..].join("\n")
        } else {
            content
        };

        let wrap_mode = if app.wrap_lines {
            Wrap { trim: false }
        } else {
            Wrap { trim: true }
        };

        let paragraph = Paragraph::new(visible_content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(title)
                    .border_style(border_style),
            )
            .wrap(wrap_mode);

        f.render_widget(paragraph, area);
    }
}

fn draw_status_bar(f: &mut Frame, app: &App, area: Rect) {
    use crate::app::DataSourceMode;
    use crate::mqtt_source::MqttSource;

    let location_info = match app.source_mode {
        DataSourceMode::Filesystem => {
            let current_file = app
                .json_viewer
                .current_file
                .as_ref()
                .and_then(|p| p.to_str())
                .unwrap_or("No file selected");
            format!("File: {}", current_file)
        }
        DataSourceMode::Mqtt => {
            // Try to get MQTT-specific status
            let base_location = app.data_source.get_location();
            let status = if let Some(mqtt_source) = unsafe {
                // Safe because we know it's MqttSource in Mqtt mode
                (app.data_source.as_ref() as *const dyn DataSource as *const MqttSource).as_ref()
            } {
                mqtt_source.connection_status.clone()
            } else {
                base_location
            };
            status
        }
    };

    let help_text = match app.input_mode {
        InputMode::Normal => {
            if app.data_source.supports_navigation() {
                "q: quit | Tab/Shift+Tab: focus | /: query | v: view | w: wrap | Space: collapse | ←/Backspace: parent"
            } else {
                "q: quit | Tab/Shift+Tab: focus | /: query | v: view | w: wrap | Space: collapse"
            }
        }
        InputMode::Query => "Enter: execute query | Esc: cancel",
    };

    let status = vec![
        Line::from(vec![
            Span::styled("Status: ", Style::default().fg(Color::Cyan)),
            Span::raw(location_info),
        ]),
        Line::from(help_text),
    ];

    let paragraph =
        Paragraph::new(status).block(Block::default().borders(Borders::ALL).title("Status"));

    f.render_widget(paragraph, area);
}

