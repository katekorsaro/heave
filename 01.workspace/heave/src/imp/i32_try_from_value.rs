use crate::*;

impl TryFrom<Value> for i32 {
    type Error = ();
    fn try_from(value: Value) -> Result<i32, Self::Error> {
        match value {
            Value::SignedInt(value) => value.try_into().map_err(|_| ()),
            _ => Err(()),
        }
    }
}
