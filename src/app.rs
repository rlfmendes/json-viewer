use anyhow::Result;
use std::path::PathBuf;
use crate::file_browser::FileBrowser;
use crate::json_viewer::JsonViewer;

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
}

impl App {
    pub fn new(path: PathBuf) -> Result<Self> {
        let file_browser = FileBrowser::new(&path)?;
        let json_viewer = JsonViewer::new();
        
        Ok(Self {
            file_browser,
            json_viewer,
            input_mode: InputMode::Normal,
            view_mode: ViewMode::Hierarchical,
            focused_area: FocusedArea::FileList,
            query_input: String::new(),
            query_result: None,
            root_path: path,
            scroll_offset: 0,
            wrap_lines: true,
            fold_depth: None,
        })
    }

    pub fn select_file(&mut self) -> Result<()> {
        if let Some(file_path) = self.file_browser.get_selected_file() {
            self.json_viewer.load_file(file_path)?;
            self.query_result = None;
            self.scroll_offset = 0; // Reset scroll when opening new file
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
        }
        Ok(())
    }

    pub fn scroll_up(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(1);
    }

    pub fn scroll_down(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_add(1);
    }

    pub fn toggle_wrap(&mut self) {
        self.wrap_lines = !self.wrap_lines;
    }

    pub fn toggle_collapse(&mut self) {
        // Toggle between fully expanded (None) and collapsed to top-level (Some(1))
        self.fold_depth = match self.fold_depth {
            Some(_) => None,
            None => Some(1),
        };
    }

    pub fn navigate_parent_dir(&mut self) -> Result<()> {
        if let Some(parent) = self.root_path.parent() {
            self.root_path = parent.to_path_buf();
            self.file_browser = FileBrowser::new(&self.root_path)?;
            self.json_viewer.current_file = None;
            self.query_result = None;
            self.scroll_offset = 0;
        }
        Ok(())
    }
}
