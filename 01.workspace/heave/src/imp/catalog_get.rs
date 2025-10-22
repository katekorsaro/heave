use crate::*;

impl Catalog {
    /// Retrieves an entity by its ID from the in-memory catalog.
    ///
    /// This is a purely in-memory operation and does not interact with the database.
    /// It will only find entities that have been loaded into or created in the catalog.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the entity to retrieve.
    ///
    /// # Returns
    ///
    /// An `Option<T>` containing the converted entity if found in the in-memory
    /// cache, otherwise `None`.
    pub fn get<T>(&self, id: &str) -> Result<Option<T>, FailedTo>
    where
        T: EAV,
    {
        let entity = self.items.get(id);
        entity
            .map(|e| T::try_from(e.clone()).map_err(|_| FailedTo::ConvertEntity))
            .transpose()
    }
}

// #[cfg(test)]
// mod unit_tests { use super::*; }
