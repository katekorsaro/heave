use crate::*;

/// A trait for types that can be represented as an Entity-Attribute-Value model.
///
/// This trait provides the necessary conversions to and from the generic `Entity`
/// representation, and it requires the type to define its own class name.
pub trait T where
    Self: TryFrom<Entity>,
    Self: TryInto<Entity>,
{
    /// Returns the class name of the type.
    ///
    /// This is used to distinguish between different types of entities in the database.
    fn class() -> &'static str;
}
