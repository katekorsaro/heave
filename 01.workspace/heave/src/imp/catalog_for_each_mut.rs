use crate::*;

impl Catalog {
    /// Iterates over each item in the catalog that can be converted into type `T` and applies a mutable predicate function.
    ///
    /// This method allows mutable iteration over entities. If the item is modified by the predicate,
    /// its state in the catalog will be marked as `EntityState::Updated`.
    ///
    /// # Type Parameters
    ///
    /// * `T`: The target type that entities should be converted into. Must implement `EAV`.
    /// * `F`: The type of the predicate function.
    ///
    /// # Arguments
    ///
    /// * `predicate`: A mutable closure that takes a mutable reference to an item of type `T` and returns a `Result<(), Box<dyn error::Error>>`. If the predicate returns an `Err`, it will be mapped to `FailedTo::ExecutePredicate`.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success (`Ok(())`) or failure (`Err(FailedTo)`).
    pub fn for_each_mut<T, F>(&self, mut predicate: F) -> Result<(), FailedTo>
    where
        T: EAV,
        F: FnMut(&mut T) -> Result<(), Box<dyn error::Error>>,
    {
        self.on_items(|items| {
            let mut errors: Vec<Box<dyn error::Error>> = Vec::new();
            for entity in items.values_mut() {
                let original_item =
                    T::try_from(entity.clone()).map_err(|_| FailedTo::ConvertEntity)?;
                let mut item = original_item.clone();
                let result = predicate(&mut item);
                if let Err(e) = result {
                    errors.push(e);
                }
                if item != original_item {
                    *entity = T::try_into(item).map_err(|_| FailedTo::ConvertObject)?;
                    entity.state = EntityState::Updated;
                }
            }
            if errors.is_empty() {
                Ok(())
            } else {
                Err(FailedTo::ExecutePredicate(errors))
            }
        })
    }
}
