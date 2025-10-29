#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn delete_should_mark_entity_as_to_delete() {
        // Should mark an existing entity's state as 'ToDelete'.
        let mut catalog = Catalog::new("dummy.db");
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
        catalog.delete(&item_id);
        let is_deleted = catalog
            .with_items(|items| {
                let entity = items.get(&item_id).unwrap();
                Ok(entity.state == EntityState::ToDelete)
            })
            .unwrap();
        assert!(is_deleted);
    }
    #[test]
    fn delete_should_have_no_effect_for_nonexistent_id() {
        // Should have no effect if the entity ID does not exist.
        let mut catalog = Catalog::new("dummy.db");
        let item = Item {
            id: "item-123".to_string(),
            name: "Test Item".to_string(),
            price: 100,
            sell_trend: 0,
            in_stock: true,
            ..Item::default()
        };
        let _ = catalog.upsert(item);
        // Attempt to delete a non-existent entity, which should not panic or change anything.
        catalog.delete("nonexistent-id");
        let not_deleted = catalog
            .with_items(|items| {
                let entity = items.get("item-123").unwrap();
                Ok(entity.state != EntityState::ToDelete)
            })
            .unwrap();
        assert!(not_deleted);
    }
}
