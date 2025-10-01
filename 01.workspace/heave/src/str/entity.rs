use crate::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct O {
    pub id: String,
    pub state: EntityState,
    pub ref_date: Option<u64>,
    pub class: String,
    pub attributes: std::collections::HashMap<String, Attribute>,
}

impl EAV for Entity {}

impl O {
    pub fn new(class: &str) -> Self {
        Self {
            id: short_uuid::short!().to_string(),
            class: String::from(class),
            ..Entity::default()
        }
    }
    pub fn with_id(mut self, id: &str) -> Self {
        self.id = id.to_string();
        self
    }
    pub fn with_class(mut self, class: &str) -> Self {
        self.class = class.to_string();
        self
    }
    pub fn with_ref_date(mut self, ref_date: u64) -> Self {
        self.ref_date = Some(ref_date);
        self
    }
    pub fn with_attribute(mut self, id: &str, value: impl Into<Value>) -> Self {
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
    pub fn set(&mut self, id: &str, value: impl Into<Value>) -> Option<Value> {
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
    pub fn has_attribute(&self, id: &str) -> bool {
        self.attributes.contains_key(id)
    }
    pub fn unwrap<T>(&self, id: &str) -> T
    where
        T: From<Value>,
    {
        self.value_of(id)
            .map(|value| T::from(value.clone()))
            .unwrap()
    }
    pub fn unwrap_opt<T>(&self, id: &str) -> Option<T>
    where
        T: From<Value>,
    {
        self.value_of(id).map(|value| T::from(value.clone()))
    }
    pub fn unwrap_or<T>(&self, id: &str, default: T) -> T
    where
        T: From<Value>,
    {
        self.value_of(id)
            .map(|value| T::from(value.clone()))
            .unwrap_or(default)
    }
}
