use crate::*;
use std::collections::HashMap;
use std::ops::*;
use std::sync::*;

/// Represents a catalog of entities that can be persisted to a SQLite database.
///
/// The `Catalog` holds entities in memory and provides methods to interact with
/// them, as well as to persist changes to and load data from a database file.
#[derive(Debug, Default)]
pub struct O {
    pub(crate) path: String,
    pub(crate) already_init: bool,
    items: Mutex<HashMap<String, Entity>>,
}

impl Catalog {
    /// Creates a new, empty in-memory `Catalog` for the database at the given path.
    ///
    /// This method does not create the database file or connect to it. It only
    /// initializes an empty catalog in memory. The database file will be accessed
    /// when `init()`, `persist()`, or `load_*` methods are called.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the SQLite database file that this catalog will manage.
    ///
    /// # Returns
    ///
    /// A new `Catalog` instance with an empty in-memory item cache.
    pub fn new(path: &str) -> Self {
        Self {
            path: String::from(path),
            ..Catalog::default()
        }
    }
    pub(crate) fn on_items<F>(&self, exec: F) -> Result<(), FailedTo>
    where
        F: FnOnce(&mut HashMap<String, Entity>) -> Result<(), FailedTo>,
    {
        let mut guarded_items = self.items.lock().map_err(|_| FailedTo::LockCatalog)?;
        exec(&mut guarded_items)
    }
    pub(crate) fn with_items<F, R>(&self, exec: F) -> Result<R, FailedTo>
    where
        F: FnOnce(&HashMap<String, Entity>) -> Result<R, FailedTo>,
    {
        let guarded_items = self.items.lock().map_err(|_| FailedTo::LockCatalog)?;
        exec(&guarded_items)
    }
}
