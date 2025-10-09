use crate::*;

impl From<u64> for Value {
    fn from(value: u64) -> Self {
        Self::UnsignedInt(value)
    }
}
