use crate::*;

impl TryFrom<Value> for u32 {
    type Error = ();
    fn try_from(value: Value) -> Result<u32, Self::Error> {
        match value {
            Value::UnsignedInt(value) => Ok(value),
            _ => Err(()),
        }
    }
}
