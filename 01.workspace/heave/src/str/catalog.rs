use crate::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct O {
    path: String,
    pub(crate) items: std::collections::HashMap<String, Entity>,
}

impl O {
    /// Creates a new `Catalog` instance.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the SQLite database file.
    ///
    /// # Returns
    ///
    /// A new `Catalog` instance.
    pub fn new(path: &str) -> Self {
        Self {
            path: String::from(path),
            ..O::default()
        }
    }

    /// Initializes the database.
    pub fn init(&self) -> result::Result<(), FailedTo> {
        let path = path::Path::new(&self.path);
        sqlite::init::db(path).map_err(|_| FailedTo::InitDatabase)?;
        Ok(())
    }

    /// Inserts a single object that implements the `EAV` trait into the catalog.
    ///
    /// # Arguments
    ///
    /// * `object` - The object to insert.
    pub fn insert(&mut self, object: impl EAV) {
        let mut entity = object.into();
        entity.state = EntityState::New;
        self.items.insert(entity.id.clone(), entity);
    }

    /// Inserts multiple objects that implement the `EAV` trait into the catalog.
    ///
    /// # Arguments
    ///
    /// * `objects` - A vector of objects to insert.
    pub fn insert_many(&mut self, objects: Vec<impl EAV>) {
        for object in objects {
            self.insert(object);
        }
    }

    /// Retrieves an entity by its ID and converts it into a specified type `T`.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the entity to retrieve.
    ///
    /// # Returns
    ///
    /// An `Option<T>` containing the converted entity if found, otherwise `None`.
    pub fn get<T>(&self, id: &str) -> Option<T>
    where
        T: EAV,
    {
        let entity = self.items.get(id);
        entity.map(|e| T::from(e.clone()))
    }

    /// Retrieves the first entity that matches a given attribute and value.
    ///
    /// # Arguments
    ///
    /// * `attribute` - The attribute of the entity to match.
    /// * `value` - The value of the attribute to match.
    ///
    /// # Returns
    ///
    /// An `Option<T>` containing the converted entity if found, otherwise `None`.
    pub fn get_by_class_and_attribute<T>(
        &self,
        attribute: &str,
        value: impl Into<Value> + Clone,
    ) -> Option<T>
    where
        T: EAV,
    {
        let mut items = self
            .items
            .values()
            .filter(|item| item.class == T::class())
            .filter(|item| item.value_of(attribute) == Some(&value.clone().into()))
            .take(1)
            .map(|item| T::from(item.clone()));
        items.next()
    }

    /// Returns an iterator over entities of a specific class.
    ///
    /// # Returns
    ///
    /// An iterator that yields items of type `T`.
    pub fn list_by_class<T>(&self) -> impl Iterator<Item = T>
    where
        T: EAV,
    {
        self.items
            .values()
            .filter(move |item| item.class == T::class())
            .map(|item| T::from(item.clone()))
    }

    /// Returns an iterator over entities that match a given attribute and value.
    ///
    /// # Arguments
    ///
    /// * `attribute` - The attribute of the entities to match.
    /// * `value` - The value of the attribute to match.
    ///
    /// # Returns
    ///
    /// An iterator that yields items of type `T`.
    pub fn list_by_class_and_attribute<T>(
        &self,
        attribute: &str,
        value: impl Into<Value> + Clone,
    ) -> impl Iterator<Item = T>
    where
        T: EAV,
    {
        let value: Value = value.into();
        self.items
            .values()
            .filter(move |item| item.class == T::class())
            .filter(move |item| item.value_of(attribute) == Some(&value))
            .map(|item| T::from(item.clone()))
    }

    /// Persists the current state of the catalog to the database.
    pub fn persist(&self) {
        let path = path::Path::new(&self.path);
        sqlite::persist::catalog(path, self);
    }

    /// Loads an entity by its ID from the database into the catalog.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the entity to load.
    pub fn load_by_id(&mut self, id: &str) {
        let path = path::Path::new(&self.path);
        let entity = sqlite::load::by_id(path, id);
        match entity {
            None => (),
            Some(entity) => {
                self.items.insert(entity.id.clone(), entity);
            }
        }
    }

    /// Loads all entities of a specific class from the database into the catalog.
    pub fn load_by_class<T>(&mut self)
    where
        T: EAV,
    {
        let class = T::class();
        let path = path::Path::new(&self.path);
        let entities = sqlite::load::by_class(path, class);
        for entity in entities {
            self.items.insert(entity.id.clone(), entity);
        }
    }
}
