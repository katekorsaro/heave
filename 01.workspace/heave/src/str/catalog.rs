use crate::*;
use std::collections::HashMap;

/// Represents a catalog of entities that can be persisted to a SQLite database.
///
/// The `Catalog` holds entities in memory and provides methods to interact with
/// them, as well as to persist changes to and load data from a database file.
#[derive(Debug, Default, PartialEq, Clone)]
pub struct O {
    path: String,
    pub(crate) items: HashMap<String, Entity>,
}
impl Catalog {
    /// Creates a new, empty in-memory `Catalog` for the database at the given path.
    ///
    /// This method does not create the database file or connect to it. It only
    /// initializes an empty catalog in memory. The database file will be accessed
    /// when `init()`, `persist()`, or `load_*` methods are called.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the SQLite database file that this catalog will manage.
    ///
    /// # Returns
    ///
    /// A new `Catalog` instance with an empty in-memory item cache.
    pub fn new(path: &str) -> Self {
        Self {
            path: String::from(path),
            ..O::default()
        }
    }
    /// Initializes the database by creating the file and schema if they don't exist.
    ///
    /// This method interacts with the filesystem to ensure the database file and its
    /// underlying tables are ready. It has no effect on the in-memory state of the
    /// catalog.
    ///
    /// # Effects
    ///
    /// - **Database State:** Creates the SQLite file and required tables if they are not
    ///   present. If the file already exists, it does nothing.
    /// - **In-Memory State:** This method does not alter the in-memory entity cache.
    pub fn init(&self) -> result::Result<(), FailedTo> {
        let path = path::Path::new(&self.path);
        sqlite::init::db(path).map_err(|_| FailedTo::InitDatabase)?;
        Ok(())
    }
    /// Inserts or updates an object in the in-memory catalog.
    ///
    /// This is a purely in-memory operation. The change will not be written to the
    /// database until `persist()` is called.
    ///
    /// # Effects
    ///
    /// - **In-Memory State:**
    ///   - If the entity does not exist in the catalog, it is added with the state
    ///     `EntityState::New`.
    ///   - If the entity already exists, it is overwritten and its state is set to
    ///     `EntityState::Updated`.
    /// - **Database State:** Unchanged. Call `persist()` to save the changes.
    ///
    /// # Arguments
    ///
    /// * `object` - The object to insert or update, which must implement the `EAV` trait.
    pub fn upsert(&mut self, object: impl EAV) -> Result<(), FailedTo> {
        let mut entity = object.try_into().map_err(|_| FailedTo::ConvertObject)?;
        if self.items.contains_key(&entity.id) {
            entity.state = EntityState::Updated;
        } else {
            entity.state = EntityState::New;
        }
        self.items.insert(entity.id.clone(), entity);
        Ok(())
    }
    /// Inserts or updates multiple objects in the in-memory catalog.
    ///
    /// This method calls `upsert()` for each object in the provided vector. Like
    /// `upsert()`, this is a purely in-memory operation.
    ///
    /// # Effects
    ///
    /// - **In-Memory State:** Adds or updates multiple entities in the cache.
    /// - **Database State:** Unchanged. Call `persist()` to save the changes.
    ///
    /// # Arguments
    ///
    /// * `objects` - A vector of objects to insert or update.
    pub fn insert_many(&mut self, objects: Vec<impl EAV>) -> Result<(), FailedTo> {
        for object in objects {
            self.upsert(object)?;
        }
        Ok(())
    }
    /// Retrieves an entity by its ID from the in-memory catalog.
    ///
    /// This is a purely in-memory operation and does not interact with the database.
    /// It will only find entities that have been loaded into or created in the catalog.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the entity to retrieve.
    ///
    /// # Returns
    ///
    /// An `Option<T>` containing the converted entity if found in the in-memory
    /// cache, otherwise `None`.
    pub fn get<T>(&self, id: &str) -> Result<Option<T>, FailedTo>
    where
        T: EAV,
    {
        let entity = self.items.get(id);
        entity
            .map(|e| T::try_from(e.clone()).map_err(|_| FailedTo::ConvertEntity))
            .transpose()
    }
    /// Retrieves the first entity from the in-memory catalog that matches a given
    /// class, attribute, and value.
    ///
    /// This is a purely in-memory operation and does not interact with the database.
    ///
    /// # Arguments
    ///
    /// * `attribute` - The attribute of the entity to match.
    /// * `value` - The value of the attribute to match.
    ///
    /// # Returns
    ///
    /// An `Option<T>` containing the first matching converted entity if found,
    /// otherwise `None`.
    pub fn get_by_class_and_attribute<T>(
        &self,
        attribute: &str,
        value: impl Into<Value> + Clone,
    ) -> Result<Option<T>, FailedTo>
    where
        T: EAV,
    {
        let mut items = self
            .items
            .values()
            .filter(|item| item.class == T::class())
            .filter(|item| item.value_of(attribute) == Some(&value.clone().into()))
            .take(1)
            .map(|item| T::try_from(item.clone()).map_err(|_| FailedTo::ConvertEntity));
        items.next().transpose()
    }
    /// Returns an iterator over all entities of a specific class in the in-memory catalog.
    ///
    /// This is a purely in-memory operation and does not interact with the database.
    ///
    /// # Returns
    ///
    /// An iterator that yields items of type `T` from the in-memory cache.
    pub fn list_by_class<T>(&self) -> impl Iterator<Item = Result<T, FailedTo>>
    where
        T: EAV,
    {
        self.items
            .values()
            .filter(move |item| item.class == T::class())
            .map(|item| T::try_from(item.clone()).map_err(|_| FailedTo::ConvertEntity))
    }
    /// Returns an iterator over all entities of a specific class and subclass
    /// in the in-memory catalog.
    ///
    /// This is a purely in-memory operation and does not interact with the database.
    ///
    /// # Returns
    ///
    /// An iterator that yields items of type `T` from the in-memory cache.
    pub fn list_by_class_and_subclass<T>(&self, subclass: &str) -> impl Iterator<Item = Result<T, FailedTo>>
    where
        T: EAV,
    {
        self.items
            .values()
            .filter(move |item| item.class == T::class())
            .filter(move |item| item.subclass == Some(subclass.to_string()))
            .map(|item| T::try_from(item.clone()).map_err(|_| FailedTo::ConvertEntity))
    }
    /// Returns an iterator over entities in the in-memory catalog that match a given
    /// class, attribute, and value.
    ///
    /// This is a purely in-memory operation and does not interact with the database.
    ///
    /// # Arguments
    ///
    /// * `attribute` - The attribute of the entities to match.
    /// * `value` - The value of the attribute to match.
    ///
    /// # Returns
    ///
    /// An iterator that yields matching items of type `T` from the in-memory cache.
    pub fn list_by_class_and_attribute<T>(
        &self,
        attribute: &str,
        value: impl Into<Value> + Clone,
    ) -> impl Iterator<Item = Result<T, FailedTo>>
    where
        T: EAV,
    {
        let value: Value = value.into();
        self.items
            .values()
            .filter(move |item| item.class == T::class())
            .filter(move |item| item.value_of(attribute) == Some(&value))
            .map(|item| T::try_from(item.clone()).map_err(|_| FailedTo::ConvertEntity))
    }
    /// Marks an entity for deletion from the in-memory catalog.
    ///
    /// This is a purely in-memory operation. The entity will not be removed from the
    /// database until `persist()` is called.
    ///
    /// # Effects
    ///
    /// - **In-Memory State:** If the entity exists, its state is set to
    ///   `EntityState::ToDelete`. If it was in a `New` state, it will simply be
    ///   forgotten upon the next `persist()` call without ever touching the database.
    /// - **Database State:** Unchanged. Call `persist()` to apply the deletion.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the entity to mark for deletion.
    pub fn delete(&mut self, id: &str) {
        let entity = self.items.get_mut(id);
        if let Some(entity) = entity {
            entity.state = EntityState::ToDelete;
        }
    }
    /// Persists all in-memory changes to the database and updates the in-memory state.
    ///
    /// This method synchronizes the state of the in-memory catalog with the database
    /// by writing all pending changes (new, updated, and deleted entities).
    ///
    /// # Effects
    ///
    /// - **Database State:**
    ///   - Entities marked as `EntityState::New` are inserted into the database.
    ///   - Entities marked as `EntityState::Updated` are updated in the database.
    ///   - Entities marked as `EntityState::ToDelete` are deleted from the database.
    ///
    /// - **In-Memory State:**
    ///   - After a successful database write, entities marked `ToDelete` are permanently
    ///     removed from the in-memory catalog.
    ///   - All other entities that were successfully persisted (new or updated) have
    ///     their state changed to `EntityState::Loaded`.
    pub fn persist(&mut self) -> result::Result<(), FailedTo> {
        let path = path::Path::new(&self.path);
        sqlite::persist::catalog(path, self).map_err(|_| FailedTo::PersistCatalog)?;
        // cleaning catalog state after db write
        self.items = self
            .items
            .extract_if(|_, item| item.state != EntityState::ToDelete)
            .map(|(k, item)| {
                (
                    k,
                    Entity {
                        state: EntityState::Loaded,
                        ..item
                    },
                )
            })
            .collect();
        Ok(())
    }
    /// Loads a single entity by its ID from the database into the in-memory catalog.
    ///
    /// This method fetches the entity from the database and updates the in-memory
    /// catalog. If an entity with the same ID already exists in memory, it will be
    /// **overwritten** with the version from the database.
    ///
    /// # Effects
    ///
    /// - **In-Memory State:**
    ///   - If an entity is found in the database, it is inserted or updated in the
    ///     in-memory cache with the state `EntityState::Loaded`.
    ///   - Any existing in-memory entity with the same ID, regardless of its state
    ///     (`New`, `Updated`), will be replaced.
    ///   - If no entity is found in the database, the in-memory cache is not modified.
    /// - **Database State:** Unchanged.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the entity to load.
    pub fn load_by_id(&mut self, id: &str) -> Result<(), FailedTo> {
        let path = path::Path::new(&self.path);
        let entity = sqlite::load::by_id(path, id).map_err(|_| FailedTo::LoadFromDB)?;
        if let Some(entity) = entity {
            self.items.insert(entity.id.clone(), entity);
        }
        Ok(())
    }
    /// Loads all entities of a specific class from the database into the in-memory catalog.
    ///
    /// This method fetches all entities matching the given class from the database.
    /// If any of the loaded entities have IDs that match entities already in the
    /// in-memory catalog, the in-memory versions will be **overwritten**.
    ///
    /// # Effects
    ///
    /// - **In-Memory State:**
    ///   - All found entities are inserted or updated in the in-memory cache with the
    ///     state `EntityState::Loaded`.
    ///   - Any existing in-memory entities with matching IDs are replaced.
    /// - **Database State:** Unchanged.
    pub fn load_by_class<T>(&mut self) -> Result<(), FailedTo>
    where
        T: EAV,
    {
        let class = T::class();
        let path = path::Path::new(&self.path);
        let entities = sqlite::load::by_class(path, class).map_err(|_| FailedTo::LoadFromDB)?;
        for entity in entities {
            self.items.insert(entity.id.clone(), entity);
        }
        Ok(())
    }
    /// Loads entities from the database that match a given `Filter`.
    ///
    /// This method queries the database for all entities that satisfy the conditions
    /// specified in the filter. If any of the loaded entities have IDs that match
    /// entities already in the in-memory catalog, the in-memory versions will be
    /// **overwritten**.
    ///
    /// **Warning:** The filter is applied at the attribute level. If different entity
    /// classes share attribute names, this method may load entities of multiple
    /// classes.
    ///
    /// # Effects
    ///
    /// - **In-Memory State:**
    ///   - All found entities are inserted or updated in the in-memory cache with the
    ///     state `EntityState::Loaded`.
    ///   - Any existing in-memory entities with matching IDs are replaced.
    /// - **Database State:** Unchanged.
    pub fn load_by_filter(&mut self, filter: &Filter) -> Result<(), FailedTo> {
        let path = path::Path::new(&self.path);
        let entities = sqlite::load::by_filter(path, filter).map_err(|_| FailedTo::LoadFromDB)?;
        for entity in entities {
            self.items.insert(entity.id.clone(), entity);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Debug, Default, PartialEq, Clone)]
    struct Item {
        pub id: String,
        pub name: String,
        pub price: u64,
        pub discount: f64,
        pub sell_trend: i64,
        pub in_stock: bool,
    }
    impl EAV for Item {
        fn class() -> &'static str {
            "item"
        }
    }
    impl From<Item> for Entity {
        fn from(value: Item) -> Entity {
            Entity::new::<Item>()
                .with_id(&value.id)
                .with_attribute("name", value.name)
                .with_attribute("price", value.price)
                .with_attribute("discount", value.discount)
                .with_attribute("sell_trend", value.sell_trend)
                .with_attribute("in_stock", value.in_stock)
        }
    }
    impl From<Entity> for Item {
        fn from(entity: Entity) -> Self {
            Self {
                id: entity.id.clone(),
                name: entity.unwrap("name").expect("name is always present"),
                price: entity.unwrap("price").expect("price is always present"),
                discount: entity
                    .unwrap("discount")
                    .expect("discount is always present"),
                sell_trend: entity
                    .unwrap("sell_trend")
                    .expect("sell_trend is always present"),
                in_stock: entity
                    .unwrap("in_stock")
                    .expect("in_stock is always present"),
            }
        }
    }
    // ## 'new()'
    #[test]
    fn new_should_create_catalog_with_path_and_empty_items() {
        // Should create a new Catalog with the given path and an empty 'items' map.
        let path = "test.db";
        let catalog = Catalog::new(path);
        assert_eq!(catalog.path, path);
        assert!(catalog.items.is_empty());
    }
    // ## 'init()'
    #[test]
    fn init_should_create_db_file_if_not_exists() {
        // Should create the SQLite database file if it doesn't exist.
        let db_path = "target/test_dbs/init_should_create_db_file_if_not_exists.db";
        let path = std::path::Path::new(db_path);
        // Ensure the directory exists
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        // Ensure the file does not exist before the test
        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
        let catalog = Catalog::new(db_path);
        let result = catalog.init();
        assert!(result.is_ok());
        assert!(path.exists());
        // Clean up the created file
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn init_should_not_fail_if_db_file_exists() {
        // Should not fail if the database file already exists.
        let db_path = "target/test_dbs/init_should_not_fail_if_db_file_exists.db";
        let path = std::path::Path::new(db_path);
        // Ensure the directory exists
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        // Create the DB file first
        let catalog = Catalog::new(db_path);
        catalog.init().unwrap();
        // Calling init() again should not fail
        let result = catalog.init();
        assert!(result.is_ok());
        assert!(path.exists());
        // Clean up
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn init_should_return_error_for_invalid_path() {
        // Should return an error for an invalid path or permissions issue.
        // Using a directory as a path should fail.
        let invalid_path = "target/test_dbs/an_invalid_path_dir";
        std::fs::create_dir_all(invalid_path).unwrap();
        let catalog = Catalog::new(invalid_path);
        let result = catalog.init();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), FailedTo::InitDatabase);
        // Clean up
        std::fs::remove_dir_all(invalid_path).unwrap();
    }
    // ## 'upsert()' & 'insert_many()'
    #[test]
    fn insert_should_add_single_entity_as_new() {
        // 'upsert()': Should add a single entity to the 'items' map with 'EntityState::New'.
        let mut catalog = Catalog::new("dummy.db");
        let item = Item {
            id: "item-123".to_string(),
            name: "Test Item".to_string(),
            price: 100,
            sell_trend: 0,
            in_stock: true,
            ..Item::default()
        };
        let item_id = item.id.clone();
        let _ = catalog.upsert(item);
        let entity = catalog.items.get(&item_id).unwrap();
        assert_eq!(entity.id, item_id);
        assert_eq!(entity.state, EntityState::New);
        assert_eq!(entity.class, "item");
        assert_eq!(entity.value_of("name"), Some(&Value::from("Test Item")));
        assert_eq!(entity.value_of("price"), Some(&Value::from(100u64)));
        assert_eq!(entity.value_of("sell_trend"), Some(&Value::from(0i64)));
        assert_eq!(entity.value_of("in_stock"), Some(&Value::from(true)));
    }
    #[test]
    fn insert_should_overwrite_existing_entity() {
        // 'upsert()': Should overwrite an existing entity with the same ID.
        let mut catalog = Catalog::new("dummy.db");
        let item1 = Item {
            id: "item-123".to_string(),
            name: "First Item".to_string(),
            price: 100,
            sell_trend: 0,
            in_stock: true,
            ..Item::default()
        };
        let item_id = item1.id.clone();
        let _ = catalog.upsert(item1);
        let item2 = Item {
            id: "item-123".to_string(),
            name: "Second Item".to_string(),
            price: 200,
            sell_trend: 10,
            in_stock: false,
            ..Item::default()
        };
        let _ = catalog.upsert(item2);
        assert_eq!(catalog.items.len(), 1);
        let entity = catalog.items.get(&item_id).unwrap();
        assert_eq!(entity.value_of("name"), Some(&Value::from("Second Item")));
        assert_eq!(entity.value_of("price"), Some(&Value::from(200u64)));
        assert_eq!(entity.value_of("sell_trend"), Some(&Value::from(10i64)));
        assert_eq!(entity.value_of("in_stock"), Some(&Value::from(false)));
        assert_eq!(entity.state, EntityState::Updated);
    }
    #[test]
    fn insert_many_should_add_all_entities() {
        // 'insert_many()': Should add all provided entities to the 'items' map.
        let mut catalog = Catalog::new("dummy.db");
        let items = vec![
            Item {
                id: "item-1".to_string(),
                name: "Item 1".to_string(),
                price: 10,
                sell_trend: 0,
                in_stock: true,
                ..Item::default()
            },
            Item {
                id: "item-2".to_string(),
                name: "Item 2".to_string(),
                price: 20,
                sell_trend: 0,
                in_stock: false,
                ..Item::default()
            },
        ];
        let _ = catalog.insert_many(items);
        assert_eq!(catalog.items.len(), 2);
        let entity1 = catalog.items.get("item-1").unwrap();
        assert_eq!(entity1.state, EntityState::New);
        assert_eq!(entity1.value_of("name"), Some(&Value::from("Item 1")));
        assert_eq!(entity1.value_of("price"), Some(&Value::from(10u64)));
        assert_eq!(entity1.value_of("sell_trend"), Some(&Value::from(0i64)));
        let entity2 = catalog.items.get("item-2").unwrap();
        assert_eq!(entity2.state, EntityState::New);
        assert_eq!(entity2.value_of("name"), Some(&Value::from("Item 2")));
        assert_eq!(entity2.value_of("price"), Some(&Value::from(20u64)));
        assert_eq!(entity2.value_of("sell_trend"), Some(&Value::from(0i64)));
    }
    // ## 'get()'
    #[test]
    fn get_should_retrieve_and_convert_entity_by_id() {
        // Should retrieve an entity by its ID and correctly convert it to the target type 'T'.
        let mut catalog = Catalog::new("dummy.db");
        let item = Item {
            id: "item-123".to_string(),
            name: "Test Item".to_string(),
            price: 100,
            sell_trend: 0,
            in_stock: true,
            ..Item::default()
        };
        let _ = catalog.upsert(item.clone());
        let retrieved_item: Option<Item> = catalog.get::<Item>("item-123").unwrap();
        assert_eq!(retrieved_item, Some(item));
    }
    #[test]
    fn get_should_return_none_for_nonexistent_id() {
        // Should return 'None' if the ID does not exist.
        let mut catalog = Catalog::new("dummy.db");
        let item = Item {
            id: "item-123".to_string(),
            name: "Test Item".to_string(),
            price: 100,
            sell_trend: 0,
            in_stock: true,
            ..Item::default()
        };
        let _ = catalog.upsert(item.clone());
        let retrieved_item: Option<Item> = catalog.get("nonexistent-id").unwrap();
        assert!(retrieved_item.is_none());
    }
    // ## 'get_by_class_and_attribute()'
    #[test]
    fn get_by_class_and_attribute_should_retrieve_correct_entity() {
        // Should retrieve the first entity matching the class, attribute, and value.
        let mut catalog = Catalog::new("dummy.db");
        let item1 = Item {
            id: "item-1".to_string(),
            name: "Test Item".to_string(),
            price: 100,
            sell_trend: 0,
            in_stock: true,
            ..Item::default()
        };
        let item2 = Item {
            id: "item-2".to_string(),
            name: "Unique Item".to_string(),
            price: 200,
            sell_trend: 0,
            in_stock: false,
            ..Item::default()
        };
        let _ = catalog.upsert(item1.clone());
        let _ = catalog.upsert(item2.clone());
        let retrieved_item: Option<Item> = catalog
            .get_by_class_and_attribute("name", "Unique Item")
            .unwrap();
        assert_eq!(retrieved_item, Some(item2));
    }
    #[test]
    fn get_by_class_and_attribute_should_work_with_different_value_types() {
        // Should work with different value types (String, u64, bool).
        let mut catalog = Catalog::new("dummy.db");
        let item1 = Item {
            id: "item-1".to_string(),
            name: "Item One".to_string(),
            price: 100,
            sell_trend: 10,
            in_stock: true,
            ..Item::default()
        };
        let item2 = Item {
            id: "item-2".to_string(),
            name: "Item Two".to_string(),
            price: 250,
            sell_trend: 20,
            in_stock: false,
            ..Item::default()
        };
        let _ = catalog.upsert(item1.clone());
        let _ = catalog.upsert(item2.clone());
        // Test with &str for String attribute
        let retrieved_by_name: Option<Item> = catalog
            .get_by_class_and_attribute("name", "Item One")
            .unwrap();
        assert_eq!(retrieved_by_name, Some(item1.clone()));
        // Test with u64 for price attribute
        let retrieved_by_price: Option<Item> =
            catalog.get_by_class_and_attribute("price", 250u64).unwrap();
        assert_eq!(retrieved_by_price, Some(item2.clone()));
        // Test with bool for in_stock attribute
        let retrieved_by_stock: Option<Item> = catalog
            .get_by_class_and_attribute("in_stock", true)
            .unwrap();
        assert_eq!(retrieved_by_stock, Some(item1.clone()));
        // Test with i64 for sell_trend attribute
        let retrieved_by_sell_trend: Option<Item> = catalog
            .get_by_class_and_attribute("sell_trend", 10i64)
            .unwrap();
        assert_eq!(retrieved_by_sell_trend, Some(item1.clone()));
    }
    #[test]
    fn get_by_class_and_attribute_should_return_none_if_no_match() {
        // Should return 'None' if no entity matches the criteria.
        let mut catalog = Catalog::new("dummy.db");
        let item = Item {
            id: "item-1".to_string(),
            name: "Test Item".to_string(),
            price: 100,
            sell_trend: 0,
            in_stock: true,
            ..Item::default()
        };
        let _ = catalog.upsert(item.clone());
        // Test with a value that doesn't exist
        let retrieved_item: Option<Item> = catalog
            .get_by_class_and_attribute("name", "Non-existent Name")
            .unwrap();
        assert!(retrieved_item.is_none());
        // Test with an attribute that doesn't exist
        let retrieved_item_2: Option<Item> = catalog
            .get_by_class_and_attribute("non-existent-attribute", "Test Item")
            .unwrap();
        assert!(retrieved_item_2.is_none());
    }
    // ## 'list_by_class()'
    #[test]
    fn list_by_class_should_return_all_entities_of_class() {
        // Should return an iterator with all entities of a specific class.
        let mut catalog = Catalog::new("dummy.db");
        let item1 = Item {
            id: "item-1".to_string(),
            name: "Item One".to_string(),
            price: 100,
            sell_trend: 0,
            in_stock: true,
            ..Item::default()
        };
        let item2 = Item {
            id: "item-2".to_string(),
            name: "Item Two".to_string(),
            price: 200,
            sell_trend: 0,
            in_stock: false,
            ..Item::default()
        };
        let _ = catalog.upsert(item1.clone());
        let _ = catalog.upsert(item2.clone());
        let results: Vec<Item> = catalog
            .list_by_class::<Item>()
            .map(|item| item.unwrap())
            .collect();
        assert_eq!(results.len(), 2);
        assert!(results.contains(&item1));
        assert!(results.contains(&item2));
    }
    #[test]
    fn list_by_class_should_return_empty_iterator_if_no_match() {
        // Should return an empty iterator if no entities of that class exist.
        let catalog = Catalog::new("dummy.db");
        let results: Vec<Item> = catalog
            .list_by_class::<Item>()
            .map(|item| item.unwrap())
            .collect();
        assert!(results.is_empty());
    }
    // ## 'list_by_class_and_attribute()'
    #[test]
    fn list_by_class_and_attribute_should_return_all_matching_entities() {
        // Should return an iterator with all entities matching the class, attribute, and value.
        let mut catalog = Catalog::new("dummy.db");
        let item1 = Item {
            id: "item-1".to_string(),
            name: "Item One".to_string(),
            price: 100,
            sell_trend: 0,
            in_stock: true,
            ..Item::default()
        };
        let item2 = Item {
            id: "item-2".to_string(),
            name: "Item Two".to_string(),
            price: 200,
            sell_trend: 0,
            in_stock: false,
            ..Item::default()
        };
        let item3 = Item {
            id: "item-3".to_string(),
            name: "Item Three".to_string(),
            price: 300,
            sell_trend: 0,
            in_stock: true,
            ..Item::default()
        };
        let _ = catalog.upsert(item1.clone());
        let _ = catalog.upsert(item2.clone());
        let _ = catalog.upsert(item3.clone());
        let results: Vec<Item> = catalog
            .list_by_class_and_attribute("in_stock", true)
            .map(|item| item.unwrap())
            .collect();
        assert_eq!(results.len(), 2);
        assert!(results.contains(&item1));
        assert!(results.contains(&item3));
        assert!(!results.contains(&item2));
    }
    #[test]
    fn list_by_class_and_attribute_should_return_empty_iterator_if_no_match() {
        // Should return an empty iterator if no entities match.
        let mut catalog = Catalog::new("dummy.db");
        let item = Item {
            id: "item-1".to_string(),
            name: "Test Item".to_string(),
            price: 100,
            sell_trend: 0,
            in_stock: true,
            ..Item::default()
        };
        let _ = catalog.upsert(item);
        // Search for a value that doesn't exist
        let results: Vec<Item> = catalog
            .list_by_class_and_attribute("in_stock", false)
            .map(|item| item.unwrap())
            .collect();
        assert!(results.is_empty());
        // Search for an attribute that doesn't exist
        let results_2: Vec<Item> = catalog
            .list_by_class_and_attribute("non-existent-attribute", true)
            .map(|item| item.unwrap())
            .collect();
        assert!(results_2.is_empty());
        // Search in a completely empty catalog
        let empty_catalog = Catalog::new("dummy.db");
        let results_3: Vec<Item> = empty_catalog
            .list_by_class_and_attribute("any_attribute", "any_value")
            .map(|item| item.unwrap())
            .collect();
        assert!(results_3.is_empty());
    }
    // ## 'delete()'
    #[test]
    fn delete_should_mark_entity_as_to_delete() {
        // Should mark an existing entity's state as 'ToDelete'.
        let mut catalog = Catalog::new("dummy.db");
        let item = Item {
            id: "item-123".to_string(),
            name: "Test Item".to_string(),
            price: 100,
            sell_trend: 0,
            in_stock: true,
            ..Item::default()
        };
        let item_id = item.id.clone();
        let _ = catalog.upsert(item);
        catalog.delete(&item_id);
        let entity = catalog.items.get(&item_id).unwrap();
        assert_eq!(entity.state, EntityState::ToDelete);
    }
    #[test]
    fn delete_should_have_no_effect_for_nonexistent_id() {
        // Should have no effect if the entity ID does not exist.
        let mut catalog = Catalog::new("dummy.db");
        let item = Item {
            id: "item-123".to_string(),
            name: "Test Item".to_string(),
            price: 100,
            sell_trend: 0,
            in_stock: true,
            ..Item::default()
        };
        let _ = catalog.upsert(item);
        let original_items = catalog.items.clone();
        // Attempt to delete a non-existent entity, which should not panic or change anything.
        catalog.delete("nonexistent-id");
        assert_eq!(catalog.items, original_items);
    }
    // ## 'persist()'
    #[test]
    fn persist_should_insert_new_entities() {
        // Should insert entities with 'EntityState::New' into the database.
        let db_path = "target/test_dbs/persist_should_insert_new_entities.db";
        let path = std::path::Path::new(db_path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
        // 1. Create catalog, insert an item, and persist
        let mut catalog1 = Catalog::new(db_path);
        catalog1.init().unwrap();
        let item1 = Item {
            id: "item-1".to_string(),
            name: "Test Item".to_string(),
            price: 123,
            sell_trend: 0,
            in_stock: true,
            ..Item::default()
        };
        let _ = catalog1.upsert(item1.clone());
        assert!(catalog1.persist().is_ok());
        // 2. Create a new catalog and load the item to verify it was persisted
        let mut catalog2 = Catalog::new(db_path);
        assert!(catalog2.load_by_id("item-1").is_ok());
        // 3. Get the item and assert it's the same as the one we inserted
        let loaded_item: Option<Item> = catalog2.get("item-1").unwrap();
        assert_eq!(loaded_item, Some(item1));
        // Clean up
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn persist_should_delete_to_delete_entities() {
        // Should delete entities with 'EntityState::ToDelete' from the database.
        let db_path = "target/test_dbs/persist_should_delete_to_delete_entities.db";
        let path = std::path::Path::new(db_path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
        // 1. Create catalog, insert an item, and persist it.
        let mut catalog1 = Catalog::new(db_path);
        catalog1.init().unwrap();
        let item1 = Item {
            id: "item-to-delete".to_string(),
            name: "Test Item".to_string(),
            price: 123,
            sell_trend: 0,
            in_stock: true,
            ..Item::default()
        };
        let _ = catalog1.upsert(item1.clone());
        assert!(catalog1.persist().is_ok());
        // 2. Mark the item for deletion and persist again.
        catalog1.delete(&item1.id);
        assert!(catalog1.persist().is_ok());
        // 3. Create a new catalog and try to load the deleted item.
        let mut catalog2 = Catalog::new(db_path);
        assert!(catalog2.load_by_id(&item1.id).is_ok());
        // 4. Assert that the item was not found.
        let loaded_item: Option<Item> = catalog2.get(&item1.id).unwrap();
        assert!(loaded_item.is_none());
        // Clean up
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn persist_should_update_updated_entities() {
        // Should update entities with 'EntityState::Updated' in the database.
        let db_path = "target/test_dbs/persist_should_update_updated_entities.db";
        let path = std::path::Path::new(db_path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
        // 1. Insert an entity and persist it.
        let mut catalog1 = Catalog::new(db_path);
        catalog1.init().unwrap();
        let original_item = Item {
            id: "item-1".to_string(),
            name: "Original Name".to_string(),
            price: 100,
            sell_trend: 0,
            in_stock: true,
            ..Item::default()
        };
        let _ = catalog1.upsert(original_item.clone());
        catalog1.persist().unwrap();
        // 2. Load it into a new catalog to simulate a separate session.
        let mut catalog2 = Catalog::new(db_path);
        catalog2.load_by_id("item-1").unwrap();
        // 3. Upsert updated data for the same item. This should mark it as 'Updated'.
        let updated_item = Item {
            id: "item-1".to_string(),
            name: "Updated Name".to_string(),
            price: 200,
            sell_trend: 0,
            in_stock: false,
            ..Item::default()
        };
        let _ = catalog2.upsert(updated_item.clone());
        assert_eq!(
            catalog2.items.get("item-1").unwrap().state,
            EntityState::Updated
        );
        // 4. Persist the changes.
        catalog2.persist().unwrap();
        // 5. Load the data into a third catalog to verify the update was written to the DB.
        let mut catalog3 = Catalog::new(db_path);
        catalog3.load_by_id("item-1").unwrap();
        let loaded_item: Item = catalog3.get("item-1").unwrap().unwrap();
        // 6. Assert that the loaded item has the updated values.
        assert_eq!(loaded_item, updated_item);
        assert_ne!(loaded_item, original_item);
        // Clean up
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn persist_should_handle_mixed_entity_states() {
        // Should handle a mix of new, updated, and deleted entities in one operation.
        let db_path = "target/test_dbs/persist_should_handle_mixed_entity_states.db";
        let path = std::path::Path::new(db_path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
        // 1. Setup: Pre-populate the database with some items.
        let mut catalog_setup = Catalog::new(db_path);
        catalog_setup.init().unwrap();
        let item_to_update_original = Item {
            id: "update-me".to_string(),
            name: "Original".to_string(),
            price: 10,
            sell_trend: 0,
            in_stock: true,
            ..Item::default()
        };
        let item_to_delete = Item {
            id: "delete-me".to_string(),
            name: "Delete Me".to_string(),
            price: 20,
            sell_trend: 0,
            in_stock: true,
            ..Item::default()
        };
        let item_to_keep = Item {
            id: "keep-me".to_string(),
            name: "Keep Me".to_string(),
            price: 30,
            sell_trend: 0,
            in_stock: true,
            ..Item::default()
        };
        let _ = catalog_setup.upsert(item_to_update_original.clone());
        let _ = catalog_setup.upsert(item_to_delete.clone());
        let _ = catalog_setup.upsert(item_to_keep.clone());
        catalog_setup.persist().unwrap();
        // 2. Manipulation: Load the data and perform mixed operations.
        let mut catalog_ops = Catalog::new(db_path);
        catalog_ops.load_by_class::<Item>().unwrap(); // Load all items
        // A new item to be inserted.
        let item_to_add = Item {
            id: "add-me".to_string(),
            name: "Add Me".to_string(),
            price: 40,
            sell_trend: 0,
            in_stock: false,
            ..Item::default()
        };
        let _ = catalog_ops.upsert(item_to_add.clone()); // State: New
        // An updated version of an existing item.
        let item_to_update_new = Item {
            id: "update-me".to_string(),
            name: "Updated".to_string(),
            price: 11,
            sell_trend: 0,
            in_stock: false,
            ..Item::default()
        };
        let _ = catalog_ops.upsert(item_to_update_new.clone()); // State: Updated
        // An item to be deleted.
        catalog_ops.delete("delete-me"); // State: ToDelete
        // item_to_keep is left untouched (State: Synced after load)
        // 3. Execution: Persist all the changes in one go.
        catalog_ops.persist().unwrap();
        // 4. Verification: Load into a new catalog and check the final state of the DB.
        let mut catalog_verify = Catalog::new(db_path);
        catalog_verify.load_by_class::<Item>().unwrap();
        // Check total count
        assert_eq!(catalog_verify.items.len(), 3);
        // Verify added item
        let added_item: Item = catalog_verify.get("add-me").unwrap().unwrap();
        assert_eq!(added_item, item_to_add);
        // Verify updated item
        let updated_item: Item = catalog_verify.get("update-me").unwrap().unwrap();
        assert_eq!(updated_item, item_to_update_new);
        // Verify deleted item
        let deleted_item: Option<Item> = catalog_verify.get("delete-me").unwrap();
        assert!(deleted_item.is_none());
        // Verify untouched item
        let kept_item: Item = catalog_verify.get("keep-me").unwrap().unwrap();
        assert_eq!(kept_item, item_to_keep);
        // Clean up
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn persist_should_return_error_on_db_failure() {
        // Should return an error if the database connection fails or a query fails.
        // Using a directory as a path should cause a failure.
        let invalid_path = "target/test_dbs/a_directory_for_persist_fail";
        std::fs::create_dir_all(invalid_path).unwrap();
        let mut catalog = Catalog::new(invalid_path);
        let item = Item {
            id: "item-1".to_string(),
            name: "Test".to_string(),
            price: 10,
            sell_trend: 0,
            in_stock: true,
            ..Item::default()
        };
        let _ = catalog.upsert(item);
        let result = catalog.persist();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), FailedTo::PersistCatalog);
        // Clean up
        std::fs::remove_dir_all(invalid_path).unwrap();
    }
    #[test]
    fn persist_should_update_in_memory_state() {
        // After persisting, the in-memory state of entities should be considered.
        // (e.g., should deleted items be removed from the 'items' map, all other items should be marked as Loaded).
        let db_path = "target/test_dbs/persist_should_update_in_memory_state.db";
        let path = std::path::Path::new(db_path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
        // 1. Setup: Create a catalog and pre-populate it with some data.
        let mut catalog = Catalog::new(db_path);
        catalog.init().unwrap();
        let item_to_update = Item {
            id: "update-me".to_string(),
            name: "Original".to_string(),
            price: 0,
            sell_trend: 0,
            in_stock: false,
            ..Item::default()
        };
        let item_to_delete = Item {
            id: "delete-me".to_string(),
            name: "Delete Me".to_string(),
            price: 0,
            sell_trend: 0,
            in_stock: false,
            ..Item::default()
        };
        let item_untouched = Item {
            id: "keep-me".to_string(),
            name: "Keep Me".to_string(),
            price: 0,
            sell_trend: 0,
            in_stock: false,
            ..Item::default()
        };
        let _ = catalog.upsert(item_to_update.clone());
        let _ = catalog.upsert(item_to_delete.clone());
        let _ = catalog.upsert(item_untouched.clone());
        catalog.persist().unwrap();
        // At this point, all items are in the DB and in-memory state is `Loaded`.
        assert_eq!(catalog.items.len(), 3);
        assert_eq!(
            catalog.items.get("update-me").unwrap().state,
            EntityState::Loaded
        );
        assert_eq!(
            catalog.items.get("delete-me").unwrap().state,
            EntityState::Loaded
        );
        assert_eq!(
            catalog.items.get("keep-me").unwrap().state,
            EntityState::Loaded
        );
        // 2. Manipulate the catalog to have entities in various states.
        // A new item to be inserted.
        let item_new = Item {
            id: "add-me".to_string(),
            name: "Add Me".to_string(),
            price: 0,
            sell_trend: 0,
            in_stock: false,
            ..Item::default()
        };
        let _ = catalog.upsert(item_new.clone()); // State: New
        // An updated version of an existing item.
        let item_updated = Item {
            id: "update-me".to_string(),
            name: "Updated".to_string(),
            price: 0,
            sell_trend: 10,
            in_stock: false,
            ..Item::default()
        };
        let _ = catalog.upsert(item_updated.clone()); // State: Updated
        // An item to be deleted.
        catalog.delete("delete-me"); // State: ToDelete
        // 'item_untouched' remains with state `Loaded`.
        // Check states before final persist
        assert_eq!(catalog.items.get("add-me").unwrap().state, EntityState::New);
        assert_eq!(
            catalog.items.get("update-me").unwrap().state,
            EntityState::Updated
        );
        assert_eq!(
            catalog.items.get("delete-me").unwrap().state,
            EntityState::ToDelete
        );
        assert_eq!(
            catalog.items.get("keep-me").unwrap().state,
            EntityState::Loaded
        );
        assert_eq!(catalog.items.len(), 4);
        // 3. Persist all changes.
        catalog.persist().unwrap();
        // 4. Verify the in-memory state after persisting.
        // The item marked for deletion should be gone.
        assert!(!catalog.items.contains_key("delete-me"));
        assert_eq!(catalog.items.len(), 3);
        // All remaining items should have their state as `Loaded`.
        let new_item_entity = catalog.items.get("add-me").unwrap();
        assert_eq!(new_item_entity.state, EntityState::Loaded);
        assert_eq!(
            new_item_entity.value_of("name"),
            Some(&Value::from("Add Me"))
        );
        let updated_item_entity = catalog.items.get("update-me").unwrap();
        assert_eq!(updated_item_entity.state, EntityState::Loaded);
        assert_eq!(
            updated_item_entity.value_of("name"),
            Some(&Value::from("Updated"))
        );
        assert_eq!(
            updated_item_entity.value_of("sell_trend"),
            Some(&Value::from(10i64))
        );
        let untouched_item_entity = catalog.items.get("keep-me").unwrap();
        assert_eq!(untouched_item_entity.state, EntityState::Loaded);
        assert_eq!(
            untouched_item_entity.value_of("name"),
            Some(&Value::from("Keep Me"))
        );
        // Clean up
        std::fs::remove_file(path).unwrap();
    }
    // ## 'load_by_id()'
    #[test]
    fn load_by_id_should_load_entity_from_db() {
        // Should load a single entity from the database into the 'items' map.
        let db_path = "target/test_dbs/load_by_id_should_load_entity_from_db.db";
        let path = std::path::Path::new(db_path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
        // 1. Create a catalog, insert an item, and persist it to the DB.
        let mut catalog1 = Catalog::new(db_path);
        catalog1.init().unwrap();
        let item_to_persist = Item {
            id: "item-1".to_string(),
            name: "Test Item".to_string(),
            price: 123,
            sell_trend: 0,
            in_stock: true,
            ..Item::default()
        };
        let _ = catalog1.upsert(item_to_persist.clone());
        catalog1.persist().unwrap();
        // 2. Create a new, empty catalog instance for the same DB.
        let mut catalog2 = Catalog::new(db_path);
        assert!(catalog2.items.is_empty());
        // 3. Load the item by its ID.
        let result = catalog2.load_by_id("item-1");
        assert!(result.is_ok());
        // 4. Verify the item is now in the in-memory 'items' map.
        assert_eq!(catalog2.items.len(), 1);
        let loaded_entity = catalog2.items.get("item-1").unwrap();
        // 5. Verify the loaded entity's data and state.
        assert_eq!(loaded_entity.id, "item-1");
        assert_eq!(loaded_entity.class, "item");
        assert_eq!(
            loaded_entity.value_of("name"),
            Some(&Value::from("Test Item"))
        );
        assert_eq!(loaded_entity.value_of("price"), Some(&Value::from(123u64)));
        assert_eq!(
            loaded_entity.value_of("sell_trend"),
            Some(&Value::from(0i64))
        );
        assert_eq!(loaded_entity.value_of("in_stock"), Some(&Value::from(true)));
        assert_eq!(loaded_entity.state, EntityState::Loaded); // Should be Synced after loading.
        // 6. Also verify by using the public 'get' method.
        let retrieved_item: Option<Item> = catalog2.get("item-1").unwrap();
        assert_eq!(retrieved_item, Some(item_to_persist));
        // Clean up
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn load_by_id_should_overwrite_in_memory_entity() {
        // Should overwrite an existing in-memory entity with the same ID.
        let db_path = "target/test_dbs/load_by_id_should_overwrite_in_memory_entity.db";
        let path = std::path::Path::new(db_path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
        // 1. Persist an item to the database.
        let mut catalog1 = Catalog::new(db_path);
        catalog1.init().unwrap();
        let item_in_db = Item {
            id: "item-1".to_string(),
            name: "Item from DB".to_string(),
            price: 100,
            sell_trend: 0,
            in_stock: true,
            ..Item::default()
        };
        let _ = catalog1.upsert(item_in_db.clone());
        catalog1.persist().unwrap();
        // 2. Create a new catalog and add a *different* in-memory version of the same item.
        let mut catalog2 = Catalog::new(db_path);
        let item_in_memory = Item {
            id: "item-1".to_string(),
            name: "In-memory version".to_string(),
            price: 200,
            sell_trend: 0,
            in_stock: false,
            ..Item::default()
        };
        let _ = catalog2.upsert(item_in_memory);
        let entity_before_load = catalog2.items.get("item-1").unwrap();
        assert_eq!(entity_before_load.state, EntityState::New);
        assert_eq!(
            entity_before_load.value_of("name"),
            Some(&Value::from("In-memory version"))
        );
        // 3. Load the item from the database, which should overwrite the in-memory version.
        let result = catalog2.load_by_id("item-1");
        assert!(result.is_ok());
        // 4. Verify that the in-memory entity has been replaced with the one from the DB.
        let entity_after_load = catalog2.items.get("item-1").unwrap();
        assert_eq!(entity_after_load.state, EntityState::Loaded);
        assert_eq!(
            entity_after_load.value_of("name"),
            Some(&Value::from("Item from DB"))
        );
        assert_eq!(
            entity_after_load.value_of("price"),
            Some(&Value::from(100u64))
        );
        assert_eq!(
            entity_after_load.value_of("sell_trend"),
            Some(&Value::from(0i64))
        );
        // 5. Verify using the public 'get' method.
        let retrieved_item: Item = catalog2.get("item-1").unwrap().unwrap();
        assert_eq!(retrieved_item, item_in_db);
        // Clean up
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn load_by_id_should_do_nothing_if_not_found() {
        // Should do nothing if the entity is not found in the database.
        let db_path = "target/test_dbs/load_by_id_should_do_nothing_if_not_found.db";
        let path = std::path::Path::new(db_path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
        // 1. Create an empty, initialized database.
        let mut catalog = Catalog::new(db_path);
        catalog.init().unwrap();
        // 2. Attempt to load an ID that does not exist.
        let result = catalog.load_by_id("nonexistent-id");
        // 3. Verify that the operation succeeded and the catalog remains empty.
        assert!(result.is_ok());
        assert!(catalog.items.is_empty());
        // Clean up
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn load_by_id_should_return_error_on_db_failure() {
        // Should return an error if the database operation fails.
        // Using a directory as a path should cause a failure.
        let invalid_path = "target/test_dbs/a_directory_for_load_fail";
        std::fs::create_dir_all(invalid_path).unwrap();
        let mut catalog = Catalog::new(invalid_path);
        // Attempt to load from the invalid path.
        let result = catalog.load_by_id("any-id");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), FailedTo::LoadFromDB);
        // Clean up
        std::fs::remove_dir_all(invalid_path).unwrap();
    }
    // ## 'load_by_class()'
    #[test]
    fn load_by_class_should_load_all_entities_of_class() {
        // Should load all entities of a given class from the database into the 'items' map.
        let db_path = "target/test_dbs/load_by_class_should_load_all_entities_of_class.db";
        let path = std::path::Path::new(db_path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
        // 1. Setup DB with a few items of the same class
        let mut catalog1 = Catalog::new(db_path);
        catalog1.init().unwrap();
        let item1 = Item {
            id: "item-1".to_string(),
            name: "Item One".to_string(),
            price: 100,
            sell_trend: 0,
            in_stock: true,
            ..Item::default()
        };
        let item2 = Item {
            id: "item-2".to_string(),
            name: "Item Two".to_string(),
            price: 200,
            sell_trend: 0,
            in_stock: false,
            ..Item::default()
        };
        let _ = catalog1.upsert(item1.clone());
        let _ = catalog1.upsert(item2.clone());
        catalog1.persist().unwrap();
        // 2. Create a new catalog and load the items by class
        let mut catalog2 = Catalog::new(db_path);
        let result = catalog2.load_by_class::<Item>();
        assert!(result.is_ok());
        // 3. Verify that all items of that class were loaded
        assert_eq!(catalog2.items.len(), 2);
        let loaded_item1: Item = catalog2.get("item-1").unwrap().unwrap();
        let loaded_item2: Item = catalog2.get("item-2").unwrap().unwrap();
        assert_eq!(loaded_item1, item1);
        assert_eq!(loaded_item2, item2);
        assert_eq!(
            catalog2.items.get("item-1").unwrap().state,
            EntityState::Loaded
        );
        assert_eq!(
            catalog2.items.get("item-2").unwrap().state,
            EntityState::Loaded
        );
        // Clean up
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn load_by_class_should_overwrite_in_memory_entities() {
        // Should overwrite any existing in-memory entities with the same IDs.
        let db_path = "target/test_dbs/load_by_class_should_overwrite_in_memory_entities.db";
        let path = std::path::Path::new(db_path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
        // 1. Persist an item to the database.
        let mut catalog1 = Catalog::new(db_path);
        catalog1.init().unwrap();
        let item_in_db = Item {
            id: "item-1".to_string(),
            name: "DB Version".to_string(),
            price: 100,
            sell_trend: 0,
            in_stock: true,
            ..Item::default()
        };
        let _ = catalog1.upsert(item_in_db.clone());
        catalog1.persist().unwrap();
        // 2. Create a new catalog with a different in-memory version of the same item.
        let mut catalog2 = Catalog::new(db_path);
        let item_in_memory = Item {
            id: "item-1".to_string(),
            name: "Memory Version".to_string(),
            price: 200,
            sell_trend: 0,
            in_stock: false,
            ..Item::default()
        };
        let _ = catalog2.upsert(item_in_memory);
        assert_eq!(catalog2.items.len(), 1);
        assert_eq!(
            catalog2.get::<Item>("item-1").unwrap().unwrap().name,
            "Memory Version"
        );
        // 3. Load from the database, which should overwrite the in-memory version.
        let result = catalog2.load_by_class::<Item>();
        assert!(result.is_ok());
        // 4. Verify that the in-memory entity has been replaced with the one from the DB.
        assert_eq!(catalog2.items.len(), 1);
        let loaded_item: Item = catalog2.get("item-1").unwrap().unwrap();
        assert_eq!(loaded_item, item_in_db);
        assert_eq!(
            catalog2.items.get("item-1").unwrap().state,
            EntityState::Loaded
        );
        // Clean up
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn load_by_class_should_do_nothing_if_none_found() {
        // Should do nothing if no entities of that class are found in the database.
        let db_path = "target/test_dbs/load_by_class_should_do_nothing_if_none_found.db";
        let path = std::path::Path::new(db_path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
        // 1. Create an empty, initialized database.
        let mut catalog = Catalog::new(db_path);
        catalog.init().unwrap();
        // 2. Attempt to load from the empty DB.
        let result = catalog.load_by_class::<Item>();
        assert!(result.is_ok());
        assert!(catalog.items.is_empty());
        // 3. Add an item to memory and try loading again from the empty DB.
        let item_in_memory = Item {
            id: "item-1".to_string(),
            name: "In-memory only".to_string(),
            price: 100,
            sell_trend: 0,
            in_stock: true,
            ..Item::default()
        };
        let _ = catalog.upsert(item_in_memory.clone());
        assert_eq!(catalog.items.len(), 1);
        let result2 = catalog.load_by_class::<Item>();
        assert!(result2.is_ok());
        // 4. Verify the in-memory item is untouched because nothing was loaded from DB.
        assert_eq!(catalog.items.len(), 1);
        let retrieved_item: Item = catalog.get("item-1").unwrap().unwrap();
        assert_eq!(retrieved_item, item_in_memory);
        // Clean up
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn load_by_class_should_return_error_on_db_failure() {
        // Should return an error if the database operation fails.
        // Using a directory as a path should cause a failure.
        let invalid_path = "target/test_dbs/a_directory_for_load_class_fail";
        std::fs::create_dir_all(invalid_path).unwrap();
        let mut catalog = Catalog::new(invalid_path);
        // Attempt to load from the invalid path.
        let result = catalog.load_by_class::<Item>();
        assert!(result.is_err());
        // Based on `load_by_id`, the error should be `LoadFromDB`.
        // This assumes `From<SqliteFailedTo>` is implemented to produce `FailedTo::LoadFromDB`.
        assert_eq!(result.unwrap_err(), FailedTo::LoadFromDB);
        // Clean up
        std::fs::remove_dir_all(invalid_path).unwrap();
    }
    // ## 'load_by_filter()'
    #[test]
    fn load_by_filter_should_load_matching_entities() {
        // Verifies that entities matching a boolean filter are correctly loaded into the catalog.
        let db_path = "target/test_dbs/lbf_should_load_matching_entities.db";
        let path = std::path::Path::new(db_path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
        let mut catalog_setup = Catalog::new(db_path);
        catalog_setup.init().unwrap();
        let items = vec![
            Item {
                id: "item-1".to_string(),
                name: "Item One".to_string(),
                price: 100,
                sell_trend: 0,
                in_stock: true,
                ..Item::default()
            },
            Item {
                id: "item-2".to_string(),
                name: "Item Two".to_string(),
                price: 200,
                sell_trend: 0,
                in_stock: false,
                ..Item::default()
            },
            Item {
                id: "item-3".to_string(),
                name: "Item Three".to_string(),
                price: 300,
                sell_trend: 0,
                in_stock: true,
                ..Item::default()
            },
        ];
        catalog_setup.insert_many(items).unwrap();
        catalog_setup.persist().unwrap();
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new().with_bool("in_stock", true);
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert_eq!(catalog.items.len(), 2);
        assert!(catalog.items.contains_key("item-1"));
        assert!(catalog.items.contains_key("item-3"));
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn load_by_filter_should_not_load_non_matching_entities() {
        // Ensures that entities that do not match the filter are not loaded.
        let db_path = "target/test_dbs/lbf_should_not_load_non_matching_entities.db";
        let path = std::path::Path::new(db_path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
        let mut catalog_setup = Catalog::new(db_path);
        catalog_setup.init().unwrap();
        let items = vec![
            Item {
                id: "item-1".to_string(),
                name: "Item One".to_string(),
                price: 100,
                sell_trend: 0,
                in_stock: true,
                ..Item::default()
            },
            Item {
                id: "item-2".to_string(),
                name: "Item Two".to_string(),
                price: 200,
                sell_trend: 0,
                in_stock: false,
                ..Item::default()
            },
        ];
        catalog_setup.insert_many(items).unwrap();
        catalog_setup.persist().unwrap();
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new().with_bool("in_stock", false);
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert_eq!(catalog.items.len(), 1);
        assert!(catalog.items.contains_key("item-2"));
        assert!(!catalog.items.contains_key("item-1"));
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn load_by_filter_with_empty_filter_should_load_all_entities() {
        // Tests that using an empty filter loads all entities from the database.
        let db_path = "target/test_dbs/lbf_with_empty_filter_should_load_all_entities.db";
        let path = std::path::Path::new(db_path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
        let mut catalog_setup = Catalog::new(db_path);
        catalog_setup.init().unwrap();
        let items = vec![
            Item {
                id: "item-1".to_string(),
                name: "Item One".to_string(),
                price: 100,
                sell_trend: 0,
                in_stock: true,
                ..Item::default()
            },
            Item {
                id: "item-2".to_string(),
                name: "Item Two".to_string(),
                price: 200,
                sell_trend: 0,
                in_stock: false,
                ..Item::default()
            },
        ];
        catalog_setup.insert_many(items).unwrap();
        catalog_setup.persist().unwrap();
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new();
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert_eq!(catalog.items.len(), 2);
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn load_by_filter_should_overwrite_existing_entities() {
        // Checks that loading from the database overwrites any existing in-memory entities with the same ID.
        let db_path = "target/test_dbs/lbf_should_overwrite_existing_entities.db";
        let path = std::path::Path::new(db_path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
        let mut catalog_setup = Catalog::new(db_path);
        catalog_setup.init().unwrap();
        let item_in_db = Item {
            id: "item-1".to_string(),
            name: "DB Version".to_string(),
            price: 100,
            sell_trend: 0,
            in_stock: true,
            ..Item::default()
        };
        catalog_setup.upsert(item_in_db.clone()).unwrap();
        catalog_setup.persist().unwrap();
        let mut catalog = Catalog::new(db_path);
        let item_in_memory = Item {
            id: "item-1".to_string(),
            name: "Memory Version".to_string(),
            price: 200,
            sell_trend: 0,
            in_stock: false,
            ..Item::default()
        };
        catalog.upsert(item_in_memory).unwrap();
        let filter = Filter::new().with_bool("in_stock", true);
        assert!(catalog.load_by_filter(&filter).is_ok());
        let loaded_item: Item = catalog.get("item-1").unwrap().unwrap();
        assert_eq!(loaded_item, item_in_db);
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn load_by_filter_should_return_error_on_db_failure() {
        // Confirms that the function returns an appropriate error if the database operation fails.
        let invalid_path = "target/test_dbs/a_directory_for_lbf_fail";
        std::fs::create_dir_all(invalid_path).unwrap();
        let mut catalog = Catalog::new(invalid_path);
        let filter = Filter::new();
        let result = catalog.load_by_filter(&filter);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), FailedTo::LoadFromDB);
        std::fs::remove_dir_all(invalid_path).unwrap();
    }
    #[test]
    fn load_by_filter_should_load_matching_sell_trend() {
        let db_path = "target/test_dbs/lbf_matching_sell_trend.db";
        let path = std::path::Path::new(db_path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
        let mut catalog_setup = Catalog::new(db_path);
        catalog_setup.init().unwrap();
        let items = vec![
            Item {
                id: "item-1".to_string(),
                name: "Item One".to_string(),
                price: 100,
                sell_trend: 10,
                in_stock: true,
                ..Item::default()
            },
            Item {
                id: "item-2".to_string(),
                name: "Item Two".to_string(),
                price: 200,
                sell_trend: -5,
                in_stock: false,
                ..Item::default()
            },
            Item {
                id: "item-3".to_string(),
                name: "Item Three".to_string(),
                price: 300,
                sell_trend: 10,
                in_stock: true,
                ..Item::default()
            },
        ];
        catalog_setup.insert_many(items).unwrap();
        catalog_setup.persist().unwrap();
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new().with_signed_int("sell_trend", Comparison::Equal, 10);
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert_eq!(catalog.items.len(), 2);
        assert!(catalog.items.contains_key("item-1"));
        assert!(catalog.items.contains_key("item-3"));
        assert!(!catalog.items.contains_key("item-2"));
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn load_by_filter_should_load_matching_negative_sell_trend() {
        let db_path = "target/test_dbs/lbf_matching_negative_sell_trend.db";
        let path = std::path::Path::new(db_path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
        let mut catalog_setup = Catalog::new(db_path);
        catalog_setup.init().unwrap();
        let items = vec![
            Item {
                id: "item-1".to_string(),
                name: "Item One".to_string(),
                price: 100,
                sell_trend: 10,
                in_stock: true,
                ..Item::default()
            },
            Item {
                id: "item-2".to_string(),
                name: "Item Two".to_string(),
                price: 200,
                sell_trend: -5,
                in_stock: false,
                ..Item::default()
            },
            Item {
                id: "item-3".to_string(),
                name: "Item Three".to_string(),
                price: 300,
                sell_trend: 10,
                in_stock: true,
                ..Item::default()
            },
        ];
        catalog_setup.insert_many(items).unwrap();
        catalog_setup.persist().unwrap();
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new().with_signed_int("sell_trend", Comparison::Equal, -5);
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert_eq!(catalog.items.len(), 1);
        assert!(catalog.items.contains_key("item-2"));
        assert!(!catalog.items.contains_key("item-1"));
        assert!(!catalog.items.contains_key("item-3"));
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn load_by_filter_should_load_matching_greater_sell_trend() {
        let db_path = "target/test_dbs/lbf_matching_greater_sell_trend.db";
        let path = std::path::Path::new(db_path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
        let mut catalog_setup = Catalog::new(db_path);
        catalog_setup.init().unwrap();
        let items = vec![
            Item {
                id: "item-1".to_string(),
                name: "Item One".to_string(),
                price: 100,
                sell_trend: 10,
                in_stock: true,
                ..Item::default()
            },
            Item {
                id: "item-2".to_string(),
                name: "Item Two".to_string(),
                price: 200,
                sell_trend: -5,
                in_stock: false,
                ..Item::default()
            },
            Item {
                id: "item-3".to_string(),
                name: "Item Three".to_string(),
                price: 300,
                sell_trend: 20,
                in_stock: true,
                ..Item::default()
            },
        ];
        catalog_setup.insert_many(items).unwrap();
        catalog_setup.persist().unwrap();
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new().with_signed_int("sell_trend", Comparison::Greater, 5);
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert_eq!(catalog.items.len(), 2);
        assert!(catalog.items.contains_key("item-1"));
        assert!(!catalog.items.contains_key("item-2"));
        assert!(catalog.items.contains_key("item-3"));
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn load_by_filter_should_load_matching_greater_sell_trend_edge_cases() {
        let db_path = "target/test_dbs/lbf_greater_sell_trend_edge_cases.db";
        let path = std::path::Path::new(db_path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
        let mut catalog_setup = Catalog::new(db_path);
        catalog_setup.init().unwrap();
        let items = vec![
            Item {
                id: "item-1".to_string(),
                name: "Item One".to_string(),
                price: 100,
                sell_trend: 10,
                in_stock: true,
                ..Item::default()
            },
            Item {
                id: "item-2".to_string(),
                name: "Item Two".to_string(),
                price: 200,
                sell_trend: -5,
                in_stock: false,
                ..Item::default()
            },
            Item {
                id: "item-3".to_string(),
                name: "Item Three".to_string(),
                price: 300,
                sell_trend: 20,
                in_stock: true,
                ..Item::default()
            },
        ];
        catalog_setup.insert_many(items).unwrap();
        catalog_setup.persist().unwrap();
        // Edge Case 1: Value equal to filter value should not be included.
        let mut catalog1 = Catalog::new(db_path);
        let filter1 = Filter::new().with_signed_int("sell_trend", Comparison::Greater, 10);
        assert!(catalog1.load_by_filter(&filter1).is_ok());
        assert_eq!(catalog1.items.len(), 1);
        assert!(catalog1.items.contains_key("item-3"));
        // Edge Case 2: No values greater than filter value.
        let mut catalog2 = Catalog::new(db_path);
        let filter2 = Filter::new().with_signed_int("sell_trend", Comparison::Greater, 20);
        assert!(catalog2.load_by_filter(&filter2).is_ok());
        assert!(catalog2.items.is_empty());
        // Edge Case 3: All values greater than filter value.
        let mut catalog3 = Catalog::new(db_path);
        let filter3 = Filter::new().with_signed_int("sell_trend", Comparison::Greater, -10);
        assert!(catalog3.load_by_filter(&filter3).is_ok());
        assert_eq!(catalog3.items.len(), 3);
        assert!(catalog3.items.contains_key("item-1"));
        assert!(catalog3.items.contains_key("item-2"));
        assert!(catalog3.items.contains_key("item-3"));
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn load_by_filter_should_load_matching_lesser_sell_trend() {
        let db_path = "target/test_dbs/lbf_matching_lesser_sell_trend.db";
        let path = std::path::Path::new(db_path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
        let mut catalog_setup = Catalog::new(db_path);
        catalog_setup.init().unwrap();
        let items = vec![
            Item {
                id: "item-1".to_string(),
                name: "Item One".to_string(),
                price: 100,
                sell_trend: 10,
                in_stock: true,
                ..Item::default()
            },
            Item {
                id: "item-2".to_string(),
                name: "Item Two".to_string(),
                price: 200,
                sell_trend: -5,
                in_stock: false,
                ..Item::default()
            },
            Item {
                id: "item-3".to_string(),
                name: "Item Three".to_string(),
                price: 300,
                sell_trend: 20,
                in_stock: true,
                ..Item::default()
            },
        ];
        catalog_setup.insert_many(items).unwrap();
        catalog_setup.persist().unwrap();
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new().with_signed_int("sell_trend", Comparison::Lesser, 15);
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert_eq!(catalog.items.len(), 2);
        assert!(catalog.items.contains_key("item-1"));
        assert!(catalog.items.contains_key("item-2"));
        assert!(!catalog.items.contains_key("item-3"));
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn load_by_filter_should_load_matching_lesser_sell_trend_edge_cases() {
        let db_path = "target/test_dbs/lbf_lesser_sell_trend_edge_cases.db";
        let path = std::path::Path::new(db_path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
        let mut catalog_setup = Catalog::new(db_path);
        catalog_setup.init().unwrap();
        let items = vec![
            Item {
                id: "item-1".to_string(),
                name: "Item One".to_string(),
                price: 100,
                sell_trend: 10,
                in_stock: true,
                ..Item::default()
            },
            Item {
                id: "item-2".to_string(),
                name: "Item Two".to_string(),
                price: 200,
                sell_trend: -5,
                in_stock: false,
                ..Item::default()
            },
            Item {
                id: "item-3".to_string(),
                name: "Item Three".to_string(),
                price: 300,
                sell_trend: 20,
                in_stock: true,
                ..Item::default()
            },
        ];
        catalog_setup.insert_many(items).unwrap();
        catalog_setup.persist().unwrap();
        // Edge Case 1: Value equal to filter value should not be included.
        let mut catalog1 = Catalog::new(db_path);
        let filter1 = Filter::new().with_signed_int("sell_trend", Comparison::Lesser, 10);
        assert!(catalog1.load_by_filter(&filter1).is_ok());
        assert_eq!(catalog1.items.len(), 1);
        assert!(catalog1.items.contains_key("item-2"));
        // Edge Case 2: No values lesser than filter value.
        let mut catalog2 = Catalog::new(db_path);
        let filter2 = Filter::new().with_signed_int("sell_trend", Comparison::Lesser, -5);
        assert!(catalog2.load_by_filter(&filter2).is_ok());
        assert!(catalog2.items.is_empty());
        // Edge Case 3: All values lesser than filter value.
        let mut catalog3 = Catalog::new(db_path);
        let filter3 = Filter::new().with_signed_int("sell_trend", Comparison::Lesser, 25);
        assert!(catalog3.load_by_filter(&filter3).is_ok());
        assert_eq!(catalog3.items.len(), 3);
        assert!(catalog3.items.contains_key("item-1"));
        assert!(catalog3.items.contains_key("item-2"));
        assert!(catalog3.items.contains_key("item-3"));
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn load_by_filter_should_load_matching_greater_or_equal_sell_trend() {
        let db_path = "target/test_dbs/lbf_matching_greater_or_equal_sell_trend.db";
        let path = std::path::Path::new(db_path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
        let mut catalog_setup = Catalog::new(db_path);
        catalog_setup.init().unwrap();
        let items = vec![
            Item {
                id: "item-1".to_string(),
                name: "Item One".to_string(),
                price: 100,
                sell_trend: 10,
                in_stock: true,
                ..Item::default()
            },
            Item {
                id: "item-2".to_string(),
                name: "Item Two".to_string(),
                price: 200,
                sell_trend: -5,
                in_stock: false,
                ..Item::default()
            },
            Item {
                id: "item-3".to_string(),
                name: "Item Three".to_string(),
                price: 300,
                sell_trend: 20,
                in_stock: true,
                ..Item::default()
            },
        ];
        catalog_setup.insert_many(items).unwrap();
        catalog_setup.persist().unwrap();
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new().with_signed_int("sell_trend", Comparison::GreaterOrEqual, 10);
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert_eq!(catalog.items.len(), 2);
        assert!(catalog.items.contains_key("item-1"));
        assert!(!catalog.items.contains_key("item-2"));
        assert!(catalog.items.contains_key("item-3"));
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn load_by_filter_should_load_matching_greater_or_equal_sell_trend_edge_cases() {
        let db_path = "target/test_dbs/lbf_greater_or_equal_sell_trend_edge_cases.db";
        let path = std::path::Path::new(db_path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
        let mut catalog_setup = Catalog::new(db_path);
        catalog_setup.init().unwrap();
        let items = vec![
            Item {
                id: "item-1".to_string(),
                name: "Item One".to_string(),
                price: 100,
                sell_trend: 10,
                in_stock: true,
                ..Item::default()
            },
            Item {
                id: "item-2".to_string(),
                name: "Item Two".to_string(),
                price: 200,
                sell_trend: -5,
                in_stock: false,
                ..Item::default()
            },
            Item {
                id: "item-3".to_string(),
                name: "Item Three".to_string(),
                price: 300,
                sell_trend: 20,
                in_stock: true,
                ..Item::default()
            },
        ];
        catalog_setup.insert_many(items).unwrap();
        catalog_setup.persist().unwrap();
        // Edge Case 1: Value equal to filter value should be included.
        let mut catalog1 = Catalog::new(db_path);
        let filter1 = Filter::new().with_signed_int("sell_trend", Comparison::GreaterOrEqual, 20);
        assert!(catalog1.load_by_filter(&filter1).is_ok());
        assert_eq!(catalog1.items.len(), 1);
        assert!(catalog1.items.contains_key("item-3"));
        // Edge Case 2: No values greater than or equal to filter value.
        let mut catalog2 = Catalog::new(db_path);
        let filter2 = Filter::new().with_signed_int("sell_trend", Comparison::GreaterOrEqual, 21);
        assert!(catalog2.load_by_filter(&filter2).is_ok());
        assert!(catalog2.items.is_empty());
        // Edge Case 3: All values greater than or equal to filter value.
        let mut catalog3 = Catalog::new(db_path);
        let filter3 = Filter::new().with_signed_int("sell_trend", Comparison::GreaterOrEqual, -5);
        assert!(catalog3.load_by_filter(&filter3).is_ok());
        assert_eq!(catalog3.items.len(), 3);
        assert!(catalog3.items.contains_key("item-1"));
        assert!(catalog3.items.contains_key("item-2"));
        assert!(catalog3.items.contains_key("item-3"));
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn load_by_filter_should_load_matching_lesser_or_equal_sell_trend() {
        let db_path = "target/test_dbs/lbf_matching_lesser_or_equal_sell_trend.db";
        let path = std::path::Path::new(db_path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
        let mut catalog_setup = Catalog::new(db_path);
        catalog_setup.init().unwrap();
        let items = vec![
            Item {
                id: "item-1".to_string(),
                name: "Item One".to_string(),
                price: 100,
                sell_trend: 10,
                in_stock: true,
                ..Item::default()
            },
            Item {
                id: "item-2".to_string(),
                name: "Item Two".to_string(),
                price: 200,
                sell_trend: -5,
                in_stock: false,
                ..Item::default()
            },
            Item {
                id: "item-3".to_string(),
                name: "Item Three".to_string(),
                price: 300,
                sell_trend: 20,
                in_stock: true,
                ..Item::default()
            },
        ];
        catalog_setup.insert_many(items).unwrap();
        catalog_setup.persist().unwrap();
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new().with_signed_int("sell_trend", Comparison::LesserOrEqual, 10);
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert_eq!(catalog.items.len(), 2);
        assert!(catalog.items.contains_key("item-1"));
        assert!(catalog.items.contains_key("item-2"));
        assert!(!catalog.items.contains_key("item-3"));
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn load_by_filter_should_load_matching_lesser_or_equal_sell_trend_edge_cases() {
        let db_path = "target/test_dbs/lbf_lesser_or_equal_sell_trend_edge_cases.db";
        let path = std::path::Path::new(db_path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
        let mut catalog_setup = Catalog::new(db_path);
        catalog_setup.init().unwrap();
        let items = vec![
            Item {
                id: "item-1".to_string(),
                name: "Item One".to_string(),
                price: 100,
                sell_trend: 10,
                in_stock: true,
                ..Item::default()
            },
            Item {
                id: "item-2".to_string(),
                name: "Item Two".to_string(),
                price: 200,
                sell_trend: -5,
                in_stock: false,
                ..Item::default()
            },
            Item {
                id: "item-3".to_string(),
                name: "Item Three".to_string(),
                price: 300,
                sell_trend: 20,
                in_stock: true,
                ..Item::default()
            },
        ];
        catalog_setup.insert_many(items).unwrap();
        catalog_setup.persist().unwrap();
        // Edge Case 1: Value equal to filter value should be included.
        let mut catalog1 = Catalog::new(db_path);
        let filter1 = Filter::new().with_signed_int("sell_trend", Comparison::LesserOrEqual, -5);
        assert!(catalog1.load_by_filter(&filter1).is_ok());
        assert_eq!(catalog1.items.len(), 1);
        assert!(catalog1.items.contains_key("item-2"));
        // Edge Case 2: No values lesser than or equal to filter value.
        let mut catalog2 = Catalog::new(db_path);
        let filter2 = Filter::new().with_signed_int("sell_trend", Comparison::LesserOrEqual, -6);
        assert!(catalog2.load_by_filter(&filter2).is_ok());
        assert!(catalog2.items.is_empty());
        // Edge Case 3: All values lesser than or equal to filter value.
        let mut catalog3 = Catalog::new(db_path);
        let filter3 = Filter::new().with_signed_int("sell_trend", Comparison::LesserOrEqual, 20);
        assert!(catalog3.load_by_filter(&filter3).is_ok());
        assert_eq!(catalog3.items.len(), 3);
        assert!(catalog3.items.contains_key("item-1"));
        assert!(catalog3.items.contains_key("item-2"));
        assert!(catalog3.items.contains_key("item-3"));
        std::fs::remove_file(path).unwrap();
    }
    // ## Integration Tests
    #[test]
    fn integration_test_init_insert_persist_load_get() {
        // Scenario: 'init' -> 'insert' -> 'persist' -> create a new catalog instance -> 'load_by_id' -> 'get' -> verify data integrity.
        let db_path = "target/test_dbs/integration_test_init_insert_persist_load_get.db";
        let path = std::path::Path::new(db_path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
        // 1. 'init' -> 'insert' -> 'persist'
        let mut catalog1 = Catalog::new(db_path);
        catalog1.init().unwrap();
        let item_to_insert = Item {
            id: "item-1".to_string(),
            name: "Integration Test Item".to_string(),
            price: 999,
            sell_trend: 0,
            in_stock: true,
            ..Item::default()
        };
        let _ = catalog1.upsert(item_to_insert.clone());
        catalog1.persist().unwrap();
        // 2. create a new catalog instance -> 'load_by_id' -> 'get'
        let mut catalog2 = Catalog::new(db_path);
        catalog2.load_by_id("item-1").unwrap();
        let loaded_item: Option<Item> = catalog2.get("item-1").unwrap();
        // 3. verify data integrity
        assert_eq!(loaded_item, Some(item_to_insert));
        // Clean up
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn integration_test_init_insert_many_persist_load_list() {
        // Scenario: 'init' -> 'insert_many' -> 'persist' -> new catalog -> 'load_by_class' -> 'list_by_class' -> verify all items are loaded.
        let db_path = "target/test_dbs/integration_test_init_insert_many_persist_load_list.db";
        let path = std::path::Path::new(db_path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
        // 1. 'init' -> 'insert_many' -> 'persist'
        let mut catalog1 = Catalog::new(db_path);
        catalog1.init().unwrap();
        let items_to_insert = vec![
            Item {
                id: "item-1".to_string(),
                name: "Item One".to_string(),
                price: 100,
                sell_trend: 0,
                in_stock: true,
                ..Item::default()
            },
            Item {
                id: "item-2".to_string(),
                name: "Item Two".to_string(),
                price: 200,
                sell_trend: 0,
                in_stock: false,
                ..Item::default()
            },
        ];
        let _ = catalog1.insert_many(items_to_insert.clone());
        catalog1.persist().unwrap();
        // 2. new catalog -> 'load_by_class' -> 'list_by_class'
        let mut catalog2 = Catalog::new(db_path);
        catalog2.load_by_class::<Item>().unwrap();
        let mut loaded_items: Vec<Item> = catalog2
            .list_by_class::<Item>()
            .map(|item| item.unwrap())
            .collect();
        // Sort by ID to ensure consistent order for comparison
        let mut expected_items = items_to_insert;
        loaded_items.sort_by(|a, b| a.id.cmp(&b.id));
        expected_items.sort_by(|a, b| a.id.cmp(&b.id));
        // 3. verify all items are loaded
        assert_eq!(loaded_items.len(), 2);
        assert_eq!(loaded_items, expected_items);
        // Clean up
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn integration_test_insert_persist_load_delete_persist_load() {
        // Scenario: 'insert' -> 'persist' -> 'load_by_id' -> 'delete' -> 'persist' -> 'load_by_id' should now return nothing for the deleted ID.
        let db_path = "target/test_dbs/integration_test_insert_persist_load_delete_persist_load.db";
        let path = std::path::Path::new(db_path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
        // 1. 'insert' -> 'persist'
        let mut catalog1 = Catalog::new(db_path);
        catalog1.init().unwrap();
        let item_to_delete = Item {
            id: "item-to-delete".to_string(),
            name: "Test Item".to_string(),
            price: 123,
            sell_trend: 0,
            in_stock: true,
            ..Item::default()
        };
        let _ = catalog1.upsert(item_to_delete.clone());
        catalog1.persist().unwrap();
        // 2. 'load_by_id' to confirm it's there
        let mut catalog2 = Catalog::new(db_path);
        catalog2.load_by_id("item-to-delete").unwrap();
        assert!(catalog2.get::<Item>("item-to-delete").unwrap().is_some());
        // 3. 'delete' -> 'persist'
        catalog2.delete("item-to-delete");
        catalog2.persist().unwrap();
        // 4. 'load_by_id' should now return nothing
        let mut catalog3 = Catalog::new(db_path);
        catalog3.load_by_id("item-to-delete").unwrap();
        let loaded_item: Option<Item> = catalog3.get("item-to-delete").unwrap();
        assert!(loaded_item.is_none());
        // Clean up
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn integration_test_insert_get_by_class_and_attribute() {
        // Scenario: 'insert' -> 'get_by_class_and_attribute' should return the correct item.
        // This test focuses on in-memory functionality.
        let mut catalog = Catalog::new("dummy.db");
        let item1 = Item {
            id: "item-1".to_string(),
            name: "First Item".to_string(),
            price: 100,
            sell_trend: 0,
            in_stock: true,
            ..Item::default()
        };
        let item2 = Item {
            id: "item-2".to_string(),
            name: "Second Item".to_string(),
            price: 200,
            sell_trend: 0,
            in_stock: false,
            ..Item::default()
        };
        let _ = catalog.upsert(item1.clone());
        let _ = catalog.upsert(item2.clone());
        // Retrieve by a unique attribute
        let retrieved_item: Option<Item> = catalog
            .get_by_class_and_attribute("name", "Second Item")
            .unwrap();
        // Verify the correct item was retrieved
        assert_eq!(retrieved_item, Some(item2));
    }
    #[test]
    fn integration_test_insert_many_list_by_class_and_attribute() {
        // Scenario: 'insert_many' -> 'list_by_class_and_attribute' should return all matching items.
        // This test focuses on in-memory functionality.
        let mut catalog = Catalog::new("dummy.db");
        let item1 = Item {
            id: "item-1".to_string(),
            name: "Item One".to_string(),
            price: 100,
            sell_trend: 0,
            in_stock: true,
            ..Item::default()
        };
        let item2 = Item {
            id: "item-2".to_string(),
            name: "Item Two".to_string(),
            price: 200,
            sell_trend: 0,
            in_stock: false,
            ..Item::default()
        };
        let item3 = Item {
            id: "item-3".to_string(),
            name: "Item Three".to_string(),
            price: 150,
            sell_trend: 0,
            in_stock: true,
            ..Item::default()
        };
        let _ = catalog.insert_many(vec![item1.clone(), item2.clone(), item3.clone()]);
        // List all items that are in stock
        let mut results: Vec<Item> = catalog
            .list_by_class_and_attribute("in_stock", true)
            .map(|item| item.unwrap())
            .collect();
        // Sort for deterministic comparison
        results.sort_by(|a, b| a.id.cmp(&b.id));
        let mut expected = vec![item1, item3];
        expected.sort_by(|a, b| a.id.cmp(&b.id));
        // Verify the correct items were retrieved
        assert_eq!(results.len(), 2);
        assert_eq!(results, expected);
    }
    #[test]
    #[ignore] // This test can be flaky as it depends on thread scheduling.
    fn integration_test_concurrency() {
        // Scenario: Concurrency - what happens if two 'Catalog' instances point to the same file?
        // This test demonstrates that without application-level locking, a "last-write-wins"
        // race condition can occur, leading to lost updates.
        let db_path = "target/test_dbs/integration_test_concurrency.db";
        let path = std::path::Path::new(db_path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
        // 1. Initial setup
        let mut catalog_setup = Catalog::new(db_path);
        catalog_setup.init().unwrap();
        let initial_item = Item {
            id: "item-1".to_string(),
            name: "Original".to_string(),
            price: 100,
            sell_trend: 0,
            in_stock: true,
            ..Item::default()
        };
        let _ = catalog_setup.upsert(initial_item);
        catalog_setup.persist().unwrap();
        let db_path_arc = std::sync::Arc::new(String::from(db_path));
        // 2. Thread 1: Loads, updates name, and persists.
        let db_path_arc1 = std::sync::Arc::clone(&db_path_arc);
        let handle1 = std::thread::spawn(move || {
            let mut catalog1 = Catalog::new(&db_path_arc1);
            catalog1.load_by_id("item-1").unwrap();
            let mut item = catalog1.get::<Item>("item-1").unwrap().unwrap();
            item.name = "Updated by Thread 1".to_string();
            let _ = catalog1.upsert(item);
            catalog1.persist().unwrap();
        });
        // 3. Thread 2: Loads, updates price, and persists.
        let db_path_arc2 = std::sync::Arc::clone(&db_path_arc);
        let handle2 = std::thread::spawn(move || {
            let mut catalog2 = Catalog::new(&db_path_arc2);
            catalog2.load_by_id("item-1").unwrap();
            let mut item = catalog2.get::<Item>("item-1").unwrap().unwrap();
            item.price = 200;
            let _ = catalog2.upsert(item);
            catalog2.persist().unwrap();
        });
        handle1.join().unwrap();
        handle2.join().unwrap();
        // 4. Verification: Load the data and check the final state.
        let mut catalog_verify = Catalog::new(db_path);
        catalog_verify.load_by_id("item-1").unwrap();
        let final_item: Item = catalog_verify.get("item-1").unwrap().unwrap();
        // The final state depends on which thread persisted last. One update will have been lost.
        let thread1_won = final_item.name == "Updated by Thread 1"
            && final_item.price == 100
            && final_item.sell_trend == 0;
        let thread2_won =
            final_item.name == "Original" && final_item.price == 200 && final_item.sell_trend == 0;
        assert!(
            thread1_won || thread2_won,
            "Final state must be the result of one of the threads winning the race."
        );
        // Clean up
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn load_by_filter_should_load_unsigned_int_comparisons() {
        let db_path = "target/test_dbs/lbf_unsigned_int_comparisons.db";
        let path = std::path::Path::new(db_path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
        let mut catalog_setup = Catalog::new(db_path);
        catalog_setup.init().unwrap();
        let items = vec![
            Item {
                id: "item-1".to_string(),
                name: "Item One".to_string(),
                price: 100,
                sell_trend: 0,
                in_stock: true,
                ..Item::default()
            },
            Item {
                id: "item-2".to_string(),
                name: "Item Two".to_string(),
                price: 200,
                sell_trend: 0,
                in_stock: false,
                ..Item::default()
            },
            Item {
                id: "item-3".to_string(),
                name: "Item Three".to_string(),
                price: 300,
                sell_trend: 0,
                in_stock: true,
                ..Item::default()
            },
        ];
        catalog_setup.insert_many(items).unwrap();
        catalog_setup.persist().unwrap();
        // Equal
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new().with_unsigned_int("price", Comparison::Equal, 200);
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert_eq!(catalog.items.len(), 1);
        assert!(catalog.items.contains_key("item-2"));
        // Greater
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new().with_unsigned_int("price", Comparison::Greater, 200);
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert_eq!(catalog.items.len(), 1);
        assert!(catalog.items.contains_key("item-3"));
        // Lesser
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new().with_unsigned_int("price", Comparison::Lesser, 200);
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert_eq!(catalog.items.len(), 1);
        assert!(catalog.items.contains_key("item-1"));
        // GreaterOrEqual
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new().with_unsigned_int("price", Comparison::GreaterOrEqual, 200);
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert_eq!(catalog.items.len(), 2);
        assert!(catalog.items.contains_key("item-2"));
        assert!(catalog.items.contains_key("item-3"));
        // LesserOrEqual
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new().with_unsigned_int("price", Comparison::LesserOrEqual, 200);
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert_eq!(catalog.items.len(), 2);
        assert!(catalog.items.contains_key("item-1"));
        assert!(catalog.items.contains_key("item-2"));
        // Edge case: No match
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new().with_unsigned_int("price", Comparison::Equal, 400);
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert!(catalog.items.is_empty());
        // Edge case: All match
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new().with_unsigned_int("price", Comparison::GreaterOrEqual, 100);
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert_eq!(catalog.items.len(), 3);
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn load_by_filter_should_load_text_is_exactly_comparisons() {
        let db_path = "target/test_dbs/lbf_text_is_exactly_comparisons.db";
        let path = std::path::Path::new(db_path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
        let mut catalog_setup = Catalog::new(db_path);
        catalog_setup.init().unwrap();
        let items = vec![
            Item {
                id: "item-1".to_string(),
                name: "Item One".to_string(),
                price: 100,
                sell_trend: 0,
                in_stock: true,
                ..Item::default()
            },
            Item {
                id: "item-2".to_string(),
                name: "Item Two".to_string(),
                price: 200,
                sell_trend: 0,
                in_stock: false,
                ..Item::default()
            },
            Item {
                id: "item-3".to_string(),
                name: "Another Item".to_string(),
                price: 300,
                sell_trend: 0,
                in_stock: true,
                ..Item::default()
            },
        ];
        catalog_setup.insert_many(items).unwrap();
        catalog_setup.persist().unwrap();
        // Exact match
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new().with_text("name", Comparison::IsExactly, "Item One");
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert_eq!(catalog.items.len(), 1);
        assert!(catalog.items.contains_key("item-1"));
        // Partial match should not work
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new().with_text("name", Comparison::IsExactly, "Item");
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert!(catalog.items.is_empty());
        // Case insensitive match
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new().with_text("name", Comparison::IsExactly, "item one");
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert_eq!(catalog.items.len(), 1);
        assert!(catalog.items.contains_key("item-1"));
        // No match
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new().with_text("name", Comparison::IsExactly, "Item Four");
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert!(catalog.items.is_empty());
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn load_by_filter_should_load_text_starts_with_comparisons() {
        let db_path = "target/test_dbs/lbf_text_starts_with_comparisons.db";
        let path = std::path::Path::new(db_path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
        let mut catalog_setup = Catalog::new(db_path);
        catalog_setup.init().unwrap();
        let items = vec![
            Item {
                id: "item-1".to_string(),
                name: "Item One".to_string(),
                price: 100,
                sell_trend: 0,
                in_stock: true,
                ..Item::default()
            },
            Item {
                id: "item-2".to_string(),
                name: "Item Two".to_string(),
                price: 200,
                sell_trend: 0,
                in_stock: false,
                ..Item::default()
            },
            Item {
                id: "item-3".to_string(),
                name: "Another Item".to_string(),
                price: 300,
                sell_trend: 0,
                in_stock: true,
                ..Item::default()
            },
        ];
        catalog_setup.insert_many(items).unwrap();
        catalog_setup.persist().unwrap();
        // Starts with "Item"
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new().with_text("name", Comparison::StartsWith, "Item");
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert_eq!(catalog.items.len(), 2);
        assert!(catalog.items.contains_key("item-1"));
        assert!(catalog.items.contains_key("item-2"));
        // Starts with "I"
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new().with_text("name", Comparison::StartsWith, "I");
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert_eq!(catalog.items.len(), 2);
        assert!(catalog.items.contains_key("item-1"));
        assert!(catalog.items.contains_key("item-2"));
        // No match
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new().with_text("name", Comparison::StartsWith, "Z");
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert!(catalog.items.is_empty());
        // Case insensitive
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new().with_text("name", Comparison::StartsWith, "item");
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert_eq!(catalog.items.len(), 2);
        assert!(catalog.items.contains_key("item-1"));
        assert!(catalog.items.contains_key("item-2"));
        // Full string match
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new().with_text("name", Comparison::StartsWith, "Item One");
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert_eq!(catalog.items.len(), 1);
        assert!(catalog.items.contains_key("item-1"));
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn load_by_filter_should_load_text_ends_with_comparisons() {
        let db_path = "target/test_dbs/lbf_text_ends_with_comparisons.db";
        let path = std::path::Path::new(db_path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
        let mut catalog_setup = Catalog::new(db_path);
        catalog_setup.init().unwrap();
        let items = vec![
            Item {
                id: "item-1".to_string(),
                name: "Item One".to_string(),
                price: 100,
                sell_trend: 0,
                in_stock: true,
                ..Item::default()
            },
            Item {
                id: "item-2".to_string(),
                name: "Item Two".to_string(),
                price: 200,
                sell_trend: 0,
                in_stock: false,
                ..Item::default()
            },
            Item {
                id: "item-3".to_string(),
                name: "Another Item".to_string(),
                price: 300,
                sell_trend: 0,
                in_stock: true,
                ..Item::default()
            },
        ];
        catalog_setup.insert_many(items).unwrap();
        catalog_setup.persist().unwrap();
        // Ends with "one" - case insensitive
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new().with_text("name", Comparison::EndsWith, "one");
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert_eq!(catalog.items.len(), 1);
        assert!(catalog.items.contains_key("item-1"));
        // Ends with "item" - case insensitive
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new().with_text("name", Comparison::EndsWith, "item");
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert_eq!(catalog.items.len(), 1);
        assert!(catalog.items.contains_key("item-3"));
        // No match
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new().with_text("name", Comparison::EndsWith, "Z");
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert!(catalog.items.is_empty());
        // Full string match - case insensitive
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new().with_text("name", Comparison::EndsWith, "item one");
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert_eq!(catalog.items.len(), 1);
        assert!(catalog.items.contains_key("item-1"));
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn load_by_filter_should_load_text_contains_comparisons() {
        let db_path = "target/test_dbs/lbf_text_contains_comparisons.db";
        let path = std::path::Path::new(db_path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
        let mut catalog_setup = Catalog::new(db_path);
        catalog_setup.init().unwrap();
        let items = vec![
            Item {
                id: "item-1".to_string(),
                name: "Item One".to_string(),
                price: 100,
                sell_trend: 0,
                in_stock: true,
                ..Item::default()
            },
            Item {
                id: "item-2".to_string(),
                name: "Item Two".to_string(),
                price: 200,
                sell_trend: 0,
                in_stock: false,
                ..Item::default()
            },
            Item {
                id: "item-3".to_string(),
                name: "Another Item".to_string(),
                price: 300,
                sell_trend: 0,
                in_stock: true,
                ..Item::default()
            },
        ];
        catalog_setup.insert_many(items).unwrap();
        catalog_setup.persist().unwrap();
        // Contains "item" - case insensitive
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new().with_text("name", Comparison::Contains, "item");
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert_eq!(catalog.items.len(), 3);
        // Contains "THE" - case insensitive
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new().with_text("name", Comparison::Contains, "THE");
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert_eq!(catalog.items.len(), 1);
        assert!(catalog.items.contains_key("item-3"));
        // No match
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new().with_text("name", Comparison::Contains, "Z");
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert!(catalog.items.is_empty());
        // Full string match - case insensitive
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new().with_text("name", Comparison::Contains, "item one");
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert_eq!(catalog.items.len(), 1);
        assert!(catalog.items.contains_key("item-1"));
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn load_by_filter_should_load_matching_discount_equal() {
        let db_path = "target/test_dbs/lbf_matching_discount_equal.db";
        let path = std::path::Path::new(db_path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
        let mut catalog_setup = Catalog::new(db_path);
        catalog_setup.init().unwrap();
        let items = vec![
            Item {
                id: "item-1".to_string(),
                discount: 0.10,
                ..Item::default()
            },
            Item {
                id: "item-2".to_string(),
                discount: 0.20,
                ..Item::default()
            },
            Item {
                id: "item-3".to_string(),
                discount: 0.10,
                ..Item::default()
            },
        ];
        catalog_setup.insert_many(items).unwrap();
        catalog_setup.persist().unwrap();
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new().with_real("discount", Comparison::Equal, 0.10);
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert_eq!(catalog.items.len(), 2);
        assert!(catalog.items.contains_key("item-1"));
        assert!(catalog.items.contains_key("item-3"));
        assert!(!catalog.items.contains_key("item-2"));
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn load_by_filter_should_load_matching_discount_greater() {
        let db_path = "target/test_dbs/lbf_matching_discount_greater.db";
        let path = std::path::Path::new(db_path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
        let mut catalog_setup = Catalog::new(db_path);
        catalog_setup.init().unwrap();
        let items = vec![
            Item {
                id: "item-1".to_string(),
                discount: 0.10,
                ..Item::default()
            },
            Item {
                id: "item-2".to_string(),
                discount: 0.20,
                ..Item::default()
            },
            Item {
                id: "item-3".to_string(),
                discount: 0.30,
                ..Item::default()
            },
        ];
        catalog_setup.insert_many(items).unwrap();
        catalog_setup.persist().unwrap();
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new().with_real("discount", Comparison::Greater, 0.15);
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert_eq!(catalog.items.len(), 2);
        assert!(catalog.items.contains_key("item-2"));
        assert!(catalog.items.contains_key("item-3"));
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn load_by_filter_should_load_matching_discount_greater_edge_cases() {
        let db_path = "target/test_dbs/lbf_discount_greater_edge_cases.db";
        let path = std::path::Path::new(db_path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
        let mut catalog_setup = Catalog::new(db_path);
        catalog_setup.init().unwrap();
        let items = vec![
            Item {
                id: "item-1".to_string(),
                discount: 0.10,
                ..Item::default()
            },
            Item {
                id: "item-2".to_string(),
                discount: 0.20,
                ..Item::default()
            },
        ];
        catalog_setup.insert_many(items).unwrap();
        catalog_setup.persist().unwrap();
        let mut catalog1 = Catalog::new(db_path);
        let filter1 = Filter::new().with_real("discount", Comparison::Greater, 0.10);
        assert!(catalog1.load_by_filter(&filter1).is_ok());
        assert_eq!(catalog1.items.len(), 1);
        assert!(catalog1.items.contains_key("item-2"));
        let mut catalog2 = Catalog::new(db_path);
        let filter2 = Filter::new().with_real("discount", Comparison::Greater, 0.20);
        assert!(catalog2.load_by_filter(&filter2).is_ok());
        assert!(catalog2.items.is_empty());
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn load_by_filter_should_load_matching_discount_lesser() {
        let db_path = "target/test_dbs/lbf_matching_discount_lesser.db";
        let path = std::path::Path::new(db_path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
        let mut catalog_setup = Catalog::new(db_path);
        catalog_setup.init().unwrap();
        let items = vec![
            Item {
                id: "item-1".to_string(),
                discount: 0.10,
                ..Item::default()
            },
            Item {
                id: "item-2".to_string(),
                discount: 0.20,
                ..Item::default()
            },
            Item {
                id: "item-3".to_string(),
                discount: 0.05,
                ..Item::default()
            },
        ];
        catalog_setup.insert_many(items).unwrap();
        catalog_setup.persist().unwrap();
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new().with_real("discount", Comparison::Lesser, 0.15);
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert_eq!(catalog.items.len(), 2);
        assert!(catalog.items.contains_key("item-1"));
        assert!(catalog.items.contains_key("item-3"));
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn load_by_filter_should_load_matching_discount_lesser_edge_cases() {
        let db_path = "target/test_dbs/lbf_discount_lesser_edge_cases.db";
        let path = std::path::Path::new(db_path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
        let mut catalog_setup = Catalog::new(db_path);
        catalog_setup.init().unwrap();
        let items = vec![
            Item {
                id: "item-1".to_string(),
                discount: 0.10,
                ..Item::default()
            },
            Item {
                id: "item-2".to_string(),
                discount: 0.20,
                ..Item::default()
            },
        ];
        catalog_setup.insert_many(items).unwrap();
        catalog_setup.persist().unwrap();
        let mut catalog1 = Catalog::new(db_path);
        let filter1 = Filter::new().with_real("discount", Comparison::Lesser, 0.20);
        assert!(catalog1.load_by_filter(&filter1).is_ok());
        assert_eq!(catalog1.items.len(), 1);
        assert!(catalog1.items.contains_key("item-1"));
        let mut catalog2 = Catalog::new(db_path);
        let filter2 = Filter::new().with_real("discount", Comparison::Lesser, 0.10);
        assert!(catalog2.load_by_filter(&filter2).is_ok());
        assert!(catalog2.items.is_empty());
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn load_by_filter_should_load_matching_discount_greater_or_equal() {
        let db_path = "target/test_dbs/lbf_matching_discount_greater_or_equal.db";
        let path = std::path::Path::new(db_path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
        let mut catalog_setup = Catalog::new(db_path);
        catalog_setup.init().unwrap();
        let items = vec![
            Item {
                id: "item-1".to_string(),
                discount: 0.10,
                ..Item::default()
            },
            Item {
                id: "item-2".to_string(),
                discount: 0.20,
                ..Item::default()
            },
            Item {
                id: "item-3".to_string(),
                discount: 0.15,
                ..Item::default()
            },
        ];
        catalog_setup.insert_many(items).unwrap();
        catalog_setup.persist().unwrap();
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new().with_real("discount", Comparison::GreaterOrEqual, 0.15);
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert_eq!(catalog.items.len(), 2);
        assert!(catalog.items.contains_key("item-2"));
        assert!(catalog.items.contains_key("item-3"));
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn load_by_filter_should_load_matching_discount_greater_or_equal_edge_cases() {
        let db_path = "target/test_dbs/lbf_discount_greater_or_equal_edge_cases.db";
        let path = std::path::Path::new(db_path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
        let mut catalog_setup = Catalog::new(db_path);
        catalog_setup.init().unwrap();
        let items = vec![
            Item {
                id: "item-1".to_string(),
                discount: 0.10,
                ..Item::default()
            },
            Item {
                id: "item-2".to_string(),
                discount: 0.20,
                ..Item::default()
            },
        ];
        catalog_setup.insert_many(items).unwrap();
        catalog_setup.persist().unwrap();
        let mut catalog1 = Catalog::new(db_path);
        let filter1 = Filter::new().with_real("discount", Comparison::GreaterOrEqual, 0.20);
        assert!(catalog1.load_by_filter(&filter1).is_ok());
        assert_eq!(catalog1.items.len(), 1);
        assert!(catalog1.items.contains_key("item-2"));
        let mut catalog2 = Catalog::new(db_path);
        let filter2 = Filter::new().with_real("discount", Comparison::GreaterOrEqual, 0.21);
        assert!(catalog2.load_by_filter(&filter2).is_ok());
        assert!(catalog2.items.is_empty());
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn load_by_filter_should_load_matching_discount_lesser_or_equal() {
        let db_path = "target/test_dbs/lbf_matching_discount_lesser_or_equal.db";
        let path = std::path::Path::new(db_path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
        let mut catalog_setup = Catalog::new(db_path);
        catalog_setup.init().unwrap();
        let items = vec![
            Item {
                id: "item-1".to_string(),
                discount: 0.10,
                ..Item::default()
            },
            Item {
                id: "item-2".to_string(),
                discount: 0.20,
                ..Item::default()
            },
            Item {
                id: "item-3".to_string(),
                discount: 0.15,
                ..Item::default()
            },
        ];
        catalog_setup.insert_many(items).unwrap();
        catalog_setup.persist().unwrap();
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new().with_real("discount", Comparison::LesserOrEqual, 0.15);
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert_eq!(catalog.items.len(), 2);
        assert!(catalog.items.contains_key("item-1"));
        assert!(catalog.items.contains_key("item-3"));
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn load_by_filter_should_load_matching_discount_lesser_or_equal_edge_cases() {
        let db_path = "target/test_dbs/lbf_discount_lesser_or_equal_edge_cases.db";
        let path = std::path::Path::new(db_path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
        let mut catalog_setup = Catalog::new(db_path);
        catalog_setup.init().unwrap();
        let items = vec![
            Item {
                id: "item-1".to_string(),
                discount: 0.10,
                ..Item::default()
            },
            Item {
                id: "item-2".to_string(),
                discount: 0.20,
                ..Item::default()
            },
        ];
        catalog_setup.insert_many(items).unwrap();
        catalog_setup.persist().unwrap();
        let mut catalog1 = Catalog::new(db_path);
        let filter1 = Filter::new().with_real("discount", Comparison::LesserOrEqual, 0.10);
        assert!(catalog1.load_by_filter(&filter1).is_ok());
        assert_eq!(catalog1.items.len(), 1);
        assert!(catalog1.items.contains_key("item-1"));
        let mut catalog2 = Catalog::new(db_path);
        let filter2 = Filter::new().with_real("discount", Comparison::LesserOrEqual, 0.09);
        assert!(catalog2.load_by_filter(&filter2).is_ok());
        assert!(catalog2.items.is_empty());
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn load_by_filter_with_multiple_conditions() {
        let db_path = "target/test_dbs/lbf_with_multiple_conditions.db";
        let path = std::path::Path::new(db_path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
        let mut catalog_setup = Catalog::new(db_path);
        catalog_setup.init().unwrap();
        let items = vec![
            Item {
                id: "item-1".to_string(),
                price: 100,
                discount: 0.10,
                in_stock: true,
                ..Default::default()
            },
            Item {
                id: "item-2".to_string(),
                price: 200,
                discount: 0.10,
                in_stock: true,
                ..Default::default()
            },
            Item {
                id: "item-3".to_string(),
                price: 100,
                discount: 0.20,
                in_stock: true,
                ..Default::default()
            },
            Item {
                id: "item-4".to_string(),
                price: 100,
                discount: 0.10,
                in_stock: false,
                ..Default::default()
            },
        ];
        catalog_setup.insert_many(items).unwrap();
        catalog_setup.persist().unwrap();
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new()
            .with_unsigned_int("price", Comparison::Equal, 100)
            .with_real("discount", Comparison::Equal, 0.10)
            .with_bool("in_stock", true);
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert_eq!(catalog.items.len(), 1);
        assert!(catalog.items.contains_key("item-1"));
        std::fs::remove_file(path).unwrap();
    }
}
