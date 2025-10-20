use crate::*;

/// A builder for creating complex queries to load entities from the database.
///
/// A `Filter` consists of one or more conditions that are combined with a logical AND.
/// It is used with `Catalog::load_by_filter` to retrieve entities that match
/// all specified criteria.
#[derive(Debug, Default, PartialEq, Clone)]
pub struct O<'a> {
    class: Option<String>,
    subclass: Option<String>,
    conditions: Vec<(String, Comparison, Condition<'a>)>,
}

impl<'a> Filter<'a> {
    /// Creates a new, empty `Filter`.
    pub fn new() -> Self {
        Self {
            class: None,
            subclass: None,
            conditions: Vec::new(),
        }
    }
    /// Adds a class condition to the filter.
    pub fn with_class(mut self, value: &str) -> Self {
        self.class = Some(value.to_string());
        self
    }
    /// Adds a subclass condition to the filter.
    pub fn with_subclass(mut self, value: &str) -> Self {
        self.subclass = Some(value.to_string());
        self
    }
    /// Adds a boolean condition to the filter.
    ///
    /// This is a shorthand for `with_bool(name, Comparison::Equal, value)`.
    pub fn with_bool(mut self, attribute_name: &str, value: bool) -> Self {
        self.conditions.push((
            attribute_name.to_string(),
            Comparison::Equal,
            Condition::Bool(value),
        ));
        self
    }
    /// Adds a signed integer (`i64`) condition to the filter.
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
    /// Adds an unsigned integer (`u64`) condition to the filter.
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
    /// Adds a text (`&str`) condition to the filter.
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
    /// Adds a real number (`f64`) condition to the filter.
    pub fn with_real(mut self, attribute_name: &str, comparison: Comparison, value: f64) -> Self {
        self.conditions.push((
            attribute_name.to_string(),
            comparison,
            Condition::Real(value),
        ));
        self
    }
    /// Returns an iterator over the conditions in the filter.
    ///
    /// This is used internally by the persistence engine.
    pub(crate) fn conditions(&self) -> impl Iterator<Item = &(String, Comparison, Condition<'a>)> {
        self.conditions.iter()
    }
    pub(crate) fn class(&self) -> &Option<String> {
        &self.class
    }
    pub(crate) fn subclass(&self) -> &Option<String> {
        &self.subclass
    }
}
