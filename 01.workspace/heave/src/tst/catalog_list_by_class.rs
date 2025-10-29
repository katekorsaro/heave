#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn list_by_class_should_return_all_entities_of_class() {
        // Should return an iterator with all entities of a specific class.
        let catalog = Catalog::new("dummy.db");
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
        let _ = catalog.upsert(item1.clone());
        let _ = catalog.upsert(item2.clone());
        let results = catalog.list_by_class::<Item>().unwrap();
        assert_eq!(results.len(), 2);
        assert!(results.contains(&Ok(item1)));
        assert!(results.contains(&Ok(item2)));
    }
    #[test]
    fn list_by_class_should_return_empty_iterator_if_no_match() {
        // Should return an empty iterator if no entities of that class exist.
        let catalog = Catalog::new("dummy.db");
        let results: Vec<_> = catalog.list_by_class::<Item>().unwrap();
        assert!(results.is_empty());
    }
}
