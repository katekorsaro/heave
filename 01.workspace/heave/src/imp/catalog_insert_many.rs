use crate::*;

impl Catalog {
    /// Inserts or updates multiple objects in the in-memory catalog.
    ///
    /// This method calls `upsert()` for each object in the provided vector. Like
    /// `upsert()`, this is a purely in-memory operation.
    ///
    /// # Effects
    ///
    /// - **In-Memory State:** Adds or updates multiple entities in the cache.
    /// - **Database State:** Unchanged. Call `persist()` to save the changes.
    ///
    /// # Arguments
    ///
    /// * `objects` - A vector of objects to insert or update.
    ///
    /// # Errors
    ///
    /// Returns `Err(FailedTo)` if any of the underlying `upsert` operations fail.
    /// This could be due to issues like an object not having a valid ID.
    pub fn insert_many(&mut self, objects: Vec<impl EAV>) -> Result<(), FailedTo> {
        for object in objects {
            self.upsert(object)?;
        }
        Ok(())
    }
}

// #[cfg(test)]
// mod unit_tests { use super::*; }
