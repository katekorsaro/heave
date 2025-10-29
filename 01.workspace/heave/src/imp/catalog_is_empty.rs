use crate::*;

impl Catalog {
    /// Checks if the catalog is empty.
    ///
    /// # Returns
    ///
    /// `Ok(true)` if the catalog contains no entities, `Ok(false)` otherwise.
    ///
    /// # Errors
    ///
    /// Returns `Err(FailedTo::LockCatalog)` if the catalog's internal mutex
    /// could not be locked.
    pub fn is_empty(&self) -> Result<bool, FailedTo> {
        self.with_items(|items| Ok(items.is_empty()))
    }
}
