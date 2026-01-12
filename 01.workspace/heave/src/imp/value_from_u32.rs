use crate::*;

impl From<u32> for Value {
    fn from(value: u32) -> Self {
        Self::UnsignedInt(value)
    }
}
