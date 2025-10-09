use crate::*;

impl From<Value> for bool {
    fn from(value: Value) -> bool {
        match value {
            Value::Bool(value) => value,
            _ => panic!("Type mismatch"),
        }
    }
}
