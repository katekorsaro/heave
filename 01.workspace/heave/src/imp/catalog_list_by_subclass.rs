use crate::*;

impl Catalog {
    /// Returns a list of all entities of a specific subclass from the in-memory catalog.
    ///
    /// This method filters the in-memory entities by the class of type `T` and the
    /// provided `subclass`, then attempts to convert them into `T`. This is a purely
    /// in-memory operation and does not interact with the database.
    ///
    /// # Arguments
    ///
    /// * `subclass` - The subclass to filter by.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `Vec<T>` of entities that were successfully
    /// converted to type `T`. Entities that fail conversion are silently
    /// skipped and not included in the returned vector.
    ///
    /// # Errors
    ///
    /// Returns `Err(FailedTo::LockCatalog)` if the catalog's internal mutex
    /// could not be locked.
    pub fn list_by_subclass<T>(&self, subclass: &str) -> Result<Vec<T>, FailedTo>
    where
        T: EAV,
    {
        self.with_items(|items| {
            Ok(items
                .values()
                .filter(move |item| item.class == T::class())
                .filter(move |item| item.subclass.as_deref() == Some(subclass))
                .filter_map(|item| T::try_from(item.clone()).ok())
                .collect())
        })
    }
}
