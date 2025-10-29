#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn get_should_retrieve_and_convert_entity_by_id() {
        // Should retrieve an entity by its ID and correctly convert it to the target type 'T'.
        let catalog = Catalog::new("dummy.db");
        let item = Item {
            id: "item-123".to_string(),
            subclass: Some("subitem".to_string()),
            name: "Test Item".to_string(),
            price: 100,
            sell_trend: 0,
            in_stock: true,
            ..Item::default()
        };
        let _ = catalog.upsert(item.clone());
        let retrieved_item: Option<Item> = catalog.get::<Item>("item-123").unwrap();
        assert_eq!(retrieved_item, Some(item));
    }
    #[test]
    fn get_should_return_none_for_nonexistent_id() {
        // Should return 'None' if the ID does not exist.
        let catalog = Catalog::new("dummy.db");
        let item = Item {
            id: "item-123".to_string(),
            name: "Test Item".to_string(),
            price: 100,
            sell_trend: 0,
            in_stock: true,
            ..Item::default()
        };
        let _ = catalog.upsert(item.clone());
        let retrieved_item: Option<Item> = catalog.get("nonexistent-id").unwrap();
        assert!(retrieved_item.is_none());
    }
}
