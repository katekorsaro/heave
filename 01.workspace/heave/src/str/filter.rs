use crate::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct O<'a> {
    conditions: Vec<(String, Comparison, Condition<'a>)>,
}

impl<'a> Filter<'a> {
    pub fn new() -> Self {
        Self {
            conditions: Vec::new(),
        }
    }
    pub fn with_bool(mut self, attribute_name: &str, value: bool) -> Self {
        self.conditions.push((
            attribute_name.to_string(),
            Comparison::Equal,
            Condition::Bool(value),
        ));
        self
    }
    pub fn with_signed_int(
        mut self,
        attribute_name: &str,
        comparison: Comparison,
        value: i64,
    ) -> Self {
        self.conditions.push((
            attribute_name.to_string(),
            comparison,
            Condition::SignedInt(value),
        ));
        self
    }
    pub fn with_unsigned_int(
        mut self,
        attribute_name: &str,
        comparison: Comparison,
        value: i64,
    ) -> Self {
        self.conditions.push((
            attribute_name.to_string(),
            comparison,
            Condition::UnsignedInt(value),
        ));
        self
    }
    pub fn with_text(
        mut self,
        attribute_name: &str,
        comparison: Comparison,
        value: &'a str,
    ) -> Self {
        self.conditions.push((
            attribute_name.to_string(),
            comparison,
            Condition::Text(value),
        ));
        self
    }
    pub fn with_real(mut self, attribute_name: &str, comparison: Comparison, value: f64) -> Self {
        self.conditions.push((
            attribute_name.to_string(),
            comparison,
            Condition::Real(value),
        ));
        self
    }
    pub(crate) fn conditions(&self) -> impl Iterator<Item = &(String, Comparison, Condition<'a>)> {
        self.conditions.iter()
    }
}
