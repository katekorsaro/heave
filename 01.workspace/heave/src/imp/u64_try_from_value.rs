use crate::*;

impl TryFrom<Value> for u64 {
    type Error = ();
    fn try_from(value: Value) -> Result<u64, Self::Error> {
        match value {
            Value::UnsignedInt(value) => Ok(value),
            _ => Err(()),
        }
    }
}
