use crate::*;

/// Represents a catalog of entities that can be persisted to a SQLite database.
///
/// The `Catalog` holds entities in memory and provides methods to interact with
/// them, as well as to persist changes to and load data from a database file.
#[derive(Debug, Default, PartialEq, Clone)]
pub struct O {
    path: String,
    pub(crate) items: std::collections::HashMap<String, Entity>,
}
impl Catalog {
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

    /// Schedules an entity for deletion. Actual delition will take place when 'persist' is called.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the entity to delete.
    pub fn delete(&mut self, id: &str) {
        let entity = self.items.get_mut(id);
        if let Some(entity) = entity {
            entity.state = EntityState::ToDelete;
        }
    }

    /// Persists the current state of the catalog to the database.
    ///
    /// - new entities will be written onto DB
    /// - marked for delition entities will be deleted
    pub fn persist(&self) -> result::Result<(), FailedTo> {
        let path = path::Path::new(&self.path);
        sqlite::persist::catalog(path, self).map_err(|_| FailedTo::PersistCatalog)?;
        Ok(())
    }

    /// Loads an entity by its ID from the database into the catalog.
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

    /// Loads all entities of a specific class from the database into the catalog.
    pub fn load_by_class<T>(&mut self) -> Result<(), FailedTo>
    where
        T: EAV,
    {
        let class = T::class();
        let path = path::Path::new(&self.path);
        let entities = sqlite::load::by_class(path, class)?;
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
                .with_attribute("in_stock", value.in_stock)
        }
    }
    impl From<Entity> for Item {
        fn from(entity: Entity) -> Self {
            Self {
                id: entity.id.clone(),
                name: entity.unwrap("name"),
                price: entity.unwrap("price"),
                in_stock: entity.unwrap("in_stock"),
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

    // ## 'insert()' & 'insert_many()'
    #[test]
    fn insert_should_add_single_entity_as_new() {
        // 'insert()': Should add a single entity to the 'items' map with 'EntityState::New'.
        let mut catalog = Catalog::new("dummy.db");
        let item = Item {
            id: "item-123".to_string(),
            name: "Test Item".to_string(),
            price: 100,
            in_stock: true,
        };
        let item_id = item.id.clone();
        catalog.insert(item);
        let entity = catalog.items.get(&item_id).unwrap();
        assert_eq!(entity.id, item_id);
        assert_eq!(entity.state, EntityState::New);
        assert_eq!(entity.class, "item");
        assert_eq!(entity.value_of("name"), Some(&Value::from("Test Item")));
        assert_eq!(entity.value_of("price"), Some(&Value::from(100u64)));
        assert_eq!(entity.value_of("in_stock"), Some(&Value::from(true)));
    }

    #[test]
    fn insert_should_overwrite_existing_entity() {
        // 'insert()': Should overwrite an existing entity with the same ID.
        let mut catalog = Catalog::new("dummy.db");
        let item1 = Item {
            id: "item-123".to_string(),
            name: "First Item".to_string(),
            price: 100,
            in_stock: true,
        };
        let item_id = item1.id.clone();
        catalog.insert(item1);
        let item2 = Item {
            id: "item-123".to_string(),
            name: "Second Item".to_string(),
            price: 200,
            in_stock: false,
        };
        catalog.insert(item2);
        assert_eq!(catalog.items.len(), 1);
        let entity = catalog.items.get(&item_id).unwrap();
        assert_eq!(entity.value_of("name"), Some(&Value::from("Second Item")));
        assert_eq!(entity.value_of("price"), Some(&Value::from(200u64)));
        assert_eq!(entity.value_of("in_stock"), Some(&Value::from(false)));
        assert_eq!(entity.state, EntityState::New);
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
                in_stock: true,
            },
            Item {
                id: "item-2".to_string(),
                name: "Item 2".to_string(),
                price: 20,
                in_stock: false,
            },
        ];
        catalog.insert_many(items);
        assert_eq!(catalog.items.len(), 2);
        let entity1 = catalog.items.get("item-1").unwrap();
        assert_eq!(entity1.state, EntityState::New);
        assert_eq!(entity1.value_of("name"), Some(&Value::from("Item 1")));
        let entity2 = catalog.items.get("item-2").unwrap();
        assert_eq!(entity2.state, EntityState::New);
        assert_eq!(entity2.value_of("name"), Some(&Value::from("Item 2")));
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
            in_stock: true,
        };
        catalog.insert(item.clone());
        let retrieved_item: Option<Item> = catalog.get("item-123");
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
            in_stock: true,
        };
        catalog.insert(item.clone());
        let retrieved_item: Option<Item> = catalog.get("nonexistent-id");
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
            in_stock: true,
        };
        let item2 = Item {
            id: "item-2".to_string(),
            name: "Unique Item".to_string(),
            price: 200,
            in_stock: false,
        };
        catalog.insert(item1.clone());
        catalog.insert(item2.clone());
        let retrieved_item: Option<Item> =
            catalog.get_by_class_and_attribute("name", "Unique Item");
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
            in_stock: true,
        };
        let item2 = Item {
            id: "item-2".to_string(),
            name: "Item Two".to_string(),
            price: 250,
            in_stock: false,
        };
        catalog.insert(item1.clone());
        catalog.insert(item2.clone());
        // Test with &str for String attribute
        let retrieved_by_name: Option<Item> =
            catalog.get_by_class_and_attribute("name", "Item One");
        assert_eq!(retrieved_by_name, Some(item1.clone()));
        // Test with u64 for price attribute
        let retrieved_by_price: Option<Item> = catalog.get_by_class_and_attribute("price", 250u64);
        assert_eq!(retrieved_by_price, Some(item2.clone()));
        // Test with bool for in_stock attribute
        let retrieved_by_stock: Option<Item> = catalog.get_by_class_and_attribute("in_stock", true);
        assert_eq!(retrieved_by_stock, Some(item1.clone()));
    }

    #[test]
    fn get_by_class_and_attribute_should_return_none_if_no_match() {
        // Should return 'None' if no entity matches the criteria.
        let mut catalog = Catalog::new("dummy.db");
        let item = Item {
            id: "item-1".to_string(),
            name: "Test Item".to_string(),
            price: 100,
            in_stock: true,
        };
        catalog.insert(item.clone());
        // Test with a value that doesn't exist
        let retrieved_item: Option<Item> =
            catalog.get_by_class_and_attribute("name", "Non-existent Name");
        assert!(retrieved_item.is_none());
        // Test with an attribute that doesn't exist
        let retrieved_item_2: Option<Item> =
            catalog.get_by_class_and_attribute("non-existent-attribute", "Test Item");
        assert!(retrieved_item_2.is_none());
    }

    // ## 'list_by_class()'
    #[test]
    fn list_by_class_should_return_all_entities_of_class() {
        // Should return an iterator with all entities of a specific class.
        todo!();
    }

    #[test]
    fn list_by_class_should_return_empty_iterator_if_no_match() {
        // Should return an empty iterator if no entities of that class exist.
        todo!();
    }

    // ## 'list_by_class_and_attribute()'
    #[test]
    fn list_by_class_and_attribute_should_return_all_matching_entities() {
        // Should return an iterator with all entities matching the class, attribute, and value.
        todo!();
    }

    #[test]
    fn list_by_class_and_attribute_should_return_empty_iterator_if_no_match() {
        // Should return an empty iterator if no entities match.
        todo!();
    }

    // ## 'delete()'
    #[test]
    fn delete_should_mark_entity_as_to_delete() {
        // Should mark an existing entity's state as 'ToDelete'.
        todo!();
    }

    #[test]
    fn delete_should_have_no_effect_for_nonexistent_id() {
        // Should have no effect if the entity ID does not exist.
        todo!();
    }

    // ## 'persist()'
    #[test]
    fn persist_should_insert_new_entities() {
        // Should insert entities with 'EntityState::New' into the database.
        todo!();
    }

    #[test]
    fn persist_should_delete_to_delete_entities() {
        // Should delete entities with 'EntityState::ToDelete' from the database.
        todo!();
    }

    #[test]
    fn persist_should_update_updated_entities() {
        // Should update entities with 'EntityState::Updated' in the database (Note: The current implementation doesn't seem to set 'Updated' state, this might be a future enhancement).
        todo!();
    }

    #[test]
    fn persist_should_handle_mixed_entity_states() {
        // Should handle a mix of new, updated, and deleted entities in one operation.
        todo!();
    }

    #[test]
    fn persist_should_return_error_on_db_failure() {
        // Should return an error if the database connection fails or a query fails.
        todo!();
    }

    #[test]
    fn persist_should_update_in_memory_state() {
        // After persisting, the in-memory state of entities should be considered. (e.g., should deleted items be removed from the 'items' map?).
        todo!();
    }

    // ## 'load_by_id()'
    #[test]
    fn load_by_id_should_load_entity_from_db() {
        // Should load a single entity from the database into the 'items' map.
        todo!();
    }

    #[test]
    fn load_by_id_should_overwrite_in_memory_entity() {
        // Should overwrite an existing in-memory entity with the same ID.
        todo!();
    }

    #[test]
    fn load_by_id_should_do_nothing_if_not_found() {
        // Should do nothing if the entity is not found in the database.
        todo!();
    }

    #[test]
    fn load_by_id_should_return_error_on_db_failure() {
        // Should return an error if the database operation fails.
        todo!();
    }

    // ## 'load_by_class()'
    #[test]
    fn load_by_class_should_load_all_entities_of_class() {
        // Should load all entities of a given class from the database into the 'items' map.
        todo!();
    }

    #[test]
    fn load_by_class_should_overwrite_in_memory_entities() {
        // Should overwrite any existing in-memory entities with the same IDs.
        todo!();
    }

    #[test]
    fn load_by_class_should_do_nothing_if_none_found() {
        // Should do nothing if no entities of that class are found.
        todo!();
    }

    #[test]
    fn load_by_class_should_return_error_on_db_failure() {
        // Should return an error if the database operation fails.
        todo!();
    }

    // ## Integration Tests
    #[test]
    fn integration_test_init_insert_persist_load_get() {
        // Scenario: 'init' -> 'insert' -> 'persist' -> create a new catalog instance -> 'load_by_id' -> 'get' -> verify data integrity.
        todo!();
    }

    #[test]
    fn integration_test_init_insert_many_persist_load_list() {
        // Scenario: 'init' -> 'insert_many' -> 'persist' -> new catalog -> 'load_by_class' -> 'list_by_class' -> verify all items are loaded.
        todo!();
    }

    #[test]
    fn integration_test_insert_persist_load_delete_persist_load() {
        // Scenario: 'insert' -> 'persist' -> 'load_by_id' -> 'delete' -> 'persist' -> 'load_by_id' should now return nothing for the deleted ID.
        todo!();
    }

    #[test]
    fn integration_test_insert_get_by_class_and_attribute() {
        // Scenario: 'insert' -> 'get_by_class_and_attribute' should return the correct item.
        todo!();
    }

    #[test]
    fn integration_test_insert_many_list_by_class_and_attribute() {
        // Scenario: 'insert_many' -> 'list_by_class_and_attribute' should return all matching items.
        todo!();
    }

    #[test]
    fn integration_test_concurrency() {
        // Scenario: Concurrency - what happens if two 'Catalog' instances point to the same file? (e.g., one reads while the other writes).
        todo!();
    }
}
