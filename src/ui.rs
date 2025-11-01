use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, FocusedArea, InputMode, ViewMode};

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Query box
            Constraint::Min(0),     // Main content
            Constraint::Length(3),  // Status bar
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

    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title(title).style(style));

    f.render_widget(paragraph, area);
}

fn draw_file_browser(f: &mut Frame, app: &App, area: Rect) {
    let is_focused = matches!(app.focused_area, FocusedArea::FileList);
    
    let items: Vec<ListItem> = app
        .file_browser
        .files
        .iter()
        .enumerate()
        .map(|(idx, path)| {
            let name = app.file_browser.get_display_name(path);
            let style = if idx == app.file_browser.selected_index {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            ListItem::new(name).style(style)
        })
        .collect();

    let title = if is_focused {
        "Files [FOCUSED] (↑↓ to navigate, Enter to select)"
    } else {
        "Files (Tab to focus)"
    };
    
    let border_style = if is_focused {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default()
    };

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(border_style)
        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    f.render_widget(list, area);
}

fn draw_json_viewer(f: &mut Frame, app: &App, area: Rect) {
    let is_focused = matches!(app.focused_area, FocusedArea::JsonDisplay);
    
    let view_mode_text = match app.view_mode {
        ViewMode::PlainText => "Plain Text",
        ViewMode::Hierarchical => "Hierarchical",
    };
    
    let wrap_text = if app.wrap_lines { "ON" } else { "OFF" };
    
    let title = if is_focused {
        format!("JSON View [FOCUSED] - {} | Wrap: {} (↑↓: scroll, v: view, w: wrap)", view_mode_text, wrap_text)
    } else {
        format!("JSON View - {} | Wrap: {} (Tab to focus)", view_mode_text, wrap_text)
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
            ViewMode::PlainText => {
                app.json_viewer.get_plain_text()
                    .unwrap_or("No file selected")
                    .to_string()
            }
            ViewMode::Hierarchical => {
                app.json_viewer.get_hierarchical_view()
                    .unwrap_or_else(|| "No valid JSON file selected".to_string())
            }
        }
    };

    // Apply scrolling by skipping lines
    let lines: Vec<&str> = content.lines().collect();
    let total_lines = lines.len();
    let visible_content = if app.scroll_offset < total_lines {
        lines.iter()
            .skip(app.scroll_offset)
            .map(|s| *s)
            .collect::<Vec<&str>>()
            .join("\n")
    } else {
        content
    };

    let wrap_mode = if app.wrap_lines {
        Wrap { trim: false }
    } else {
        Wrap { trim: true }
    };

    let paragraph = Paragraph::new(visible_content)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(title)
            .border_style(border_style))
        .wrap(wrap_mode)
        .scroll((0, 0));

    f.render_widget(paragraph, area);
}

fn draw_status_bar(f: &mut Frame, app: &App, area: Rect) {
    let current_file = app
        .json_viewer
        .current_file
        .as_ref()
        .and_then(|p| p.to_str())
        .unwrap_or("No file selected");

    let help_text = match app.input_mode {
        InputMode::Normal => "q: quit | Tab: cycle focus | /: query | v: view | w: wrap",
        InputMode::Query => "Enter: execute query | Esc: cancel",
    };

    let status = vec![
        Line::from(vec![
            Span::styled("File: ", Style::default().fg(Color::Cyan)),
            Span::raw(current_file),
        ]),
        Line::from(help_text),
    ];

    let paragraph = Paragraph::new(status)
        .block(Block::default().borders(Borders::ALL).title("Status"));

    f.render_widget(paragraph, area);
}
