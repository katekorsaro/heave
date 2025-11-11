use crate::*;

impl Catalog {
    /// Initializes the database by creating the file and schema if they don't exist.
    ///
    /// This method interacts with the filesystem to ensure the database file and its
    /// underlying tables are ready. It has no effect on the in-memory state of the
    /// catalog.
    ///
    /// # Effects
    ///
    /// - **Database State:** Creates the SQLite file and required tables if they are not
    ///   present. If the file already exists, it does nothing.
    /// - **In-Memory State:** This method does not alter the in-memory entity cache.
    ///
    /// # Errors
    ///
    /// Returns `Err(FailedTo::InitDatabase)` if there is an issue creating the
    /// database file or initializing its schema.
    pub fn init(&self) -> result::Result<(), FailedTo> {
        let path = path::Path::new(&self.path);
        sqlite::init::db(path).map_err(|_| FailedTo::InitDatabase)?;
        Ok(())
    }
}

// #[cfg(test)]
// mod unit_tests { use super::*; }
