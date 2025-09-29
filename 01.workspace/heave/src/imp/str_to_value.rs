use crate::*;

impl ToValue for &str {
    fn to_value(self) -> Value {
        Value::Text(String::from(self))
    }
}

// #[cfg(test)]
// mod unit_tests { use super::*; }
