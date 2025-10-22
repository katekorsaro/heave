use crate::*;

impl Catalog {
    /// Loads a single entity by its ID from the database into the in-memory catalog.
    ///
    /// This method fetches the entity from the database and updates the in-memory
    /// catalog. If an entity with the same ID already exists in memory, it will be
    /// **overwritten** with the version from the database.
    ///
    /// # Effects
    ///
    /// - **In-Memory State:**
    ///   - If an entity is found in the database, it is inserted or updated in the
    ///     in-memory cache with the state `EntityState::Loaded`.
    ///   - Any existing in-memory entity with the same ID, regardless of its state
    ///     (`New`, `Updated`), will be replaced.
    ///   - If no entity is found in the database, the in-memory cache is not modified.
    /// - **Database State:** Unchanged.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the entity to load.
    pub fn load_by_id(&mut self, id: &str) -> Result<(), FailedTo> {
        let path = path::Path::new(&self.path);
        let entity = sqlite::load::by_id(path, id).map_err(|_| FailedTo::LoadFromDB)?;
        if let Some(entity) = entity {
            self.items.insert(entity.id.clone(), entity);
        }
        Ok(())
    }
}

// #[cfg(test)]
// mod unit_tests { use super::*; }
