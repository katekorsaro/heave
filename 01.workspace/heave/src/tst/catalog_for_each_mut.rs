#[cfg(test)]
mod tests {
    use crate::*;
    fn prepare_catalog(catalog: &mut Catalog) {
        let items = vec![
            Item {
                id: "1".to_string(),
                price: 100,
                ..Default::default()
            },
            Item {
                id: "2".to_string(),
                price: 200,
                ..Default::default()
            },
            Item {
                id: "3".to_string(),
                price: 300,
                ..Default::default()
            },
        ];
        let _ = catalog.insert_many(items);
    }
    #[test]
    fn all_prices_should_be_changed() {
        let mut catalog = Catalog::new("test.db");
        prepare_catalog(&mut catalog);
        let _ = catalog.for_each_mut(|item: &mut Item| {
            item.price += 50;
            Ok(())
        });
        let item1: Item = catalog.get("1").unwrap().unwrap();
        let item2: Item = catalog.get("2").unwrap().unwrap();
        let item3: Item = catalog.get("3").unwrap().unwrap();
        assert_eq!(item1.price, 150);
        assert_eq!(item2.price, 250);
        assert_eq!(item3.price, 350);
        let _ = catalog.with_items(|items| {
            let entity = items.get("1").unwrap();
            assert_eq!(entity.state, EntityState::Updated);
            Ok(())
        });
        let _ = catalog.with_items(|items| {
            let entity = items.get("2").unwrap();
            assert_eq!(entity.state, EntityState::Updated);
            Ok(())
        });
        let _ = catalog.with_items(|items| {
            let entity = items.get("3").unwrap();
            assert_eq!(entity.state, EntityState::Updated);
            Ok(())
        });
    }
}
