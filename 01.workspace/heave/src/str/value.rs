use crate::*;

/// Represents the value of an entity's attribute.
///
/// This enum can hold different data types, allowing for flexible and
/// semi-structured data storage.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Value {
    /// A boolean value (`true` or `false`).
    Bool(bool),
    /// A floating-point number (`f64`).
    Real(f64),
    /// A signed 64-bit integer.
    SignedInt(i64),
    /// A UTF-8 encoded string.
    Text(String),
    /// An unsigned 32-bit integer.
    UnsignedInt(u32),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let string_value = match self {
            Value::Bool(value) => match value {
                true => 1.to_string(),
                false => 0.to_string(),
            },
            Value::Real(value) => value.to_string(),
            Value::SignedInt(value) => value.to_string(),
            Value::UnsignedInt(value) => value.to_string(),
            Value::Text(value) => value.clone(),
        };
        write!(f, "{}", string_value)
    }
}
