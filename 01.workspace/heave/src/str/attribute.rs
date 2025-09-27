use crate::*;

#[derive(Debug, PartialEq, Clone)]
pub struct O {
    pub id: String,
    pub value: Value,
}

impl O {
    pub fn new(id: &str, value: impl ToValue) -> Self {
        Self {
            id: String::from(id),
            value: value.to_value(),
        }
    }
}
