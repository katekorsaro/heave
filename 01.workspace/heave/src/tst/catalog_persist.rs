#[cfg(test)]
mod tests {
    use crate::*;
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
        let catalog1 = Catalog::new(db_path);
        catalog1.init().unwrap();
        let item1 = Item {
            id: "item-1".to_string(),
            subclass: Some("subitem".to_string()),
            name: "Test Item".to_string(),
            price: 123,
            sell_trend: 0,
            in_stock: true,
            ..Item::default()
        };
        let _ = catalog1.upsert(item1.clone());
        assert!(catalog1.persist().is_ok());
        // 2. Create a new catalog and load the item to verify it was persisted
        let catalog2 = Catalog::new(db_path);
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
        let catalog1 = Catalog::new(db_path);
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
        catalog1.delete(&item1.id).unwrap();
        assert!(catalog1.persist().is_ok());
        // 3. Create a new catalog and try to load the deleted item.
        let catalog2 = Catalog::new(db_path);
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
        let catalog1 = Catalog::new(db_path);
        catalog1.init().unwrap();
        let original_item = Item {
            id: "item-1".to_string(),
            subclass: Some("subitem".to_string()),
            name: "Original Name".to_string(),
            price: 100,
            sell_trend: 0,
            in_stock: true,
            ..Item::default()
        };
        let _ = catalog1.upsert(original_item.clone());
        catalog1.persist().unwrap();
        // 2. Load it into a new catalog to simulate a separate session.
        let catalog2 = Catalog::new(db_path);
        catalog2.load_by_id("item-1").unwrap();
        // 3. Upsert updated data for the same item. This should mark it as 'Updated'.
        let updated_item = Item {
            id: "item-1".to_string(),
            subclass: Some("subitem".to_string()),
            name: "Updated Name".to_string(),
            price: 200,
            sell_trend: 0,
            in_stock: false,
            ..Item::default()
        };
        let _ = catalog2.upsert(updated_item.clone());
        assert_eq!(
            catalog2
                .with_items(|items| { Ok(items.get("item-1").unwrap().state) })
                .unwrap(),
            EntityState::Updated
        );
        // 4. Persist the changes.
        catalog2.persist().unwrap();
        // 5. Load the data into a third catalog to verify the update was written to the DB.
        let catalog3 = Catalog::new(db_path);
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
        let catalog_setup = Catalog::new(db_path);
        catalog_setup.init().unwrap();
        let item_to_update_original = Item {
            id: "update-me".to_string(),
            subclass: Some("subitem".to_string()),
            name: "Original".to_string(),
            price: 10,
            sell_trend: 0,
            in_stock: true,
            ..Item::default()
        };
        let item_to_delete = Item {
            id: "delete-me".to_string(),
            subclass: Some("subitem".to_string()),
            name: "Delete Me".to_string(),
            price: 20,
            sell_trend: 0,
            in_stock: true,
            ..Item::default()
        };
        let item_to_keep = Item {
            id: "keep-me".to_string(),
            subclass: Some("subitem".to_string()),
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
        let catalog_ops = Catalog::new(db_path);
        catalog_ops.load::<Item>().unwrap(); // Load all items
        // A new item to be inserted.
        let item_to_add = Item {
            id: "add-me".to_string(),
            subclass: Some("subitem".to_string()),
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
            subclass: Some("subitem".to_string()),
            name: "Updated".to_string(),
            price: 11,
            sell_trend: 0,
            in_stock: false,
            ..Item::default()
        };
        let _ = catalog_ops.upsert(item_to_update_new.clone()); // State: Updated
        // An item to be deleted.
        catalog_ops.delete("delete-me").unwrap(); // State: ToDelete
        // item_to_keep is left untouched (State: Synced after load)
        // 3. Execution: Persist all the changes in one go.
        catalog_ops.persist().unwrap();
        // 4. Verification: Load into a new catalog and check the final state of the DB.
        let catalog_verify = Catalog::new(db_path);
        catalog_verify.load::<Item>().unwrap();
        // Check total count
        assert_eq!(catalog_verify.len().unwrap(), 3);
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
        let catalog = Catalog::new(invalid_path);
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
        let catalog = Catalog::new(db_path);
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
        assert_eq!(catalog.len().unwrap(), 3);
        assert_eq!(
            catalog
                .with_items(|items| { Ok(items.get("update-me").unwrap().state) })
                .unwrap(),
            EntityState::Loaded
        );
        assert_eq!(
            catalog
                .with_items(|items| { Ok(items.get("delete-me").unwrap().state) })
                .unwrap(),
            EntityState::Loaded
        );
        assert_eq!(
            catalog
                .with_items(|items| { Ok(items.get("keep-me").unwrap().state) })
                .unwrap(),
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
        catalog.delete("delete-me").unwrap(); // State: ToDelete
        // 'item_untouched' remains with state `Loaded`.
        // Check states before final persist
        assert_eq!(
            catalog
                .with_items(|items| { Ok(items.get("add-me").unwrap().state) })
                .unwrap(),
            EntityState::New
        );
        assert_eq!(
            catalog
                .with_items(|items| { Ok(items.get("update-me").unwrap().state) })
                .unwrap(),
            EntityState::Updated
        );
        assert_eq!(
            catalog
                .with_items(|items| { Ok(items.get("delete-me").unwrap().state) })
                .unwrap(),
            EntityState::ToDelete
        );
        assert_eq!(
            catalog
                .with_items(|items| { Ok(items.get("keep-me").unwrap().state) })
                .unwrap(),
            EntityState::Loaded
        );
        assert_eq!(catalog.len().unwrap(), 4);
        // 3. Persist all changes.
        catalog.persist().unwrap();
        // 4. Verify the in-memory state after persisting.
        // The item marked for deletion should be gone.
        assert!(!catalog.contains_key("delete-me").unwrap());
        assert_eq!(catalog.len().unwrap(), 3);
        // All remaining items should have their state as `Loaded`.
        let new_item_entity = catalog
            .with_items(|items| Ok(items.get("add-me").unwrap().clone()))
            .unwrap();
        assert_eq!(new_item_entity.state, EntityState::Loaded);
        assert_eq!(
            new_item_entity.value_of("name"),
            Some(&Value::from("Add Me"))
        );
        let updated_item_entity = catalog
            .with_items(|items| Ok(items.get("update-me").unwrap().clone()))
            .unwrap();
        assert_eq!(updated_item_entity.state, EntityState::Loaded);
        assert_eq!(
            updated_item_entity.value_of("name"),
            Some(&Value::from("Updated"))
        );
        assert_eq!(
            updated_item_entity.value_of("sell_trend"),
            Some(&Value::from(10i64))
        );
        let untouched_item_entity = catalog
            .with_items(|items| Ok(items.get("keep-me").unwrap().clone()))
            .unwrap();
        assert_eq!(untouched_item_entity.state, EntityState::Loaded);
        assert_eq!(
            untouched_item_entity.value_of("name"),
            Some(&Value::from("Keep Me"))
        );
        // Clean up
        std::fs::remove_file(path).unwrap();
    }
}
