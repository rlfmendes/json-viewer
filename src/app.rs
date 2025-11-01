use crate::file_browser::FileBrowser;
use crate::json_viewer::JsonViewer;
use anyhow::Result;
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver};

pub enum InputMode {
    Normal,
    Query,
}

pub enum ViewMode {
    PlainText,
    Hierarchical,
}

pub enum FocusedArea {
    Query,
    FileList,
    JsonDisplay,
}

pub struct App {
    pub file_browser: FileBrowser,
    pub json_viewer: JsonViewer,
    pub input_mode: InputMode,
    pub view_mode: ViewMode,
    pub focused_area: FocusedArea,
    pub query_input: String,
    pub query_result: Option<String>,
    pub root_path: PathBuf,
    pub scroll_offset: usize,
    pub wrap_lines: bool,
    pub fold_depth: Option<usize>,
    pub cursor_line: usize,
    pub collapsed_paths: HashSet<String>,
    pub line_to_path: Vec<String>, // Maps line number to JSON path
    pub watcher: Option<RecommendedWatcher>,
    pub watcher_rx: Option<Receiver<Result<Event, notify::Error>>>,
    pub files_changed: bool,
}

impl App {
    pub fn new(path: PathBuf) -> Result<Self> {
        // Canonicalize the path to absolute path immediately
        let canonical_path = path.canonicalize().unwrap_or_else(|_| path.clone());

        let file_browser = FileBrowser::new(&canonical_path)?;
        let json_viewer = JsonViewer::new();

        // Setup file watcher
        let (tx, rx) = channel();
        let mut watcher = notify::recommended_watcher(move |res| {
            let _ = tx.send(res);
        })?;

        // Watch the directory for changes
        watcher.watch(&canonical_path, RecursiveMode::Recursive)?;

        Ok(Self {
            file_browser,
            json_viewer,
            input_mode: InputMode::Normal,
            view_mode: ViewMode::Hierarchical,
            focused_area: FocusedArea::FileList,
            query_input: String::new(),
            query_result: None,
            root_path: canonical_path,
            scroll_offset: 0,
            wrap_lines: true,
            fold_depth: None,
            cursor_line: 0,
            collapsed_paths: HashSet::new(),
            line_to_path: Vec::new(),
            watcher: Some(watcher),
            watcher_rx: Some(rx),
            files_changed: false,
        })
    }

    pub fn check_file_changes(&mut self) {
        if let Some(rx) = &self.watcher_rx {
            // Check for file system events (non-blocking)
            while let Ok(result) = rx.try_recv() {
                if let Ok(event) = result {
                    // Check if the event involves .json or .txt files
                    let is_relevant = event.paths.iter().any(|p| {
                        if let Some(ext) = p.extension() {
                            ext == "json" || ext == "txt"
                        } else {
                            false
                        }
                    });

                    if is_relevant {
                        self.files_changed = true;
                    }
                }
            }
        }

        // Refresh file list if changes detected
        if self.files_changed {
            let _ = self.file_browser.refresh();
            self.files_changed = false;
        }
    }

    pub fn select_file(&mut self) -> Result<()> {
        if let Some(file_path) = self.file_browser.get_selected_file() {
            self.json_viewer.load_file(file_path)?;
            self.query_result = None;
            self.scroll_offset = 0; // Reset scroll when opening new file
            self.cursor_line = 0; // Reset cursor when opening new file
        }
        Ok(())
    }

    pub fn navigate_into_directory(&mut self) -> Result<()> {
        if let Some(dir_path) = self.file_browser.get_selected_directory() {
            self.root_path = dir_path.clone();
            self.file_browser = FileBrowser::new(&self.root_path)?;
            self.json_viewer.current_file = None;
            self.query_result = None;
            self.scroll_offset = 0;

            // Update watcher for new directory
            if let Some(watcher) = &mut self.watcher {
                let _ = watcher.watch(&self.root_path, notify::RecursiveMode::Recursive);
            }
        }
        Ok(())
    }

    pub fn enter_query_mode(&mut self) {
        self.input_mode = InputMode::Query;
        self.query_input.clear();
    }

    pub fn exit_query_mode(&mut self) {
        self.input_mode = InputMode::Normal;
    }

    pub fn toggle_view(&mut self) {
        self.view_mode = match self.view_mode {
            ViewMode::PlainText => ViewMode::Hierarchical,
            ViewMode::Hierarchical => ViewMode::PlainText,
        };
    }

    pub fn cycle_focus(&mut self) {
        self.focused_area = match self.focused_area {
            FocusedArea::Query => FocusedArea::FileList,
            FocusedArea::FileList => FocusedArea::JsonDisplay,
            FocusedArea::JsonDisplay => FocusedArea::Query,
        };
    }

    pub fn cycle_focus_reverse(&mut self) {
        self.focused_area = match self.focused_area {
            FocusedArea::Query => FocusedArea::JsonDisplay,
            FocusedArea::FileList => FocusedArea::Query,
            FocusedArea::JsonDisplay => FocusedArea::FileList,
        };
    }

    pub fn execute_query(&mut self) -> Result<()> {
        if !self.query_input.is_empty() {
            self.query_result = Some(self.json_viewer.query(&self.query_input)?);
            self.scroll_offset = 0; // Reset scroll when executing query
            self.cursor_line = 0; // Reset cursor when executing query
        }
        Ok(())
    }

    pub fn cursor_up(&mut self) {
        if self.cursor_line > 0 {
            self.cursor_line -= 1;
            // Auto-scroll up if cursor goes above visible area
            if self.cursor_line < self.scroll_offset {
                self.scroll_offset = self.cursor_line;
            }
        }
    }

    pub fn cursor_down(&mut self, total_lines: usize, visible_height: usize) {
        if total_lines > 0 && self.cursor_line < total_lines - 1 {
            self.cursor_line += 1;
            // Auto-scroll down if cursor goes below visible area
            let max_visible_line = self.scroll_offset + visible_height.saturating_sub(1);
            if self.cursor_line > max_visible_line {
                self.scroll_offset = self
                    .cursor_line
                    .saturating_sub(visible_height.saturating_sub(1));
            }
        }
    }

    pub fn toggle_wrap(&mut self) {
        self.wrap_lines = !self.wrap_lines;
    }

    pub fn toggle_collapse(&mut self) {
        // Toggle collapse state for the node at cursor position
        if self.cursor_line < self.line_to_path.len() {
            let path = &self.line_to_path[self.cursor_line];
            if !path.is_empty() {
                if self.collapsed_paths.contains(path) {
                    self.collapsed_paths.remove(path);
                } else {
                    self.collapsed_paths.insert(path.clone());
                }
            }
        }
    }

    pub fn navigate_parent_dir(&mut self) -> Result<()> {
        if let Some(parent) = self.root_path.parent() {
            self.root_path = parent.to_path_buf();
            self.file_browser = FileBrowser::new(&self.root_path)?;
            self.json_viewer.current_file = None;
            self.query_result = None;
            self.scroll_offset = 0;

            // Update watcher for new directory
            if let Some(watcher) = &mut self.watcher {
                let _ = watcher.watch(&self.root_path, notify::RecursiveMode::Recursive);
            }
        }
        Ok(())
    }
}
