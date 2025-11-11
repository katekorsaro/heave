use crate::*;

impl Catalog {
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
