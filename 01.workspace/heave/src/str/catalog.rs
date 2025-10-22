use crate::*;
use std::collections::HashMap;

/// Represents a catalog of entities that can be persisted to a SQLite database.
///
/// The `Catalog` holds entities in memory and provides methods to interact with
/// them, as well as to persist changes to and load data from a database file.
#[derive(Debug, Default, PartialEq, Clone)]
pub struct O {
    pub(crate) path: String,
    pub(crate) items: HashMap<String, Entity>,
}
