#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn upsert_should_add_single_entity_as_new() {
        // 'upsert()': Should add a single entity to the 'items' map with 'EntityState::New'.
        let catalog = Catalog::new("dummy.db");
        let item = Item {
            id: "item-123".to_string(),
            name: "Test Item".to_string(),
            price: 100,
            sell_trend: 0,
            in_stock: true,
            ..Item::default()
        };
        let item_id = item.id.clone();
        let _ = catalog.upsert(item);
        let entity = catalog
            .with_items(|items| Ok(items.get(&item_id).unwrap().clone()))
            .unwrap();
        assert_eq!(entity.id, item_id);
        assert_eq!(entity.state, EntityState::New);
        assert_eq!(entity.class, "item");
        assert_eq!(entity.value_of("name"), Some(&Value::from("Test Item")));
        assert_eq!(entity.value_of("price"), Some(&Value::from(100u64)));
        assert_eq!(entity.value_of("sell_trend"), Some(&Value::from(0i64)));
        assert_eq!(entity.value_of("in_stock"), Some(&Value::from(true)));
    }
    #[test]
    fn upsert_should_overwrite_existing_entity() {
        // 'upsert()': Should overwrite an existing entity with the same ID.
        let catalog = Catalog::new("dummy.db");
        let item1 = Item {
            id: "item-123".to_string(),
            name: "First Item".to_string(),
            price: 100,
            sell_trend: 0,
            in_stock: true,
            ..Item::default()
        };
        let item_id = item1.id.clone();
        let _ = catalog.upsert(item1);
        let item2 = Item {
            id: "item-123".to_string(),
            name: "Second Item".to_string(),
            price: 200,
            sell_trend: 10,
            in_stock: false,
            ..Item::default()
        };
        let _ = catalog.upsert(item2);
        assert_eq!(catalog.len().unwrap(), 1);
        let entity = catalog
            .with_items(|items| Ok(items.get(&item_id).unwrap().clone()))
            .unwrap();
        assert_eq!(entity.value_of("name"), Some(&Value::from("Second Item")));
        assert_eq!(entity.value_of("price"), Some(&Value::from(200u64)));
        assert_eq!(entity.value_of("sell_trend"), Some(&Value::from(10i64)));
        assert_eq!(entity.value_of("in_stock"), Some(&Value::from(false)));
        assert_eq!(entity.state, EntityState::Updated);
    }
}
