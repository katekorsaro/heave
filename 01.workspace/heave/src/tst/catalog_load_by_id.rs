#[cfg(test)]
mod tests {
    use crate::*;
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
            subclass: Some("subitem".to_string()),
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
        assert!(catalog2.is_empty().unwrap());
        // 3. Load the item by its ID.
        let result = catalog2.load_by_id("item-1");
        assert!(result.is_ok());
        // 4. Verify the item is now in the in-memory 'items' map.
        let len = catalog2.len().unwrap();
        assert_eq!(len, 1);
        let loaded_entity = catalog2
            .with_items(|items| Ok(items.get("item-1").unwrap().clone()))
            .unwrap();
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
            subclass: Some("subitem".to_string()),
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
            subclass: Some("subitem".to_string()),
            name: "In-memory version".to_string(),
            price: 200,
            sell_trend: 0,
            in_stock: false,
            ..Item::default()
        };
        let _ = catalog2.upsert(item_in_memory);
        let entity_before_load = catalog2
            .with_items(|items| Ok(items.get("item-1").unwrap().clone()))
            .unwrap();
        assert_eq!(entity_before_load.state, EntityState::New);
        assert_eq!(
            entity_before_load.value_of("name"),
            Some(&Value::from("In-memory version"))
        );
        // 3. Load the item from the database, which should overwrite the in-memory version.
        let result = catalog2.load_by_id("item-1");
        assert!(result.is_ok());
        // 4. Verify that the in-memory entity has been replaced with the one from the DB.
        let entity_after_load = catalog2
            .with_items(|items| Ok(items.get("item-1").unwrap().clone()))
            .unwrap();
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
        assert!(catalog.is_empty().unwrap());
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
}
