use crate::*;

impl Catalog {
    /// Loads entities from the database that match a given `Filter`.
    ///
    /// This method queries the database for all entities that satisfy the conditions
    /// specified in the filter. If any of the loaded entities have IDs that match
    /// entities already in the in-memory catalog, the in-memory versions will be
    /// **overwritten**.
    ///
    /// **Warning:** The filter is applied at the attribute level. If different entity
    /// classes share attribute names, this method may load entities of multiple
    /// classes.
    ///
    /// # Effects
    ///
    /// - **In-Memory State:**
    ///   - All found entities are inserted or updated in the in-memory cache with the
    ///     state `EntityState::Loaded`.
    ///   - Any existing in-memory entities with matching IDs are replaced.
    /// - **Database State:** Unchanged.
    pub fn load_by_filter(&mut self, filter: &Filter) -> Result<(), FailedTo> {
        let path = path::Path::new(&self.path);
        self.on_items(|items| {
            let entities =
                sqlite::load::by_filter(path, filter).map_err(|_| FailedTo::LoadFromDB)?;
            for entity in entities {
                items.insert(entity.id.clone(), entity);
            }
            Ok(())
        })
    }
}

// #[cfg(test)]
// mod unit_tests { use super::*; }
