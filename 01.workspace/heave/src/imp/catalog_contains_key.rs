use crate::*;

impl Catalog {
    /// Checks if the catalog contains an entity with the specified ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID to check for.
    ///
    /// # Returns
    ///
    /// `Ok(true)` if an entity with the specified ID exists in the catalog,
    /// `Ok(false)` otherwise.
    ///
    /// # Errors
    ///
    /// Returns `Err(FailedTo::LockCatalog)` if the catalog's internal mutex
    /// could not be locked.
    pub fn contains_key(&self, id: &str) -> Result<bool, FailedTo> {
        self.with_items(|items| Ok(items.contains_key(id)))
    }
}
