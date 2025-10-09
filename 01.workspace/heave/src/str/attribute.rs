use crate::*;

#[derive(Debug, PartialEq, Clone)]
pub struct O {
    pub id: String,
    pub value: Value,
}

impl Attribute {
    pub fn new(id: &str, value: impl Into<Value>) -> Self {
        Self {
            id: String::from(id),
            value: value.into(),
        }
    }
}
