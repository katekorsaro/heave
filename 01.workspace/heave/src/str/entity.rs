use crate::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct O {
    pub class: String,
    pub attributes: std::collections::HashMap<String, Attribute>,
}

impl O {
    pub fn new(class: &str) -> Self {
        Self {
            class: String::from(class),
            attributes: std::collections::HashMap::new(),
        }
    }
    pub fn with_attribute(mut self, id: &str, value: Value) -> Self {
        let attribute = Attribute::new(id, value);
        self.attributes.insert(attribute.id.clone(), attribute);
        self
    }
}
