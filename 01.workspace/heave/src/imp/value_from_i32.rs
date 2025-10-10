use crate::*;

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Self::SignedInt(value.into())
    }
}
