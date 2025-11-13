#[cfg(test)]
mod tests {
    use crate::*;
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
        let catalog1 = Catalog::new(db_path);
        catalog1.init().unwrap();
        let item_to_insert = Item {
            id: "item-1".to_string(),
            subclass: Some("subitem".to_string()),
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
                subclass: Some("subitem".to_string()),
                name: "Item One".to_string(),
                price: 100,
                sell_trend: 0,
                in_stock: true,
                ..Item::default()
            },
            Item {
                id: "item-2".to_string(),
                subclass: Some("subitem".to_string()),
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
        catalog2.load::<Item>().unwrap();
        let mut loaded_items: Vec<Item> = catalog2
            .list_by_class::<Item>()
            .unwrap()
            .into_iter()
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
        let catalog1 = Catalog::new(db_path);
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
        let catalog_setup = Catalog::new(db_path);
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
}
