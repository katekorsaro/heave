#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn insert_many_should_add_all_entities() {
        // 'insert_many()': Should add all provided entities to the 'items' map.
        let catalog = Catalog::new("dummy.db");
        let items = vec![
            Item {
                id: "item-1".to_string(),
                name: "Item 1".to_string(),
                price: 10,
                sell_trend: 0,
                in_stock: true,
                ..Item::default()
            },
            Item {
                id: "item-2".to_string(),
                name: "Item 2".to_string(),
                price: 20,
                sell_trend: 0,
                in_stock: false,
                ..Item::default()
            },
        ];
        let _ = catalog.insert_many(items);
        let len = catalog.len().unwrap();
        assert_eq!(len, 2);
        let entity1 = catalog
            .with_items(|items| {
                let entity = items.get("item-1").unwrap();
                Ok(entity.clone())
            })
            .unwrap();
        assert_eq!(entity1.state, EntityState::New);
        assert_eq!(entity1.value_of("name"), Some(&Value::from("Item 1")));
        assert_eq!(entity1.value_of("price"), Some(&Value::from(10u32)));
        assert_eq!(entity1.value_of("sell_trend"), Some(&Value::from(0i64)));
        let entity2 = catalog
            .with_items(|items| {
                let entity = items.get("item-2").unwrap();
                Ok(entity.clone())
            })
            .unwrap();
        assert_eq!(entity2.state, EntityState::New);
        assert_eq!(entity2.value_of("name"), Some(&Value::from("Item 2")));
        assert_eq!(entity2.value_of("price"), Some(&Value::from(20u32)));
        assert_eq!(entity2.value_of("sell_trend"), Some(&Value::from(0i64)));
    }
}
