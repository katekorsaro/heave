use crate::*;

impl ToValue for String {
    fn to_value(self) -> Value {
        Value::Text(self)
    }
}

// #[cfg(test)]
// mod unit_tests { use super::*; }
