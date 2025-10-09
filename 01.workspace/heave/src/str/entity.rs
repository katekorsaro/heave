use crate::*;

/// Represents a generic entity with an ID, class, and a set of attributes.
#[derive(Debug, PartialEq, Clone)]
pub struct O {
    pub id: String,
    pub state: EntityState,
    pub ref_date: Option<u64>,
    pub class: String,
    pub attributes: std::collections::HashMap<String, Attribute>,
}

impl Entity {
    /// Creates a new `Entity` instance for a given type `T` that implements `EAV`.
    ///
    /// # Returns
    ///
    /// A new `Entity` instance.
    pub fn new<T>() -> Self
    where
        T: EAV,
    {
        Self {
            id: String::new(),
            state: EntityState::New,
            ref_date: None,
            class: T::class().to_string(),
            attributes: std::collections::HashMap::<String, Attribute>::new(),
        }
    }

    /// Sets the ID of the entity.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID to set for the entity.
    ///
    /// # Returns
    ///
    /// The entity with the updated ID.
    pub fn with_id(mut self, id: &str) -> Self {
        self.id = id.to_string();
        self
    }

    /// Sets the reference date of the entity.
    ///
    /// # Arguments
    ///
    /// * `ref_date` - The reference date to set.
    ///
    /// # Returns
    ///
    /// The entity with the updated reference date.
    pub fn with_ref_date(mut self, ref_date: u64) -> Self {
        self.ref_date = Some(ref_date);
        self
    }

    /// Adds or updates an attribute for the entity.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the attribute.
    /// * `value` - The value of the attribute.
    ///
    /// # Returns
    ///
    /// The entity with the added or updated attribute.
    pub fn with_attribute(mut self, id: &str, value: impl Into<Value>) -> Self {
        let attribute = Attribute::new(id, value);
        self.attributes.insert(attribute.id.clone(), attribute);
        self
    }

    /// Adds or updates an attribute if the value is `Some`.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the attribute.
    /// * `value` - The optional value of the attribute.
    ///
    /// # Returns
    ///
    /// The entity, possibly with the new attribute.
    pub fn with_opt_attribute(mut self, id: &str, value: Option<impl Into<Value>>) -> Self {
        if let Some(value) = value {
            let attribute = Attribute::new(id, value);
            self.attributes.insert(attribute.id.clone(), attribute);
        }
        self
    }

    pub(crate) fn value_of(&self, id: &str) -> Option<&Value> {
        let attribute = self.attributes.get(id);
        match attribute {
            None => None,
            Some(attribute) => Some(&attribute.value),
        }
    }

    /// Unwraps an attribute's value into a specified type `T`.
    ///
    /// # Panics
    ///
    /// Panics if the attribute does not exist.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the attribute to unwrap.
    ///
    /// # Returns
    ///
    /// The value of the attribute converted to type `T`.
    pub fn unwrap<T>(&self, id: &str) -> T
    where
        T: From<Value>,
    {
        self.value_of(id)
            .map(|value| T::from(value.clone()))
            .unwrap()
    }

    /// Unwraps an attribute's value into an `Option<T>`.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the attribute to unwrap.
    ///
    /// # Returns
    ///
    /// An `Option<T>` containing the converted value if the attribute exists, otherwise `None`.
    pub fn unwrap_opt<T>(&self, id: &str) -> Option<T>
    where
        T: From<Value>,
    {
        self.value_of(id).map(|value| T::from(value.clone()))
    }

    /// Unwraps an attribute's value, returning a default if it doesn't exist.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the attribute to unwrap.
    /// * `default` - The default value to return if the attribute is not found.
    ///
    /// # Returns
    ///
    /// The converted value of the attribute or the default value.
    pub fn unwrap_or<T>(&self, id: &str, default: T) -> T
    where
        T: From<Value>,
    {
        self.value_of(id)
            .map(|value| T::from(value.clone()))
            .unwrap_or(default)
    }
}
