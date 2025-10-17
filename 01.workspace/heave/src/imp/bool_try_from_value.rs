use crate::*;

impl TryFrom<Value> for bool {
    type Error = ();
    fn try_from(value: Value) -> Result<bool, Self::Error> {
        match value {
            Value::Bool(value) => Ok(value),
            _ => Err(()),
        }
    }
}
