use crate::*;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Hash)]
pub enum E {
    InitDatabase,
    LoadFromDB,
    MapAttribute,
    MapEntity,
    PersistCatalog,
    SQLite(sqlite::FailedTo),
}

impl From<sqlite::FailedTo> for E {
    fn from(value: sqlite::FailedTo) -> Self {
        Self::SQLite(value)
    }
}
