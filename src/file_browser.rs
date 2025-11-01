use anyhow::Result;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub enum FileEntry {
   ParentDir,
   File(PathBuf),
}

pub struct FileBrowser {
    pub files: Vec<PathBuf>,
    pub selected_index: usize,
   pub current_path: PathBuf,
   pub has_parent: bool,
}

impl FileBrowser {
    pub fn new(root: &Path) -> Result<Self> {
        let current_path = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());
        let mut files = Vec::new();
        
        for entry in WalkDir::new(&current_path)
            .max_depth(1)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            let canonical_path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
            if canonical_path != current_path && path.is_file() {
                // Only include JSON files and text files
                if let Some(ext) = path.extension() {
                    if ext == "json" || ext == "txt" {
                        files.push(path.to_path_buf());
                    }
                }
            }
        }        files.sort();

        let has_parent = current_path.parent().is_some();

        Ok(Self {
            files,
            selected_index: 0,
           current_path,
           has_parent,
        })
    }

   pub fn entry_count(&self) -> usize {
       if self.has_parent {
           self.files.len() + 1 // +1 for ".." parent entry
       } else {
           self.files.len()
       }
   }

   pub fn get_entry_at(&self, index: usize) -> Option<FileEntry> {
       if self.has_parent {
           if index == 0 {
               return Some(FileEntry::ParentDir);
           }
           self.files.get(index - 1).map(|p| FileEntry::File(p.clone()))
       } else {
           self.files.get(index).map(|p| FileEntry::File(p.clone()))
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

    pub fn get_selected_file(&self) -> Option<&PathBuf> {
       if self.has_parent {
           if self.selected_index == 0 {
               return None; // Parent directory selected
           }
           self.files.get(self.selected_index - 1)
       } else {
           self.files.get(self.selected_index)
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
       let mut files = Vec::new();
       
       for entry in WalkDir::new(&self.current_path)
           .max_depth(1)
           .follow_links(true)
           .into_iter()
           .filter_map(|e| e.ok())
       {
           let path = entry.path();
           let canonical_path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
           if canonical_path != self.current_path && path.is_file() {
               if let Some(ext) = path.extension() {
                   if ext == "json" || ext == "txt" {
                       files.push(path.to_path_buf());
                   }
               }
           }
       }

       files.sort();
       
       // Maintain selection if possible
       if self.selected_index >= files.len() + if self.has_parent { 1 } else { 0 } {
           self.selected_index = 0;
       }
       
       self.files = files;
       Ok(())
   }
}
