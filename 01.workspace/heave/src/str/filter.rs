use crate::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct O {
    conditions: Vec<(String, Condition)>,
}

impl Filter {
    pub fn new() -> Self {
        Self {
            conditions: Vec::new(),
        }
    }
    pub fn with_bool(mut self, attribute_name: &str, value: bool) -> Self {
        self.conditions
            .push((attribute_name.to_string(), Condition::Bool(value)));
        self
    }
    pub(crate) fn conditions(&self) -> impl Iterator<Item = &(String, Condition)> {
        self.conditions.iter()
    }
}
