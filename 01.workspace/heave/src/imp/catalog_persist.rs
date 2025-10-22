use crate::*;

impl Catalog {
    /// Persists all in-memory changes to the database and updates the in-memory state.
    ///
    /// This method synchronizes the state of the in-memory catalog with the database
    /// by writing all pending changes (new, updated, and deleted entities).
    ///
    /// # Effects
    ///
    /// - **Database State:**
    ///   - Entities marked as `EntityState::New` are inserted into the database.
    ///   - Entities marked as `EntityState::Updated` are updated in the database.
    ///   - Entities marked as `EntityState::ToDelete` are deleted from the database.
    ///
    /// - **In-Memory State:**
    ///   - After a successful database write, entities marked `ToDelete` are permanently
    ///     removed from the in-memory catalog.
    ///   - All other entities that were successfully persisted (new or updated) have
    ///     their state changed to `EntityState::Loaded`.
    pub fn persist(&mut self) -> result::Result<(), FailedTo> {
        let path = path::Path::new(&self.path);
        sqlite::persist::catalog(path, self).map_err(|_| FailedTo::PersistCatalog)?;
        // cleaning catalog state after db write
        self.items = self
            .items
            .extract_if(|_, item| item.state != EntityState::ToDelete)
            .map(|(k, item)| {
                (
                    k,
                    Entity {
                        state: EntityState::Loaded,
                        ..item
                    },
                )
            })
            .collect();
        Ok(())
    }
}

// #[cfg(test)]
// mod unit_tests { use super::*; }
