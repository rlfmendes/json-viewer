use crate::data_source::{DataEntry, DataSource};
use anyhow::Result;
use std::any::Any;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Clone)]
enum FileEntry {
    Directory(PathBuf),
    File(PathBuf),
}

pub struct FilesystemSource {
    entries: Vec<FileEntry>,
    data_entries: Vec<DataEntry>,
    selected_index: usize,
    current_path: PathBuf,
    has_parent: bool,
    scroll_offset: usize,
}

impl FilesystemSource {
    pub fn new(root: &Path) -> Result<Self> {
        let current_path = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());
        let mut entries = Vec::new();

        for entry in WalkDir::new(&current_path)
            .max_depth(1)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            let canonical_path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
            if canonical_path != current_path {
                if path.is_dir() {
                    entries.push(FileEntry::Directory(path.to_path_buf()));
                } else if path.is_file() {
                    // Only include JSON files and text files
                    if let Some(ext) = path.extension() {
                        if ext == "json" || ext == "txt" {
                            entries.push(FileEntry::File(path.to_path_buf()));
                        }
                    }
                }
            }
        }

        // Sort entries: directories first, then files
        entries.sort_by(|a, b| match (a, b) {
            (FileEntry::Directory(p1), FileEntry::Directory(p2)) => p1.cmp(p2),
            (FileEntry::File(p1), FileEntry::File(p2)) => p1.cmp(p2),
            (FileEntry::Directory(_), FileEntry::File(_)) => std::cmp::Ordering::Less,
            (FileEntry::File(_), FileEntry::Directory(_)) => std::cmp::Ordering::Greater,
        });

        let has_parent = current_path.parent().is_some();
        let data_entries = Self::convert_to_data_entries(&entries, has_parent);

        Ok(Self {
            entries,
            data_entries,
            selected_index: 0,
            current_path,
            has_parent,
            scroll_offset: 0,
        })
    }

    fn convert_to_data_entries(entries: &[FileEntry], has_parent: bool) -> Vec<DataEntry> {
        let mut data_entries = Vec::new();

        // Add parent entry if available
        if has_parent {
            data_entries.push(DataEntry {
                id: "..".to_string(),
                display_name: "[D] ..".to_string(),
                is_navigable: true,
                metadata: None,
            });
        }

        // Convert file entries
        for (idx, entry) in entries.iter().enumerate() {
            let (prefix, path, is_navigable) = match entry {
                FileEntry::Directory(path) => ("[D]", path, true),
                FileEntry::File(path) => ("[-]", path, false),
            };

            let display_name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("???");

            data_entries.push(DataEntry {
                id: idx.to_string(),
                display_name: format!("{prefix} {display_name}"),
                is_navigable,
                metadata: None,
            });
        }

        data_entries
    }

    fn get_selected_file(&self) -> Option<&PathBuf> {
        if self.has_parent {
            if self.selected_index == 0 {
                return None; // Parent directory selected
            }
            match self.entries.get(self.selected_index - 1) {
                Some(FileEntry::File(path)) => Some(path),
                _ => None,
            }
        } else {
            match self.entries.get(self.selected_index) {
                Some(FileEntry::File(path)) => Some(path),
                _ => None,
            }
        }
    }

    fn get_selected_directory(&self) -> Option<&PathBuf> {
        if self.has_parent {
            if self.selected_index == 0 {
                return None; // Parent directory selected
            }
            match self.entries.get(self.selected_index - 1) {
                Some(FileEntry::Directory(path)) => Some(path),
                _ => None,
            }
        } else {
            match self.entries.get(self.selected_index) {
                Some(FileEntry::Directory(path)) => Some(path),
                _ => None,
            }
        }
    }

    #[allow(dead_code)]
    fn is_parent_selected(&self) -> bool {
        self.has_parent && self.selected_index == 0
    }
}

impl DataSource for FilesystemSource {
    fn get_entries(&self) -> &[DataEntry] {
        &self.data_entries
    }

    fn get_content(&self, _id: &str) -> Result<String> {
        if let Some(path) = self.get_selected_file() {
            std::fs::read_to_string(path).map_err(|e| e.into())
        } else {
            Ok(String::new())
        }
    }

    fn entry_count(&self) -> usize {
        self.data_entries.len()
    }

    fn next(&mut self) {
        let count = self.entry_count();
        if count > 0 {
            self.selected_index = (self.selected_index + 1) % count;
        }
    }

    fn previous(&mut self) {
        let count = self.entry_count();
        if count > 0 {
            if self.selected_index > 0 {
                self.selected_index -= 1;
            } else {
                self.selected_index = count - 1;
            }
        }
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn scroll_offset(&self) -> usize {
        self.scroll_offset
    }

    fn ensure_visible(&mut self, visible_height: usize) {
        if self.selected_index < self.scroll_offset {
            self.scroll_offset = self.selected_index;
        } else if self.selected_index >= self.scroll_offset + visible_height {
            self.scroll_offset = self.selected_index.saturating_sub(visible_height - 1);
        }
    }

    fn get_selected_entry(&self) -> Option<&DataEntry> {
        self.data_entries.get(self.selected_index)
    }

    fn supports_navigation(&self) -> bool {
        true
    }

    fn navigate_into(&mut self) -> Result<()> {
        if let Some(dir_path) = self.get_selected_directory() {
            let new_path = dir_path.clone();
            *self = Self::new(&new_path)?;
        }
        Ok(())
    }

    fn navigate_parent(&mut self) -> Result<()> {
        if let Some(parent) = self.current_path.parent() {
            *self = Self::new(parent)?;
        }
        Ok(())
    }

    fn has_parent(&self) -> bool {
        self.has_parent
    }

    fn refresh(&mut self) -> Result<()> {
        let current_path = self.current_path.clone();
        *self = Self::new(&current_path)?;
        Ok(())
    }

    fn get_location(&self) -> String {
        self.current_path
            .to_str()
            .unwrap_or("Unknown")
            .to_string()
    }

    fn get_file_path(&self) -> Option<PathBuf> {
        self.get_selected_file().cloned()
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn status_text(&self) -> Option<String> {
        Some(format!(
            "Path: {} | Entries: {}",
            self.current_path.to_string_lossy(),
            self.data_entries.len()
        ))
    }
}
