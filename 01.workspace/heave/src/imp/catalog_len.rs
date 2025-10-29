use crate::*;

impl Catalog {
    pub fn len(&self) -> Result<usize, FailedTo> {
        self.with_items(|items| Ok(items.len()))
    }
}
