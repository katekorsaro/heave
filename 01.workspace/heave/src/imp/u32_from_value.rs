use crate::*;

impl From<Value> for u32 {
    fn from(value: Value) -> u32 {
        match value {
            Value::UnsignedInt(value) => value
                .try_into()
                .map_err(|_| "Type mismatch".to_string())
                .unwrap(),
            _ => panic!("Type mismatch"),
        }
    }
}
