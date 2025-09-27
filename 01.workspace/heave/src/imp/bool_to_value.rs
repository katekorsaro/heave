use crate::*;

impl ToValue for bool {
    fn to_value(self) -> Value {
        Value::Bool(self)
    }
}

// #[cfg(test)]
// mod unit_tests { use super::*; }
