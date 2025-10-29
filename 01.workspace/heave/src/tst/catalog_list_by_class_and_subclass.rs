#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn list_by_class_and_subclass_should_return_matching_entities() {
        let catalog = Catalog::new("dummy.db");
        let item1 = Item {
            id: "item-1".to_string(),
            subclass: Some("electronics".to_string()),
            ..Default::default()
        };
        let item2 = Item {
            id: "item-2".to_string(),
            subclass: Some("books".to_string()),
            ..Default::default()
        };
        let item3 = Item {
            id: "item-3".to_string(),
            subclass: Some("electronics".to_string()),
            ..Default::default()
        };
        let _ = catalog.upsert(item1.clone());
        let _ = catalog.upsert(item2.clone());
        let _ = catalog.upsert(item3.clone());
        let results: Vec<_> = catalog
            .list_by_class_and_subclass::<Item>("electronics")
            .unwrap()
            .into_iter()
            .map(|item| item.unwrap())
            .collect();
        assert_eq!(results.len(), 2);
        assert!(results.contains(&item1));
        assert!(results.contains(&item3));
        assert!(!results.contains(&item2));
    }
    #[test]
    fn list_by_class_and_subclass_should_return_empty_if_no_match() {
        let catalog = Catalog::new("dummy.db");
        let item1 = Item {
            id: "item-1".to_string(),
            subclass: Some("electronics".to_string()),
            ..Default::default()
        };
        let _ = catalog.upsert(item1.clone());
        let results: Vec<_> = catalog
            .list_by_class_and_subclass::<Item>("books")
            .unwrap()
            .into_iter()
            .map(|item| item.unwrap())
            .collect();
        assert!(results.is_empty());
    }
}
