use crate::*;

impl Catalog {
    /// Loads all entities of a specific class from the database into the in-memory catalog.
    ///
    /// This method fetches all entities matching the given class from the database.
    /// If any of the loaded entities have IDs that match entities already in the
    /// in-memory catalog, the in-memory versions will be **overwritten**.
    ///
    /// # Effects
    ///
    /// - **In-Memory State:**
    ///   - All found entities are inserted or updated in the in-memory cache with the
    ///     state `EntityState::Loaded`.
    ///   - Any existing in-memory entities with matching IDs are replaced.
    /// - **Database State:** Unchanged.
    pub fn load_by_class<T>(&mut self) -> Result<(), FailedTo>
    where
        T: EAV,
    {
        let class = T::class();
        let path = path::Path::new(&self.path);
        let entities = sqlite::load::by_class(path, class).map_err(|_| FailedTo::LoadFromDB)?;
        for entity in entities {
            self.items.insert(entity.id.clone(), entity);
        }
        Ok(())
    }
}

// #[cfg(test)]
// mod unit_tests { use super::*; }
