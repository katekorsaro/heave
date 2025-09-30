use crate::*;

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

// #[cfg(test)]
// mod unit_tests { use super::*; }
