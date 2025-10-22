use crate::*;

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
}
