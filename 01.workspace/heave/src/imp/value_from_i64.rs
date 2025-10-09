use crate::*;

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Self::SignedInt(value)
    }
}
