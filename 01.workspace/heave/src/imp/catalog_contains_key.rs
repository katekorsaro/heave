use crate::*;

impl Catalog {
    pub fn contains_key(&self, k: &str) -> Result<bool, FailedTo> {
        self.with_items(|items| Ok(items.contains_key(k)))
    }
}
