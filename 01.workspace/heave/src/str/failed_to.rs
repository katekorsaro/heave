use crate::*;

/// Represents the possible failures that can occur in the library.
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Hash)]
pub enum FailedTo {
    /// Failed to compose filter statement
    ComposeFilter,
    /// Failed to convert from Entity to type.
    ConvertEntity,
    /// Failed to convert from type to Entity.
    ConvertObject,
    /// Failed to convert from Value to type.
    ConvertValue,
    /// Failed to initialize the database.
    InitDatabase,
    /// Failed to load data from the database.
    LoadFromDB,
    /// Failed to lock catalog in a multithread environment.
    LockCatalog,
    /// Failed to map a database row to an attribute.
    MapAttribute,
    /// Failed to map a database row to an entity.
    MapEntity,
    /// Failed to persist the catalog to the database.
    PersistCatalog,
    /// A failure originating from the underlying SQLite implementation.
    SQLite(sqlite::FailedTo),
}

impl From<sqlite::FailedTo> for FailedTo {
    fn from(value: sqlite::FailedTo) -> Self {
        Self::SQLite(value)
    }
}
