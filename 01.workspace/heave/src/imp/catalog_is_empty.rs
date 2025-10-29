use crate::*;

impl Catalog {
    pub fn is_empty(&self) -> Result<bool, FailedTo> {
        self.with_items(|items| Ok(items.is_empty()))
    }
}
