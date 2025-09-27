use crate::*;

impl ToValue for f64 {
    fn to_value(self) -> Value {
        Value::Real(self)
    }
}

// #[cfg(test)]
// mod unit_tests { use super::*; }
