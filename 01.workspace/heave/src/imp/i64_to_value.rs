use crate::*;

impl ToValue for i64 {
    fn to_value(self) -> Value {
        Value::SignedInt(self)
    }
}

// #[cfg(test)]
// mod unit_tests { use super::*; }
