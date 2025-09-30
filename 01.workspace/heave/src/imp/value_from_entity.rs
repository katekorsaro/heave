use crate::*;

impl From<Entity> for Value {
    fn from(value: Entity) -> Self {
        Self::Text(value.id)
    }
}

// #[cfg(test)]
// mod unit_tests { use super::*; }
