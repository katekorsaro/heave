use crate::*;

impl From<Value> for Entity {
    fn from(value: Value) -> Entity {
        let id = match value {
            Value::Text(value) => value,
            _ => panic!("Type mismatch"),
        };
        Entity {
            id,
            ref_date: None,
            state: EntityState::Unknown,
            class: String::new(),
            attributes: std::collections::HashMap::new(),
        }
    }
}

// #[cfg(test)]
// mod unit_tests { use super::*; }
