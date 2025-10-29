#[cfg(test)]
mod tests {
    use crate::*;
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
        assert_eq!(catalog.len().unwrap(), 2);
        assert!(catalog.contains_key("item-1").unwrap());
        assert!(catalog.contains_key("item-3").unwrap());
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
        assert_eq!(catalog.len().unwrap(), 1);
        assert!(catalog.contains_key("item-2").unwrap());
        assert!(!catalog.contains_key("item-1").unwrap());
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
        assert_eq!(catalog.len().unwrap(), 2);
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
            subclass: Some("subitem".to_string()),
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
            subclass: Some("subitem".to_string()),
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
        assert_eq!(catalog.len().unwrap(), 2);
        assert!(catalog.contains_key("item-1").unwrap());
        assert!(catalog.contains_key("item-3").unwrap());
        assert!(!catalog.contains_key("item-2").unwrap());
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
        assert_eq!(catalog.len().unwrap(), 1);
        assert!(catalog.contains_key("item-2").unwrap());
        assert!(!catalog.contains_key("item-1").unwrap());
        assert!(!catalog.contains_key("item-3").unwrap());
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
        assert_eq!(catalog.len().unwrap(), 2);
        assert!(catalog.contains_key("item-1").unwrap());
        assert!(!catalog.contains_key("item-2").unwrap());
        assert!(catalog.contains_key("item-3").unwrap());
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
        assert_eq!(catalog1.len().unwrap(), 1);
        assert!(catalog1.contains_key("item-3").unwrap());
        // Edge Case 2: No values greater than filter value.
        let mut catalog2 = Catalog::new(db_path);
        let filter2 = Filter::new().with_signed_int("sell_trend", Comparison::Greater, 20);
        assert!(catalog2.load_by_filter(&filter2).is_ok());
        assert!(catalog2.is_empty().unwrap());
        // Edge Case 3: All values greater than filter value.
        let mut catalog3 = Catalog::new(db_path);
        let filter3 = Filter::new().with_signed_int("sell_trend", Comparison::Greater, -10);
        assert!(catalog3.load_by_filter(&filter3).is_ok());
        assert_eq!(catalog3.len().unwrap(), 3);
        assert!(catalog3.contains_key("item-1").unwrap());
        assert!(catalog3.contains_key("item-2").unwrap());
        assert!(catalog3.contains_key("item-3").unwrap());
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
        assert_eq!(catalog.len().unwrap(), 2);
        assert!(catalog.contains_key("item-1").unwrap());
        assert!(catalog.contains_key("item-2").unwrap());
        assert!(!catalog.contains_key("item-3").unwrap());
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
        assert_eq!(catalog1.len().unwrap(), 1);
        assert!(catalog1.contains_key("item-2").unwrap());
        // Edge Case 2: No values lesser than filter value.
        let mut catalog2 = Catalog::new(db_path);
        let filter2 = Filter::new().with_signed_int("sell_trend", Comparison::Lesser, -5);
        assert!(catalog2.load_by_filter(&filter2).is_ok());
        assert!(catalog2.is_empty().unwrap());
        // Edge Case 3: All values lesser than filter value.
        let mut catalog3 = Catalog::new(db_path);
        let filter3 = Filter::new().with_signed_int("sell_trend", Comparison::Lesser, 25);
        assert!(catalog3.load_by_filter(&filter3).is_ok());
        assert_eq!(catalog3.len().unwrap(), 3);
        assert!(catalog3.contains_key("item-1").unwrap());
        assert!(catalog3.contains_key("item-2").unwrap());
        assert!(catalog3.contains_key("item-3").unwrap());
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
        assert_eq!(catalog.len().unwrap(), 2);
        assert!(catalog.contains_key("item-1").unwrap());
        assert!(!catalog.contains_key("item-2").unwrap());
        assert!(catalog.contains_key("item-3").unwrap());
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
        assert_eq!(catalog1.len().unwrap(), 1);
        assert!(catalog1.contains_key("item-3").unwrap());
        // Edge Case 2: No values greater than or equal to filter value.
        let mut catalog2 = Catalog::new(db_path);
        let filter2 = Filter::new().with_signed_int("sell_trend", Comparison::GreaterOrEqual, 21);
        assert!(catalog2.load_by_filter(&filter2).is_ok());
        assert!(catalog2.is_empty().unwrap());
        // Edge Case 3: All values greater than or equal to filter value.
        let mut catalog3 = Catalog::new(db_path);
        let filter3 = Filter::new().with_signed_int("sell_trend", Comparison::GreaterOrEqual, -5);
        assert!(catalog3.load_by_filter(&filter3).is_ok());
        assert_eq!(catalog3.len().unwrap(), 3);
        assert!(catalog3.contains_key("item-1").unwrap());
        assert!(catalog3.contains_key("item-2").unwrap());
        assert!(catalog3.contains_key("item-3").unwrap());
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
        assert_eq!(catalog.len().unwrap(), 2);
        assert!(catalog.contains_key("item-1").unwrap());
        assert!(catalog.contains_key("item-2").unwrap());
        assert!(!catalog.contains_key("item-3").unwrap());
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
        assert_eq!(catalog1.len().unwrap(), 1);
        assert!(catalog1.contains_key("item-2").unwrap());
        // Edge Case 2: No values lesser than or equal to filter value.
        let mut catalog2 = Catalog::new(db_path);
        let filter2 = Filter::new().with_signed_int("sell_trend", Comparison::LesserOrEqual, -6);
        assert!(catalog2.load_by_filter(&filter2).is_ok());
        assert!(catalog2.is_empty().unwrap());
        // Edge Case 3: All values lesser than or equal to filter value.
        let mut catalog3 = Catalog::new(db_path);
        let filter3 = Filter::new().with_signed_int("sell_trend", Comparison::LesserOrEqual, 20);
        assert!(catalog3.load_by_filter(&filter3).is_ok());
        assert_eq!(catalog3.len().unwrap(), 3);
        assert!(catalog3.contains_key("item-1").unwrap());
        assert!(catalog3.contains_key("item-2").unwrap());
        assert!(catalog3.contains_key("item-3").unwrap());
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
        assert_eq!(catalog.len().unwrap(), 1);
        assert!(catalog.contains_key("item-2").unwrap());
        // Greater
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new().with_unsigned_int("price", Comparison::Greater, 200);
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert_eq!(catalog.len().unwrap(), 1);
        assert!(catalog.contains_key("item-3").unwrap());
        // Lesser
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new().with_unsigned_int("price", Comparison::Lesser, 200);
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert_eq!(catalog.len().unwrap(), 1);
        assert!(catalog.contains_key("item-1").unwrap());
        // GreaterOrEqual
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new().with_unsigned_int("price", Comparison::GreaterOrEqual, 200);
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert_eq!(catalog.len().unwrap(), 2);
        assert!(catalog.contains_key("item-2").unwrap());
        assert!(catalog.contains_key("item-3").unwrap());
        // LesserOrEqual
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new().with_unsigned_int("price", Comparison::LesserOrEqual, 200);
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert_eq!(catalog.len().unwrap(), 2);
        assert!(catalog.contains_key("item-1").unwrap());
        assert!(catalog.contains_key("item-2").unwrap());
        // Edge case: No match
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new().with_unsigned_int("price", Comparison::Equal, 400);
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert!(catalog.is_empty().unwrap());
        // Edge case: All match
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new().with_unsigned_int("price", Comparison::GreaterOrEqual, 100);
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert_eq!(catalog.len().unwrap(), 3);
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
        assert_eq!(catalog.len().unwrap(), 1);
        assert!(catalog.contains_key("item-1").unwrap());
        // Partial match should not work
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new().with_text("name", Comparison::IsExactly, "Item");
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert!(catalog.is_empty().unwrap());
        // Case insensitive match
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new().with_text("name", Comparison::IsExactly, "item one");
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert_eq!(catalog.len().unwrap(), 1);
        assert!(catalog.contains_key("item-1").unwrap());
        // No match
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new().with_text("name", Comparison::IsExactly, "Item Four");
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert!(catalog.is_empty().unwrap());
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
        assert_eq!(catalog.len().unwrap(), 2);
        assert!(catalog.contains_key("item-1").unwrap());
        assert!(catalog.contains_key("item-2").unwrap());
        // Starts with "I"
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new().with_text("name", Comparison::StartsWith, "I");
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert_eq!(catalog.len().unwrap(), 2);
        assert!(catalog.contains_key("item-1").unwrap());
        assert!(catalog.contains_key("item-2").unwrap());
        // No match
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new().with_text("name", Comparison::StartsWith, "Z");
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert!(catalog.is_empty().unwrap());
        // Case insensitive
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new().with_text("name", Comparison::StartsWith, "item");
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert_eq!(catalog.len().unwrap(), 2);
        assert!(catalog.contains_key("item-1").unwrap());
        assert!(catalog.contains_key("item-2").unwrap());
        // Full string match
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new().with_text("name", Comparison::StartsWith, "Item One");
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert_eq!(catalog.len().unwrap(), 1);
        assert!(catalog.contains_key("item-1").unwrap());
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
        assert_eq!(catalog.len().unwrap(), 1);
        assert!(catalog.contains_key("item-1").unwrap());
        // Ends with "item" - case insensitive
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new().with_text("name", Comparison::EndsWith, "item");
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert_eq!(catalog.len().unwrap(), 1);
        assert!(catalog.contains_key("item-3").unwrap());
        // No match
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new().with_text("name", Comparison::EndsWith, "Z");
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert!(catalog.is_empty().unwrap());
        // Full string match - case insensitive
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new().with_text("name", Comparison::EndsWith, "item one");
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert_eq!(catalog.len().unwrap(), 1);
        assert!(catalog.contains_key("item-1").unwrap());
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
        assert_eq!(catalog.len().unwrap(), 3);
        // Contains "THE" - case insensitive
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new().with_text("name", Comparison::Contains, "THE");
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert_eq!(catalog.len().unwrap(), 1);
        assert!(catalog.contains_key("item-3").unwrap());
        // No match
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new().with_text("name", Comparison::Contains, "Z");
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert!(catalog.is_empty().unwrap());
        // Full string match - case insensitive
        let mut catalog = Catalog::new(db_path);
        let filter = Filter::new().with_text("name", Comparison::Contains, "item one");
        assert!(catalog.load_by_filter(&filter).is_ok());
        assert_eq!(catalog.len().unwrap(), 1);
        assert!(catalog.contains_key("item-1").unwrap());
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
        assert_eq!(catalog.len().unwrap(), 2);
        assert!(catalog.contains_key("item-1").unwrap());
        assert!(catalog.contains_key("item-3").unwrap());
        assert!(!catalog.contains_key("item-2").unwrap());
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
        assert_eq!(catalog.len().unwrap(), 2);
        assert!(catalog.contains_key("item-2").unwrap());
        assert!(catalog.contains_key("item-3").unwrap());
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
        assert_eq!(catalog1.len().unwrap(), 1);
        assert!(catalog1.contains_key("item-2").unwrap());
        let mut catalog2 = Catalog::new(db_path);
        let filter2 = Filter::new().with_real("discount", Comparison::Greater, 0.20);
        assert!(catalog2.load_by_filter(&filter2).is_ok());
        assert!(catalog2.is_empty().unwrap());
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
        assert_eq!(catalog.len().unwrap(), 2);
        assert!(catalog.contains_key("item-1").unwrap());
        assert!(catalog.contains_key("item-3").unwrap());
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
        assert_eq!(catalog1.len().unwrap(), 1);
        assert!(catalog1.contains_key("item-1").unwrap());
        let mut catalog2 = Catalog::new(db_path);
        let filter2 = Filter::new().with_real("discount", Comparison::Lesser, 0.10);
        assert!(catalog2.load_by_filter(&filter2).is_ok());
        assert!(catalog2.is_empty().unwrap());
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
        assert_eq!(catalog.len().unwrap(), 2);
        assert!(catalog.contains_key("item-2").unwrap());
        assert!(catalog.contains_key("item-3").unwrap());
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
        assert_eq!(catalog1.len().unwrap(), 1);
        assert!(catalog1.contains_key("item-2").unwrap());
        let mut catalog2 = Catalog::new(db_path);
        let filter2 = Filter::new().with_real("discount", Comparison::GreaterOrEqual, 0.21);
        assert!(catalog2.load_by_filter(&filter2).is_ok());
        assert!(catalog2.is_empty().unwrap());
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
        assert_eq!(catalog.len().unwrap(), 2);
        assert!(catalog.contains_key("item-1").unwrap());
        assert!(catalog.contains_key("item-3").unwrap());
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
        assert_eq!(catalog1.len().unwrap(), 1);
        assert!(catalog1.contains_key("item-1").unwrap());
        let mut catalog2 = Catalog::new(db_path);
        let filter2 = Filter::new().with_real("discount", Comparison::LesserOrEqual, 0.09);
        assert!(catalog2.load_by_filter(&filter2).is_ok());
        assert!(catalog2.is_empty().unwrap());
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
        assert_eq!(catalog.len().unwrap(), 1);
        assert!(catalog.contains_key("item-1").unwrap());
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn load_by_filter_with_class_and_subclass_clauses() {
        // Verifies that class and subclass clauses in a filter correctly load entities.
        let db_path = "target/test_dbs/lbf_with_class_subclass_clauses.db";
        let path = std::path::Path::new(db_path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
        let mut catalog_setup = Catalog::new(db_path);
        catalog_setup.init().unwrap();
        // Define a second struct with a different class
        #[derive(Debug, Default, PartialEq, Clone)]
        struct AnotherItem {
            pub id: String,
        }
        impl EAV for AnotherItem {
            fn class() -> &'static str {
                "another_item"
            }
        }
        impl From<AnotherItem> for Entity {
            fn from(value: AnotherItem) -> Entity {
                Entity::new::<AnotherItem>().with_id(&value.id)
            }
        }
        impl From<Entity> for AnotherItem {
            fn from(entity: Entity) -> Self {
                Self {
                    id: entity.id.clone(),
                }
            }
        }
        let item1 = Item {
            id: "item-1".to_string(),
            subclass: Some("sub-a".to_string()),
            ..Default::default()
        };
        let item2 = Item {
            id: "item-2".to_string(),
            subclass: Some("sub-b".to_string()),
            ..Default::default()
        };
        let item3 = Item {
            id: "item-3".to_string(),
            subclass: Some("sub-a".to_string()),
            ..Default::default()
        };
        let item4 = Item {
            id: "item-4".to_string(),
            subclass: None, // -> "subitem"
            ..Default::default()
        };
        let another_item = AnotherItem {
            id: "another-1".to_string(),
        };
        catalog_setup.upsert(item1).unwrap();
        catalog_setup.upsert(item2).unwrap();
        catalog_setup.upsert(item3).unwrap();
        catalog_setup.upsert(item4).unwrap();
        catalog_setup.upsert(another_item).unwrap();
        catalog_setup.persist().unwrap();
        // Test 1: Filter by class "item"
        let mut catalog1 = Catalog::new(db_path);
        let filter1 = Filter::new().with_class("item");
        assert!(catalog1.load_by_filter(&filter1).is_ok());
        assert_eq!(catalog1.len().unwrap(), 4);
        assert!(catalog1.contains_key("item-1").unwrap());
        assert!(catalog1.contains_key("item-2").unwrap());
        assert!(catalog1.contains_key("item-3").unwrap());
        assert!(catalog1.contains_key("item-4").unwrap());
        // Test 2: Filter by class "another_item"
        let mut catalog2 = Catalog::new(db_path);
        let filter2 = Filter::new().with_class("another_item");
        assert!(catalog2.load_by_filter(&filter2).is_ok());
        assert_eq!(catalog2.len().unwrap(), 1);
        assert!(catalog2.contains_key("another-1").unwrap());
        // Test 3: Filter by class "item" and subclass "sub-a"
        let mut catalog3 = Catalog::new(db_path);
        let filter3 = Filter::new().with_class("item").with_subclass("sub-a");
        assert!(catalog3.load_by_filter(&filter3).is_ok());
        assert_eq!(catalog3.len().unwrap(), 2);
        assert!(catalog3.contains_key("item-1").unwrap());
        assert!(catalog3.contains_key("item-3").unwrap());
        // Test 4: Filter by class "item" and subclass "subitem" (the default)
        let mut catalog4 = Catalog::new(db_path);
        let filter4 = Filter::new().with_class("item").with_subclass("subitem");
        assert!(catalog4.load_by_filter(&filter4).is_ok());
        assert_eq!(catalog4.len().unwrap(), 1);
        assert!(catalog4.contains_key("item-4").unwrap());
        std::fs::remove_file(path).unwrap();
    }
}
