use crate::*;

impl Catalog {
    /// Ensures the database is initialized, performing the operation only once.
    ///
    /// This method provides an idempotent way to initialize the database. It's safe to
    /// call multiple times, but the actual initialization logic will only run if it
    /// hasn't already succeeded.
    ///
    /// # Effects
    ///
    /// - **Database State:** On the first successful call, this method creates the
    ///   SQLite file and schema if they don't exist. Subsequent calls have no effect.
    /// - **In-Memory State:** On the first successful call, sets an internal flag to
    ///   prevent future initializations. This method does not alter the in-memory
    ///   entity cache.
    ///
    /// # Errors
    ///
    /// Returns `Err(FailedTo::InitDatabase)` if the underlying call to `init()`
    /// fails. This can only happen on the first attempt.
    pub fn ensure_init(&mut self) -> Result<(), FailedTo> {
        if self.already_init {
            return Ok(());
        }
        self.init()?;
        self.already_init = true;
        Ok(())
    }
}
