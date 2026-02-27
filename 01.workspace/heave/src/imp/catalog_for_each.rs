use crate::*;

impl Catalog {
    /// Iterates over each item in the catalog that can be converted into type `T` and applies a predicate function.
    ///
    /// This method allows read-only iteration over entities. The predicate receives an immutable reference
    /// to the item.
    ///
    /// # Type Parameters
    ///
    /// * `T`: The target type that entities should be converted into. Must implement `EAV`.
    /// * `F`: The type of the predicate function.
    ///
    /// # Arguments
    ///
    /// * `predicate`: A mutable closure that takes an immutable reference to an item of type `T`.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success (`Ok(())`) or failure (`Err(FailedTo::LockCatalog)`).
    /// Entities that cannot be converted to type `T` are silently skipped from the iteration
    /// and do not cause this function to return an error.
    pub fn for_each<T, F>(&self, mut predicate: F) -> Result<(), FailedTo>
    where
        T: EAV,
        F: FnMut(&T),
    {
        self.with_items(|items| {
            items
                .values()
                .flat_map(|entity| T::try_from(entity.clone()).map_err(|_| FailedTo::ConvertEntity))
                .for_each(|item| predicate(&item));
            Ok(())
        })
    }
}
