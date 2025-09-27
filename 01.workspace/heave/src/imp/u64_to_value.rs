use crate::*;

impl ToValue for u64 {
    fn to_value(self) -> Value {
        Value::UnsignedInt(self)
    }
}

// #[cfg(test)]
// mod unit_tests { use super::*; }
