use anyhow::Result;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct FileBrowser {
    pub files: Vec<PathBuf>,
    pub selected_index: usize,
}

impl FileBrowser {
    pub fn new(path: &Path) -> Result<Self> {
        let mut files = Vec::new();
        
        for entry in WalkDir::new(path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                // Only include JSON files and text files
                if let Some(ext) = path.extension() {
                    if ext == "json" || ext == "txt" {
                        files.push(path.to_path_buf());
                    }
                }
            }
        }

        files.sort();

        Ok(Self {
            files,
            selected_index: 0,
        })
    }

    pub fn next(&mut self) {
        if !self.files.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.files.len();
        }
    }

    pub fn previous(&mut self) {
        if !self.files.is_empty() {
            if self.selected_index > 0 {
                self.selected_index -= 1;
            } else {
                self.selected_index = self.files.len() - 1;
            }
        }
    }

    pub fn get_selected_file(&self) -> Option<&PathBuf> {
        self.files.get(self.selected_index)
    }

    pub fn get_display_name(&self, path: &Path) -> String {
        path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("???")
            .to_string()
    }
}
