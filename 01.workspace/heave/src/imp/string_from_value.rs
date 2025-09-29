use crate::*;

impl From<Value> for String {
    fn from(value: Value) -> String {
        match value {
            Value::Text(value) => value,
            _ => panic!("Type mismatch"),
        }
    }
}

// #[cfg(test)]
// mod unit_tests { use super::*; }
