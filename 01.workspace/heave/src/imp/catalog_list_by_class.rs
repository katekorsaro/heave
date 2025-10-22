use crate::*;

impl Catalog {
    /// Returns an iterator over all entities of a specific class in the in-memory catalog.
    ///
    /// This is a purely in-memory operation and does not interact with the database.
    ///
    /// # Returns
    ///
    /// An iterator that yields items of type `T` from the in-memory cache.
    pub fn list_by_class<T>(&self) -> impl Iterator<Item = Result<T, FailedTo>>
    where
        T: EAV,
    {
        self.items
            .values()
            .filter(move |item| item.class == T::class())
            .map(|item| T::try_from(item.clone()).map_err(|_| FailedTo::ConvertEntity))
    }
}

// #[cfg(test)]
// mod unit_tests { use super::*; }
