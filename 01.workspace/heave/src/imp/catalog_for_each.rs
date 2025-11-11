use crate::*;

impl Catalog {
    pub fn for_each<T, F>(&self, predicate: F) -> Result<(), FailedTo>
    where
        T: EAV,
        F: Fn(&T) -> (),
    {
        self.with_items(|items| {
            Ok(items
                .values()
                .flat_map(|entity| T::try_from(entity.clone()).map_err(|_| FailedTo::ConvertEntity))
                .for_each(|item| predicate(&item)))
        })
    }
}
