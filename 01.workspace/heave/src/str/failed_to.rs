use crate::*;

/// Represents the possible failures that can occur in the library.
#[derive(Debug)]
pub enum FailedTo {
    /// Failed to compose filter statement
    ComposeFilter,
    /// Failed to convert from Entity to type.
    ConvertEntity,
    /// Failed to convert from type to Entity.
    ConvertObject,
    /// Failed to convert from Value to type.
    ConvertValue,
    /// Failed to execute predicate to mutate an item.
    ExecutePredicate(Vec<Box<dyn error::Error>>),
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

impl PartialEq for FailedTo {
    fn eq(&self, other: &FailedTo) -> bool {
        matches!(
            (self, other),
            (FailedTo::ComposeFilter, FailedTo::ComposeFilter)
                | (FailedTo::ConvertEntity, FailedTo::ConvertEntity)
                | (FailedTo::ConvertObject, FailedTo::ConvertObject)
                | (FailedTo::ConvertValue, FailedTo::ConvertValue)
                | (FailedTo::ExecutePredicate(_), FailedTo::ExecutePredicate(_))
                | (FailedTo::InitDatabase, FailedTo::InitDatabase)
                | (FailedTo::LoadFromDB, FailedTo::LoadFromDB)
                | (FailedTo::LockCatalog, FailedTo::LockCatalog)
                | (FailedTo::MapAttribute, FailedTo::MapAttribute)
                | (FailedTo::MapEntity, FailedTo::MapEntity)
                | (FailedTo::PersistCatalog, FailedTo::PersistCatalog)
                | (FailedTo::SQLite(_), FailedTo::SQLite(_))
        )
    }
}

impl From<sqlite::FailedTo> for FailedTo {
    fn from(value: sqlite::FailedTo) -> Self {
        Self::SQLite(value)
    }
}
