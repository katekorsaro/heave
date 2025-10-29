use crate::*;

impl Catalog {
    /// Inserts or updates an object in the in-memory catalog.
    ///
    /// This is a purely in-memory operation. The change will not be written to the
    /// database until `persist()` is called.
    ///
    /// # Effects
    ///
    /// - **In-Memory State:**
    ///   - If the entity does not exist in the catalog, it is added with the state
    ///     `EntityState::New`.
    ///   - If the entity already exists, it is overwritten and its state is set to
    ///     `EntityState::Updated`.
    /// - **Database State:** Unchanged. Call `persist()` to save the changes.
    ///
    /// # Arguments
    ///
    /// * `object` - The object to insert or update, which must implement the `EAV` trait.
    pub fn upsert(&mut self, object: impl EAV) -> Result<(), FailedTo> {
        let mut entity = object.try_into().map_err(|_| FailedTo::ConvertObject)?;
        self.on_items(|items| {
            if items.contains_key(&entity.id) {
                entity.state = EntityState::Updated;
            } else {
                entity.state = EntityState::New;
            }
            items.insert(entity.id.clone(), entity);
            Ok(())
        })
    }
}
