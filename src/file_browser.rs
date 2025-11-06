use anyhow::Result;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Clone)]
enum FileEntry {
    Directory(PathBuf),
    File(PathBuf),
}

#[allow(dead_code)]
pub struct FileBrowser {
    #[allow(private_interfaces)]
    pub entries: Vec<FileEntry>,
    pub selected_index: usize,
    pub current_path: PathBuf,
    pub has_parent: bool,
    pub scroll_offset: usize,
}

#[allow(dead_code)]
impl FileBrowser {
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

        Ok(Self {
            entries,
            selected_index: 0,
            current_path,
            has_parent,
            scroll_offset: 0,
        })
    }

    pub fn entry_count(&self) -> usize {
        if self.has_parent {
            self.entries.len() + 1 // +1 for ".." parent entry
        } else {
            self.entries.len()
        }
    }

    pub fn next(&mut self) {
        let count = self.entry_count();
        if count > 0 {
            self.selected_index = (self.selected_index + 1) % count;
        }
    }

    pub fn previous(&mut self) {
        let count = self.entry_count();
        if count > 0 {
            if self.selected_index > 0 {
                self.selected_index -= 1;
            } else {
                self.selected_index = count - 1;
            }
        }
    }

    pub fn ensure_visible(&mut self, visible_height: usize) {
        // Ensure the selected item is visible within the scroll window
        if self.selected_index < self.scroll_offset {
            self.scroll_offset = self.selected_index;
        } else if self.selected_index >= self.scroll_offset + visible_height {
            self.scroll_offset = self.selected_index.saturating_sub(visible_height - 1);
        }
    }

    pub fn get_selected_file(&self) -> Option<&PathBuf> {
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

    pub fn get_selected_directory(&self) -> Option<&PathBuf> {
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

    pub fn is_parent_selected(&self) -> bool {
        self.has_parent && self.selected_index == 0
    }

    pub fn get_display_name(&self, path: &Path) -> String {
        path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("???")
            .to_string()
    }

    pub fn refresh(&mut self) -> Result<()> {
        let mut entries = Vec::new();

        for entry in WalkDir::new(&self.current_path)
            .max_depth(1)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            let canonical_path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
            if canonical_path != self.current_path {
                if path.is_dir() {
                    entries.push(FileEntry::Directory(path.to_path_buf()));
                } else if path.is_file() {
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

        // Maintain selection if possible
        if self.selected_index >= entries.len() + if self.has_parent { 1 } else { 0 } {
            self.selected_index = 0;
        }

        self.entries = entries;
        Ok(())
    }
}
