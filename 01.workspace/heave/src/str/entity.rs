use crate::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct O {
    pub id: String,
    pub class: String,
    pub attributes: std::collections::HashMap<String, Attribute>,
}

impl O {
    pub fn new(class: &str) -> Self {
        Self {
            id: short_uuid::short!().to_string(),
            class: String::from(class),
            attributes: std::collections::HashMap::new(),
        }
    }
    pub fn with_attribute(mut self, id: &str, value: impl ToValue) -> Self {
        let attribute = Attribute::new(id, value);
        self.attributes.insert(attribute.id.clone(), attribute);
        self
    }
    pub fn value_of(&self, id: &str) -> Option<&Value> {
        let attribute = self.attributes.get(id);
        match attribute {
            None => None,
            Some(attribute) => Some(&attribute.value),
        }
    }
    pub fn set(&mut self, id: &str, value: impl ToValue) -> Option<Value> {
        let attribute = Attribute::new(id, value);
        let previous_attribute = self.attributes.insert(attribute.id.clone(), attribute);
        match previous_attribute {
            None => None,
            Some(attribute) => Some(attribute.value),
        }
    }
    pub fn unset(&mut self, id: &str) -> Option<Value> {
        let attribute = self.attributes.remove(id);
        match attribute {
            None => None,
            Some(attribute) => Some(attribute.value),
        }
    }
    pub fn unwrap<T>(&self, id: &str) -> T
    where
        T: From<Value>,
    {
        let value = self.value_of(id).unwrap();
        T::from(value.clone())
    }
}
