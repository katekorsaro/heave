use crate::*;

impl TryFrom<Value> for i64 {
    type Error = ();
    fn try_from(value: Value) -> Result<i64, Self::Error> {
        match value {
            Value::SignedInt(value) => Ok(value),
            _ => Err(()),
        }
    }
}
