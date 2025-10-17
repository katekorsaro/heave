use crate::*;

impl TryFrom<Value> for String {
    type Error = ();
    fn try_from(value: Value) -> Result<String, Self::Error> {
        match value {
            Value::Text(value) => Ok(value),
            _ => Err(()),
        }
    }
}
