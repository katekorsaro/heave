use crate::*;

impl From<Value> for u64 {
    fn from(value: Value) -> u64 {
        match value {
            Value::UnsignedInt(value) => value,
            _ => panic!("Type mismatch"),
        }
    }
}
