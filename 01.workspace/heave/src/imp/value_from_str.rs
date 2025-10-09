use crate::*;

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Self::Text(String::from(value))
    }
}
