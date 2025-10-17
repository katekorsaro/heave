use crate::*;

impl TryFrom<Value> for u32 {
    type Error = ();
    fn try_from(value: Value) -> Result<u32, Self::Error> {
        match value {
            Value::UnsignedInt(value) => value.try_into().map_err(|_| ()),
            _ => Err(()),
        }
    }
}
