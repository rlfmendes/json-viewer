use anyhow::Result;
use std::any::Any;
use std::path::PathBuf;

/// Represents an entry in the data source (file, message, etc.)
#[derive(Clone, Debug)]
pub struct DataEntry {
    pub id: String,
    pub display_name: String,
    pub is_navigable: bool, // true for directories, false for files/messages
    pub metadata: Option<String>, // Optional extra info to display
}

/// Trait for abstracting different data sources (filesystem, MQTT, etc.)
pub trait DataSource {
    /// Get all entries at the current level
    fn get_entries(&self) -> &[DataEntry];

    /// Get the content of a specific entry by ID
    fn get_content(&self, id: &str) -> Result<String>;

    /// Get the total number of entries
    fn entry_count(&self) -> usize;

    /// Move to the next entry
    fn next(&mut self);

    /// Move to the previous entry
    fn previous(&mut self);

    /// Get the currently selected index
    fn selected_index(&self) -> usize;

    /// Get the scroll offset for rendering
    fn scroll_offset(&self) -> usize;

    /// Ensure the selected item is visible
    fn ensure_visible(&mut self, visible_height: usize);

    /// Get the selected entry, if any
    fn get_selected_entry(&self) -> Option<&DataEntry>;

    /// Whether this data source supports directory navigation
    fn supports_navigation(&self) -> bool;

    /// Navigate into a directory (only for sources that support it)
    fn navigate_into(&mut self) -> Result<()>;

    /// Navigate to parent directory (only for sources that support it)
    fn navigate_parent(&mut self) -> Result<()>;

    /// Check if parent navigation is available
    fn has_parent(&self) -> bool;

    /// Refresh/update the list of entries
    fn refresh(&mut self) -> Result<()>;

    /// Get current path or location description
    fn get_location(&self) -> String;

    /// Get a file path for the selected entry (filesystem only, returns None for others)
    fn get_file_path(&self) -> Option<PathBuf> {
        None
    }

    /// Get access to Any for downcasting to concrete types
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
