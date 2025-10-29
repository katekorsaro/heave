use crate::*;

/// Represents a generic entity with an ID, class, and a set of attributes.
#[derive(Debug, PartialEq, Clone)]
pub struct O {
    /// The unique identifier for this entity.
    pub(crate) id: String,
    /// The current state of the entity within the `Catalog`'s in-memory cache.
    /// This tracks whether the entity is new, modified, or marked for deletion.
    pub(crate) state: EntityState,
    /// An optional timestamp or version number, typically used for optimistic
    /// locking or tracking when the entity was last referenced or modified.
    pub(crate) ref_date: Option<u64>,
    /// A string identifying the "type" or "class" of the entity (e.g., "product", "user").
    /// This is used to group and query entities of the same kind.
    pub(crate) class: String,
    /// A string identifying the "subtype" or "subclass" of the entity (e.g.,
    /// "computer", "phone", "customer"). This is used to group and query entities
    /// of different subtype but of the same kind or inside the same domain (class)
    pub(crate) subclass: Option<String>,
    /// A map of attribute names to `Attribute` values, holding the actual data
    /// of the entity.
    pub(crate) attributes: std::collections::HashMap<String, Attribute>,
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
            subclass: None,
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
    /// Sets the subclass of the entity.
    ///
    /// # Arguments
    ///
    /// * `subclass` - The subclass to set for the entity.
    ///
    /// # Returns
    ///
    /// The entity with the updated subclass.
    pub fn with_subclass(mut self, subclass: &str) -> Self {
        self.subclass = Some(subclass.to_string());
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
    /// Returns a reference to the `Value` of an attribute.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the attribute to get the value of.
    ///
    /// # Returns
    ///
    /// An `Option<&Value>` which is `Some(&Value)` if the attribute exists,
    /// or `None` if it does not.
    pub(crate) fn value_of(&self, id: &str) -> Option<&Value> {
        let attribute = self.attributes.get(id);
        match attribute {
            None => None,
            Some(attribute) => Some(&attribute.value),
        }
    }
    /// Unwraps an attribute's value into a specified type `T`.
    ///
    /// This function requires `T` to implement `TryFrom<Value>`.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the attribute to unwrap.
    ///
    /// # Returns
    ///
    /// A `Result<T, FailedTo>` which is `Ok(T)` if the conversion is successful,
    /// or `Err(FailedTo::ConvertValue)` if it fails.
    pub fn unwrap<T>(&self, id: &str) -> Result<T, FailedTo>
    where
        T: TryFrom<Value>,
    {
        self.value_of(id)
            .map(|value| T::try_from(value.clone()).map_err(|_| FailedTo::ConvertValue))
            .unwrap()
    }
    /// Unwraps an attribute's value into an `Option<T>`.
    ///
    /// This function requires `T` to implement `TryFrom<Value>`.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the attribute to unwrap.
    ///
    /// # Returns
    ///
    /// A `Result<Option<T>, FailedTo>`.
    /// - `Ok(Some(T))` if the attribute exists and the conversion is successful.
    /// - `Ok(None)` if the attribute does not exist.
    /// - `Err(FailedTo::ConvertValue)` if the attribute exists but the conversion fails.
    pub fn unwrap_opt<T>(&self, id: &str) -> Result<Option<T>, FailedTo>
    where
        T: TryFrom<Value>,
    {
        self.value_of(id)
            .map(|value| T::try_from(value.clone()).map_err(|_| FailedTo::ConvertValue))
            .transpose()
    }
    /// Unwraps an attribute's value, returning a default value if it doesn't exist or fails to convert.
    ///
    /// This function requires `T` to implement `TryFrom<Value>`.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the attribute to unwrap.
    /// * `default` - The default value to return if the attribute is not found or conversion fails.
    ///
    /// # Returns
    ///
    /// The converted value of the attribute, or the default value.
    pub fn unwrap_or<T>(&self, id: &str, default: T) -> Result<T, FailedTo>
    where
        T: TryFrom<Value>,
    {
        self.value_of(id)
            .map(|value| T::try_from(value.clone()).map_err(|_| FailedTo::ConvertValue))
            .transpose()
            .map(|value| value.unwrap_or(default))
    }
    /// Returns the ID of the entity.
    pub fn id(&self) -> String {
        self.id.clone()
    }
    /// Returns the subclass of the entity, if it has one.
    pub fn subclass(&self) -> Option<String> {
        self.subclass.clone()
    }
}
