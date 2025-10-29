use crate::*;

/// Represents an attribute of an entity, consisting of an ID and a `Value`.
#[derive(Debug, PartialEq, Clone)]
pub struct O {
    /// The unique identifier for this attribute (e.g., "name", "price").
    pub id: String,
    /// The value of the attribute.
    pub value: Value,
}

impl Attribute {
    /// Creates a new `Attribute` instance.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the attribute.
    /// * `value` - The value of the attribute, which can be any type that
    ///   implements `Into<Value>`.
    ///
    /// # Returns
    ///
    /// A new `Attribute` instance.
    pub fn new(id: &str, value: impl Into<Value>) -> Self {
        Self {
            id: String::from(id),
            value: value.into(),
        }
    }
}
