use crate::*;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum E {
    Bool(bool),
    Real(f64),
    SignedInt(i64),
    Text(String),
    UnsignedInt(u64),
}

impl std::fmt::Display for E {
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
