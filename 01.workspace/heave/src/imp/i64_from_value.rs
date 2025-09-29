use crate::*;

impl From<Value> for i64 {
    fn from(value: Value) -> i64 {
        match value {
            Value::SignedInt(value) => value,
            _ => panic!("Type mismatch"),
        }
    }
}

// #[cfg(test)]
// mod unit_tests { use super::*; }
