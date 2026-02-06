#[cfg(test)]
mod tests {
    use crate::*;
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
        let catalog1 = Catalog::new(db_path);
        catalog1.init().unwrap();
        let item1 = Item {
            id: "item-1".to_string(),
            subclass: Some("subitem".to_string()),
            name: "Item One".to_string(),
            price: 100,
            sell_trend: 0,
            in_stock: true,
            ..Item::default()
        };
        let item2 = Item {
            id: "item-2".to_string(),
            subclass: Some("subitem".to_string()),
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
        let catalog2 = Catalog::new(db_path);
        let result = catalog2.load::<Item>();
        assert!(result.is_ok());
        // 3. Verify that all items of that class were loaded
        let len = catalog2.len().unwrap();
        assert_eq!(len, 2);
        let loaded_item1: Item = catalog2.get("item-1").unwrap().unwrap();
        let loaded_item2: Item = catalog2.get("item-2").unwrap().unwrap();
        assert_eq!(loaded_item1, item1);
        assert_eq!(loaded_item2, item2);
        assert_eq!(
            catalog2
                .with_items(|items| { Ok(items.get("item-1").unwrap().state) })
                .unwrap(),
            EntityState::Loaded
        );
        assert_eq!(
            catalog2
                .with_items(|items| { Ok(items.get("item-2").unwrap().state) })
                .unwrap(),
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
        let catalog1 = Catalog::new(db_path);
        catalog1.init().unwrap();
        let item_in_db = Item {
            id: "item-1".to_string(),
            subclass: Some("subitem".to_string()),
            name: "DB Version".to_string(),
            price: 100,
            sell_trend: 0,
            in_stock: true,
            ..Item::default()
        };
        let _ = catalog1.upsert(item_in_db.clone());
        catalog1.persist().unwrap();
        // 2. Create a new catalog with a different in-memory version of the same item.
        let catalog2 = Catalog::new(db_path);
        let item_in_memory = Item {
            id: "item-1".to_string(),
            subclass: Some("subitem".to_string()),
            name: "Memory Version".to_string(),
            price: 200,
            sell_trend: 0,
            in_stock: false,
            ..Item::default()
        };
        let _ = catalog2.upsert(item_in_memory);
        let len = catalog2.len().unwrap();
        assert_eq!(len, 1);
        assert_eq!(
            catalog2.get::<Item>("item-1").unwrap().unwrap().name,
            "Memory Version"
        );
        // 3. Load from the database, which should overwrite the in-memory version.
        let result = catalog2.load::<Item>();
        assert!(result.is_ok());
        // 4. Verify that the in-memory entity has been replaced with the one from the DB.
        let len = catalog2.len().unwrap();
        assert_eq!(len, 1);
        let loaded_item: Item = catalog2.get("item-1").unwrap().unwrap();
        assert_eq!(loaded_item, item_in_db);
        assert_eq!(
            catalog2
                .with_items(|items| { Ok(items.get("item-1").unwrap().state) })
                .unwrap(),
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
        let catalog = Catalog::new(db_path);
        catalog.init().unwrap();
        // 2. Attempt to load from the empty DB.
        let result = catalog.load::<Item>();
        assert!(result.is_ok());
        let is_empty = catalog.is_empty().unwrap();
        assert!(is_empty);
        // 3. Add an item to memory and try loading again from the empty DB.
        let item_in_memory = Item {
            id: "item-1".to_string(),
            subclass: Some("subitem".to_string()),
            name: "In-memory only".to_string(),
            price: 100,
            sell_trend: 0,
            in_stock: true,
            ..Item::default()
        };
        let _ = catalog.upsert(item_in_memory.clone());
        let len = catalog.len().unwrap();
        assert_eq!(len, 1);
        let result2 = catalog.load::<Item>();
        assert!(result2.is_ok());
        // 4. Verify the in-memory item is untouched because nothing was loaded from DB.
        let len = catalog.len().unwrap();
        assert_eq!(len, 1);
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
        let catalog = Catalog::new(invalid_path);
        // Attempt to load from the invalid path.
        let result = catalog.load::<Item>();
        assert!(result.is_err());
        // Based on `load_by_id`, the error should be `LoadFromDB`.
        // This assumes `From<SqliteFailedTo>` is implemented to produce `FailedTo::LoadFromDB`.
        assert_eq!(result.unwrap_err(), FailedTo::LoadFromDB);
        // Clean up
        std::fs::remove_dir_all(invalid_path).unwrap();
    }
}
