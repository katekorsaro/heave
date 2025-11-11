use crate::*;

impl Catalog {
    pub fn for_each_mut<T, F>(&self, mut predicate: F) -> Result<(), FailedTo>
    where
        T: EAV,
        F: FnMut(&mut T),
    {
        self.on_items(|items| {
            for entity in items.values_mut() {
                let original_item =
                    T::try_from(entity.clone()).map_err(|_| FailedTo::ConvertEntity)?;
                let mut item = T::try_from(entity.clone()).map_err(|_| FailedTo::ConvertEntity)?;
                predicate(&mut item);
                if item != original_item {
                    *entity = T::try_into(item).map_err(|_| FailedTo::ConvertObject)?;
                    entity.state = EntityState::Updated;
                }
            }
            Ok(())
        })
    }
}
