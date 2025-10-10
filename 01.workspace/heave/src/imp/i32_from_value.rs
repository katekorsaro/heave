use crate::*;

impl From<Value> for i32 {
    fn from(value: Value) -> i32 {
        match value {
            Value::SignedInt(value) => value
                .try_into()
                .map_err(|_| "Type mismatch".to_string())
                .unwrap(),
            _ => panic!("Type mismatch"),
        }
    }
}
