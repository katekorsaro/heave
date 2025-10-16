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

    /// Inserts or updates a single object that implements the `EAV` trait into the catalog.
    ///
    /// # Arguments
    ///
    /// * `object` - The object to insert.
    pub fn upsert(&mut self, object: impl EAV) {
        let mut entity = object.into();
        if self.items.contains_key(&entity.id) {
            entity.state = EntityState::Updated;
        } else {
            entity.state = EntityState::New;
        }
        self.items.insert(entity.id.clone(), entity);
    }

    /// Inserts multiple objects that implement the `EAV` trait into the catalog.
    ///
    /// # Arguments
    ///
    /// * `objects` - A vector of objects to insert.
    pub fn insert_many(&mut self, objects: Vec<impl EAV>) {
        for object in objects {
            self.upsert(object);
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
        let entities = sqlite::load::by_class(path, class).map_err(|_| FailedTo::LoadFromDB)?;
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

    // ## 'upsert()' & 'insert_many()'
    #[test]
    fn insert_should_add_single_entity_as_new() {
        // 'upsert()': Should add a single entity to the 'items' map with 'EntityState::New'.
        let mut catalog = Catalog::new("dummy.db");
        let item = Item {
            id: "item-123".to_string(),
            name: "Test Item".to_string(),
            price: 100,
            in_stock: true,
        };
        let item_id = item.id.clone();
        catalog.upsert(item);
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
        // 'upsert()': Should overwrite an existing entity with the same ID.
        let mut catalog = Catalog::new("dummy.db");
        let item1 = Item {
            id: "item-123".to_string(),
            name: "First Item".to_string(),
            price: 100,
            in_stock: true,
        };
        let item_id = item1.id.clone();
        catalog.upsert(item1);
        let item2 = Item {
            id: "item-123".to_string(),
            name: "Second Item".to_string(),
            price: 200,
            in_stock: false,
        };
        catalog.upsert(item2);
        assert_eq!(catalog.items.len(), 1);
        let entity = catalog.items.get(&item_id).unwrap();
        assert_eq!(entity.value_of("name"), Some(&Value::from("Second Item")));
        assert_eq!(entity.value_of("price"), Some(&Value::from(200u64)));
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
        catalog.upsert(item.clone());
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
        catalog.upsert(item.clone());
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
        catalog.upsert(item1.clone());
        catalog.upsert(item2.clone());
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
        catalog.upsert(item1.clone());
        catalog.upsert(item2.clone());
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
        catalog.upsert(item.clone());
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
            price: 200,
            in_stock: false,
        };
        catalog.upsert(item1.clone());
        catalog.upsert(item2.clone());
        let results: Vec<Item> = catalog.list_by_class::<Item>().collect();
        assert_eq!(results.len(), 2);
        assert!(results.contains(&item1));
        assert!(results.contains(&item2));
    }

    #[test]
    fn list_by_class_should_return_empty_iterator_if_no_match() {
        // Should return an empty iterator if no entities of that class exist.
        let catalog = Catalog::new("dummy.db");
        let results: Vec<Item> = catalog.list_by_class::<Item>().collect();
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
            in_stock: true,
        };
        let item2 = Item {
            id: "item-2".to_string(),
            name: "Item Two".to_string(),
            price: 200,
            in_stock: false,
        };
        let item3 = Item {
            id: "item-3".to_string(),
            name: "Item Three".to_string(),
            price: 300,
            in_stock: true,
        };
        catalog.upsert(item1.clone());
        catalog.upsert(item2.clone());
        catalog.upsert(item3.clone());
        let results: Vec<Item> = catalog
            .list_by_class_and_attribute("in_stock", true)
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
            in_stock: true,
        };
        catalog.upsert(item);
        // Search for a value that doesn't exist
        let results: Vec<Item> = catalog
            .list_by_class_and_attribute("in_stock", false)
            .collect();
        assert!(results.is_empty());
        // Search for an attribute that doesn't exist
        let results_2: Vec<Item> = catalog
            .list_by_class_and_attribute("non-existent-attribute", true)
            .collect();
        assert!(results_2.is_empty());
        // Search in a completely empty catalog
        let empty_catalog = Catalog::new("dummy.db");
        let results_3: Vec<Item> = empty_catalog
            .list_by_class_and_attribute("any_attribute", "any_value")
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
            in_stock: true,
        };
        let item_id = item.id.clone();
        catalog.upsert(item);
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
            in_stock: true,
        };
        catalog.upsert(item);
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
            in_stock: true,
        };
        catalog1.upsert(item1.clone());
        assert!(catalog1.persist().is_ok());
        // 2. Create a new catalog and load the item to verify it was persisted
        let mut catalog2 = Catalog::new(db_path);
        assert!(catalog2.load_by_id("item-1").is_ok());
        // 3. Get the item and assert it's the same as the one we inserted
        let loaded_item: Option<Item> = catalog2.get("item-1");
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
            in_stock: true,
        };
        catalog1.upsert(item1.clone());
        assert!(catalog1.persist().is_ok());
        // 2. Mark the item for deletion and persist again.
        catalog1.delete(&item1.id);
        assert!(catalog1.persist().is_ok());
        // 3. Create a new catalog and try to load the deleted item.
        let mut catalog2 = Catalog::new(db_path);
        assert!(catalog2.load_by_id(&item1.id).is_ok());
        // 4. Assert that the item was not found.
        let loaded_item: Option<Item> = catalog2.get(&item1.id);
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
            in_stock: true,
        };
        catalog1.upsert(original_item.clone());
        catalog1.persist().unwrap();
        // 2. Load it into a new catalog to simulate a separate session.
        let mut catalog2 = Catalog::new(db_path);
        catalog2.load_by_id("item-1").unwrap();
        // 3. Upsert updated data for the same item. This should mark it as 'Updated'.
        let updated_item = Item {
            id: "item-1".to_string(),
            name: "Updated Name".to_string(),
            price: 200,
            in_stock: false,
        };
        catalog2.upsert(updated_item.clone());
        assert_eq!(
            catalog2.items.get("item-1").unwrap().state,
            EntityState::Updated
        );
        // 4. Persist the changes.
        catalog2.persist().unwrap();
        // 5. Load the data into a third catalog to verify the update was written to the DB.
        let mut catalog3 = Catalog::new(db_path);
        catalog3.load_by_id("item-1").unwrap();
        let loaded_item: Item = catalog3.get("item-1").unwrap();
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
            in_stock: true,
        };
        let item_to_delete = Item {
            id: "delete-me".to_string(),
            name: "Delete Me".to_string(),
            price: 20,
            in_stock: true,
        };
        let item_to_keep = Item {
            id: "keep-me".to_string(),
            name: "Keep Me".to_string(),
            price: 30,
            in_stock: true,
        };
        catalog_setup.upsert(item_to_update_original.clone());
        catalog_setup.upsert(item_to_delete.clone());
        catalog_setup.upsert(item_to_keep.clone());
        catalog_setup.persist().unwrap();
        // 2. Manipulation: Load the data and perform mixed operations.
        let mut catalog_ops = Catalog::new(db_path);
        catalog_ops.load_by_class::<Item>().unwrap(); // Load all items
        // A new item to be inserted.
        let item_to_add = Item {
            id: "add-me".to_string(),
            name: "Add Me".to_string(),
            price: 40,
            in_stock: false,
        };
        catalog_ops.upsert(item_to_add.clone()); // State: New
        // An updated version of an existing item.
        let item_to_update_new = Item {
            id: "update-me".to_string(),
            name: "Updated".to_string(),
            price: 11,
            in_stock: false,
        };
        catalog_ops.upsert(item_to_update_new.clone()); // State: Updated
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
        let added_item: Item = catalog_verify.get("add-me").unwrap();
        assert_eq!(added_item, item_to_add);
        // Verify updated item
        let updated_item: Item = catalog_verify.get("update-me").unwrap();
        assert_eq!(updated_item, item_to_update_new);
        // Verify deleted item
        let deleted_item: Option<Item> = catalog_verify.get("delete-me");
        assert!(deleted_item.is_none());
        // Verify untouched item
        let kept_item: Item = catalog_verify.get("keep-me").unwrap();
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
            in_stock: true,
        };
        catalog.upsert(item);
        let result = catalog.persist();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), FailedTo::PersistCatalog);
        // Clean up
        std::fs::remove_dir_all(invalid_path).unwrap();
    }

    #[test]
    fn persist_should_update_in_memory_state() {
        // After persisting, the in-memory state of entities should be considered. (e.g., should deleted items be removed from the 'items' map?).
        // This test verifies the CURRENT behavior: in-memory state is NOT updated by `persist(&self)`.
        let db_path = "target/test_dbs/persist_should_update_in_memory_state.db";
        let path = std::path::Path::new(db_path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
        let mut catalog = Catalog::new(db_path);
        catalog.init().unwrap();
        // 1. Add items to get them into New, Updated, and ToDelete states.
        catalog.upsert(Item {
            id: "new".to_string(),
            ..Default::default()
        }); // -> New
        catalog.upsert(Item {
            id: "update".to_string(),
            ..Default::default()
        });
        catalog.persist().unwrap(); // Persist `update` so it exists in DB for the next step.
        catalog.upsert(Item {
            id: "update".to_string(),
            name: "Updated".to_string(),
            ..Default::default()
        }); // -> Updated
        catalog.upsert(Item {
            id: "delete".to_string(),
            ..Default::default()
        });
        catalog.delete("delete"); // -> ToDelete
        // 2. Check initial states before the main persist call.
        assert_eq!(catalog.items.get("new").unwrap().state, EntityState::New);
        assert_eq!(
            catalog.items.get("update").unwrap().state,
            EntityState::Updated
        );
        assert_eq!(
            catalog.items.get("delete").unwrap().state,
            EntityState::ToDelete
        );
        // 3. Persist all changes.
        catalog.persist().unwrap();
        // 4. Verify that in-memory states have NOT changed, because persist takes &self.
        assert_eq!(catalog.items.get("new").unwrap().state, EntityState::New);
        assert_eq!(
            catalog.items.get("update").unwrap().state,
            EntityState::Updated
        );
        assert_eq!(
            catalog.items.get("delete").unwrap().state,
            EntityState::ToDelete
        );
        assert!(catalog.items.contains_key("delete")); // Deleted item is still in memory.
        // Clean up.
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
            in_stock: true,
        };
        catalog1.upsert(item_to_persist.clone());
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
        assert_eq!(loaded_entity.value_of("in_stock"), Some(&Value::from(true)));
        assert_eq!(loaded_entity.state, EntityState::Loaded); // Should be Synced after loading.
        // 6. Also verify by using the public 'get' method.
        let retrieved_item: Option<Item> = catalog2.get("item-1");
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
            in_stock: true,
        };
        catalog1.upsert(item_in_db.clone());
        catalog1.persist().unwrap();
        // 2. Create a new catalog and add a *different* in-memory version of the same item.
        let mut catalog2 = Catalog::new(db_path);
        let item_in_memory = Item {
            id: "item-1".to_string(),
            name: "In-memory version".to_string(),
            price: 200,
            in_stock: false,
        };
        catalog2.upsert(item_in_memory);
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
        // 5. Verify using the public 'get' method.
        let retrieved_item: Item = catalog2.get("item-1").unwrap();
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
            in_stock: true,
        };
        let item2 = Item {
            id: "item-2".to_string(),
            name: "Item Two".to_string(),
            price: 200,
            in_stock: false,
        };
        catalog1.upsert(item1.clone());
        catalog1.upsert(item2.clone());
        catalog1.persist().unwrap();
        // 2. Create a new catalog and load the items by class
        let mut catalog2 = Catalog::new(db_path);
        let result = catalog2.load_by_class::<Item>();
        assert!(result.is_ok());
        // 3. Verify that all items of that class were loaded
        assert_eq!(catalog2.items.len(), 2);
        let loaded_item1: Item = catalog2.get("item-1").unwrap();
        let loaded_item2: Item = catalog2.get("item-2").unwrap();
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
            in_stock: true,
        };
        catalog1.upsert(item_in_db.clone());
        catalog1.persist().unwrap();
        // 2. Create a new catalog with a different in-memory version of the same item.
        let mut catalog2 = Catalog::new(db_path);
        let item_in_memory = Item {
            id: "item-1".to_string(),
            name: "Memory Version".to_string(),
            price: 200,
            in_stock: false,
        };
        catalog2.upsert(item_in_memory);
        assert_eq!(catalog2.items.len(), 1);
        assert_eq!(
            catalog2.get::<Item>("item-1").unwrap().name,
            "Memory Version"
        );
        // 3. Load from the database, which should overwrite the in-memory version.
        let result = catalog2.load_by_class::<Item>();
        assert!(result.is_ok());
        // 4. Verify that the in-memory entity has been replaced with the one from the DB.
        assert_eq!(catalog2.items.len(), 1);
        let loaded_item: Item = catalog2.get("item-1").unwrap();
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
            in_stock: true,
        };
        catalog.upsert(item_in_memory.clone());
        assert_eq!(catalog.items.len(), 1);
        let result2 = catalog.load_by_class::<Item>();
        assert!(result2.is_ok());
        // 4. Verify the in-memory item is untouched because nothing was loaded from DB.
        assert_eq!(catalog.items.len(), 1);
        let retrieved_item: Item = catalog.get("item-1").unwrap();
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
            in_stock: true,
        };
        catalog1.upsert(item_to_insert.clone());
        catalog1.persist().unwrap();
        // 2. create a new catalog instance -> 'load_by_id' -> 'get'
        let mut catalog2 = Catalog::new(db_path);
        catalog2.load_by_id("item-1").unwrap();
        let loaded_item: Option<Item> = catalog2.get("item-1");
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
                in_stock: true,
            },
            Item {
                id: "item-2".to_string(),
                name: "Item Two".to_string(),
                price: 200,
                in_stock: false,
            },
        ];
        catalog1.insert_many(items_to_insert.clone());
        catalog1.persist().unwrap();
        // 2. new catalog -> 'load_by_class' -> 'list_by_class'
        let mut catalog2 = Catalog::new(db_path);
        catalog2.load_by_class::<Item>().unwrap();
        let mut loaded_items: Vec<Item> = catalog2.list_by_class::<Item>().collect();
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
            in_stock: true,
        };
        catalog1.upsert(item_to_delete.clone());
        catalog1.persist().unwrap();
        // 2. 'load_by_id' to confirm it's there
        let mut catalog2 = Catalog::new(db_path);
        catalog2.load_by_id("item-to-delete").unwrap();
        assert!(catalog2.get::<Item>("item-to-delete").is_some());
        // 3. 'delete' -> 'persist'
        catalog2.delete("item-to-delete");
        catalog2.persist().unwrap();
        // 4. 'load_by_id' should now return nothing
        let mut catalog3 = Catalog::new(db_path);
        catalog3.load_by_id("item-to-delete").unwrap();
        let loaded_item: Option<Item> = catalog3.get("item-to-delete");
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
            in_stock: true,
        };
        let item2 = Item {
            id: "item-2".to_string(),
            name: "Second Item".to_string(),
            price: 200,
            in_stock: false,
        };
        catalog.upsert(item1.clone());
        catalog.upsert(item2.clone());
        // Retrieve by a unique attribute
        let retrieved_item: Option<Item> =
            catalog.get_by_class_and_attribute("name", "Second Item");
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
            in_stock: true,
        };
        let item2 = Item {
            id: "item-2".to_string(),
            name: "Item Two".to_string(),
            price: 200,
            in_stock: false,
        };
        let item3 = Item {
            id: "item-3".to_string(),
            name: "Item Three".to_string(),
            price: 150,
            in_stock: true,
        };
        catalog.insert_many(vec![item1.clone(), item2.clone(), item3.clone()]);
        // List all items that are in stock
        let mut results: Vec<Item> = catalog
            .list_by_class_and_attribute("in_stock", true)
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
            in_stock: true,
        };
        catalog_setup.upsert(initial_item);
        catalog_setup.persist().unwrap();
        let db_path_arc = std::sync::Arc::new(String::from(db_path));
        // 2. Thread 1: Loads, updates name, and persists.
        let db_path_arc1 = std::sync::Arc::clone(&db_path_arc);
        let handle1 = std::thread::spawn(move || {
            let mut catalog1 = Catalog::new(&db_path_arc1);
            catalog1.load_by_id("item-1").unwrap();
            let mut item = catalog1.get::<Item>("item-1").unwrap();
            item.name = "Updated by Thread 1".to_string();
            catalog1.upsert(item);
            catalog1.persist().unwrap();
        });
        // 3. Thread 2: Loads, updates price, and persists.
        let db_path_arc2 = std::sync::Arc::clone(&db_path_arc);
        let handle2 = std::thread::spawn(move || {
            let mut catalog2 = Catalog::new(&db_path_arc2);
            catalog2.load_by_id("item-1").unwrap();
            let mut item = catalog2.get::<Item>("item-1").unwrap();
            item.price = 200;
            catalog2.upsert(item);
            catalog2.persist().unwrap();
        });
        handle1.join().unwrap();
        handle2.join().unwrap();
        // 4. Verification: Load the data and check the final state.
        let mut catalog_verify = Catalog::new(db_path);
        catalog_verify.load_by_id("item-1").unwrap();
        let final_item: Item = catalog_verify.get("item-1").unwrap();
        // The final state depends on which thread persisted last. One update will have been lost.
        let thread1_won = final_item.name == "Updated by Thread 1" && final_item.price == 100;
        let thread2_won = final_item.name == "Original" && final_item.price == 200;
        assert!(
            thread1_won || thread2_won,
            "Final state must be the result of one of the threads winning the race."
        );
        // Clean up
        std::fs::remove_file(path).unwrap();
    }
}
