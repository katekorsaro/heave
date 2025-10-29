use crate::*;

impl Catalog {
    /// Returns the number of entities in the catalog.
    ///
    /// # Returns
    ///
    /// The total number of entities currently held in the catalog's memory.
    ///
    /// # Errors
    ///
    /// Returns `Err(FailedTo::LockCatalog)` if the catalog's internal mutex
    /// could not be locked.
    pub fn len(&self) -> Result<usize, FailedTo> {
        self.with_items(|items| Ok(items.len()))
    }
}
