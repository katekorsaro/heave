#[cfg(test)]
mod tests {
    use crate::*;
    use std::sync::*;
    use std::thread;
    #[test]
    fn thread_safety_test() {
        let db_path = "target/test_dbs/thread_safety_test.db";
        let path = std::path::Path::new(db_path);
        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        let catalog = Arc::new(Catalog::new(db_path));
        catalog.init().unwrap();
        let mut handles = vec![];
        for i in 0..10 {
            let catalog = Arc::clone(&catalog);
            let handle = thread::spawn(move || {
                for j in 0..100 {
                    let item_id = format!("item-{}-{}", i, j);
                    let item = Item {
                        id: item_id.clone(),
                        subclass: Some("subclass".to_string()),
                        name: format!("Item {} {}", i, j),
                        price: (i * 100 + j),
                        sell_trend: 0,
                        in_stock: true,
                        ..Item::default()
                    };
                    catalog.upsert(item).unwrap();
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
        let total_items = catalog.with_items(|items| Ok(items.len())).unwrap();
        assert_eq!(total_items, 1000);
        catalog.persist().unwrap();
        let mut new_catalog = Catalog::new(db_path);
        new_catalog.load_by_class::<Item>().unwrap();
        let total_items_after_load = new_catalog.with_items(|items| Ok(items.len())).unwrap();
        assert_eq!(total_items_after_load, 1000);
        std::fs::remove_file(path).unwrap();
    }
}
