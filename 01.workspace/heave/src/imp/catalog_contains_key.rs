use crate::*;

impl Catalog {
    /// Checks if the catalog contains an entity with the specified key.
    ///
    /// # Arguments
    ///
    /// * `k` - The key to check for.
    ///
    /// # Returns
    ///
    /// `Ok(true)` if an entity with the specified key exists in the catalog,
    /// `Ok(false)` otherwise.
    ///
    /// # Errors
    ///
    /// Returns `Err(FailedTo::LockCatalog)` if the catalog's internal mutex
    /// could not be locked.
    pub fn contains_key(&self, k: &str) -> Result<bool, FailedTo> {
        self.with_items(|items| Ok(items.contains_key(k)))
    }
}
