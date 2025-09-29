use crate::*;

impl From<Value> for f64 {
    fn from(value: Value) -> f64 {
        match value {
            Value::Real(value) => value,
            _ => panic!("Type mismatch"),
        }
    }
}

// #[cfg(test)]
// mod unit_tests { use super::*; }
