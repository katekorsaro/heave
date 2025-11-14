use crate::*;

impl Catalog {
    pub fn ensure_init(&mut self) -> Result<(), FailedTo> {
        if self.already_init {
            return Ok(());
        }
        let result = self.init()?;
        self.already_init = true;
        Ok(())
    }
}
