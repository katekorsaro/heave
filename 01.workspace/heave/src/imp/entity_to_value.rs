use crate::*;

impl ToValue for Entity {
    fn to_value(self) -> Value {
        self.id.to_value()
    }
}

// #[cfg(test)]
// mod unit_tests { use super::*; }
