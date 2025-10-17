use crate::*;

impl TryFrom<Value> for f64 {
    type Error = ();
    fn try_from(value: Value) -> Result<f64, Self::Error> {
        match value {
            Value::Real(value) => Ok(value),
            _ => Err(()),
        }
    }
}
