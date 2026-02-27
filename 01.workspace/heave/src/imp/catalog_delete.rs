use crate::*;

impl Catalog {
    /// Marks an entity for deletion from the in-memory catalog.
    ///
    /// This is a purely in-memory operation. The entity will not be removed from the
    /// database until `persist()` is called.
    ///
    /// # Effects
    ///
    /// - **In-Memory State:** If the entity exists, its state is set to
    ///   `EntityState::ToDelete`. If it was in a `New` state, it will simply be
    ///   forgotten upon the next `persist()` call without ever touching the database.
    /// - **Database State:** Unchanged. Call `persist()` to apply the deletion.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the entity to mark for deletion.
    pub fn delete(&self, id: &str) -> Result<(), FailedTo> {
        self.on_items(|items| {
            let entity = items.get_mut(id);
            if let Some(entity) = entity {
                entity.state = EntityState::ToDelete;
            }
            Ok(())
        })
    }
}
