#[cfg(test)]
mod tests {
    use crate::*;
    use std::collections::HashMap;
    #[test]
    fn for_each_should_iterate_over_all_items() {
        let catalog = Catalog::new("test.db");
        let item1 = Item {
            id: "1".to_string(),
            name: "Item 1".to_string(),
            price: 10,
            ..Default::default()
        };
        let item2 = Item {
            id: "2".to_string(),
            name: "Item 2".to_string(),
            price: 20,
            ..Default::default()
        };
        let item3 = Item {
            id: "3".to_string(),
            name: "Item 3".to_string(),
            price: 30,
            ..Default::default()
        };
        let _ = catalog.upsert(item1.clone());
        let _ = catalog.upsert(item2.clone());
        let _ = catalog.upsert(item3.clone());
        let mut collected_items = HashMap::new();
        catalog
            .for_each::<Item, _>(|item| {
                collected_items.insert(item.id.clone(), item.clone());
            })
            .unwrap();
        assert_eq!(collected_items.len(), 3);
        assert_eq!(collected_items.get("1").unwrap(), &item1);
        assert_eq!(collected_items.get("2").unwrap(), &item2);
        assert_eq!(collected_items.get("3").unwrap(), &item3);
    }
}
