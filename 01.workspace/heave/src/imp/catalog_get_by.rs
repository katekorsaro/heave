use crate::*;

impl Catalog {
    /// Retrieves the first entity from the in-memory catalog that satisfies a given predicate.
    ///
    /// This is a purely in-memory operation and does not interact with the database.
    /// It iterates through all entities currently loaded in the catalog, attempts to
    /// convert each to the specified type `T`, and then applies the provided
    /// `predicate` function. The first entity that successfully converts and
    /// satisfies the predicate is returned.
    ///
    /// Entities that fail to convert to type `T` are silently skipped and do not
    /// cause an error to be returned by this function.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The target type to convert the entity into, which must implement the `EAV` trait.
    /// * `F` - The type of the predicate closure, which takes a reference to `T` and returns a `bool`.
    ///
    /// # Arguments
    ///
    /// * `predicate` - A closure that defines the condition an entity must meet to be returned.
    ///
    /// # Returns
    ///
    /// An `Ok(Some(T))` containing the first converted entity that satisfies the predicate,
    /// or `Ok(None)` if no such entity is found or if all matching entities fail conversion.
    ///
    /// # Errors
    ///
    /// Returns `Err(FailedTo)` if an internal error occurs during the `with_items` operation
    /// (e.g., mutex poisoning).
    pub fn get_by<T, F>(&self, predicate: F) -> Result<Option<T>, FailedTo>
    where
        T: EAV,
        F: Fn(&T) -> bool,
    {
        self.with_items(|items| {
            Ok(items
                .values()
                .flat_map(|entity| T::try_from(entity.clone()))
                .find(|item| predicate(item)))
        })
    }
}
