use crate::*;

impl Catalog {
    /// Returns a list of all entities of a specific class and subclass from the in-memory catalog.
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
    /// A `Result` containing a `Vec` of `Result<T, FailedTo>>`. Each inner `Result`
    /// represents the outcome of converting an entity to type `T`.
    ///
    /// - `Ok(Vec<Ok(T)>)`: A vector of successfully converted entities.
    /// - `Ok(Vec<Err(FailedTo::ConvertEntity)>)`: If an entity of the correct class
    ///   and subclass could not be converted to type `T`.
    ///
    /// # Errors
    ///
    /// Returns `Err(FailedTo::LockCatalog)` if the catalog's internal mutex
    /// could not be locked.
    pub fn list_by_class_and_subclass<T>(
        &self,
        subclass: &str,
    ) -> Result<Vec<Result<T, FailedTo>>, FailedTo>
    where
        T: EAV,
    {
        self.with_items(|items| {
            Ok(items
                .values()
                .filter(move |item| item.class == T::class())
                .filter(move |item| item.subclass == Some(subclass.to_string()))
                .map(|item| T::try_from(item.clone()).map_err(|_| FailedTo::ConvertEntity))
                .collect())
        })
    }
}
