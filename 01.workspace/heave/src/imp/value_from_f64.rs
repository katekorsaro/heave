use crate::*;

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::Real(value)
    }
}

// #[cfg(test)]
// mod unit_tests { use super::*; }
