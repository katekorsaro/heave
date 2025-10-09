/// Represents the state of an entity within the catalog.
#[derive(Debug, Default, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Hash)]
pub enum E {
    /// The entity is newly created and has not been persisted.
    #[default]
    New,
    /// The state of the entity is unknown.
    Unknown,
    /// The entity has been loaded from the database.
    Loaded,
    /// The entity is marked for deletion.
    ToDelete,
}