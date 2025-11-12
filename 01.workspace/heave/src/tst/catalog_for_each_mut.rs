#[cfg(test)]
mod tests {
    use crate::*;
    #[derive(Debug, PartialEq)]
    enum Test {
        Error(String),
    }
    impl std::fmt::Display for Test {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
            write!(f, "error!")
        }
    }
    impl std::error::Error for Test {}
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
    #[test]
    fn for_each_mut_should_handle_empty_catalog() {
        let catalog = Catalog::new("test.db");
        let result = catalog.for_each_mut(|_item: &mut Item| Ok(()));
        assert!(result.is_ok());
        let count = catalog.with_items(|items| Ok(items.len())).unwrap();
        assert_eq!(count, 0);
    }
    #[test]
    fn for_each_mut_should_handle_closure_error() {
        let mut catalog = Catalog::new("test.db");
        prepare_catalog(&mut catalog);
        let result = catalog.for_each_mut(|item: &mut Item| {
            if item.id == "2" {
                Err(Box::new(Test::Error(item.id.clone())))
            } else {
                item.price += 50;
                Ok(())
            }
        });
        assert!(result.is_err());
        if let Err(FailedTo::ExecutePredicate(e)) = result {
            assert_eq!(
                e[0].downcast_ref::<Test>(),
                Some(&Test::Error("2".to_string()))
            );
        }
        let item1: Item = catalog.get("1").unwrap().unwrap();
        let item2: Item = catalog.get("2").unwrap().unwrap();
        let item3: Item = catalog.get("3").unwrap().unwrap();
        assert_eq!(item1.price, 150); // Modified
        assert_eq!(item2.price, 200); // Not modified due to error
        assert_eq!(item3.price, 350); // Modified
        let _ = catalog.with_items(|items| {
            let entity = items.get("1").unwrap();
            assert_eq!(entity.state, EntityState::Updated);
            Ok(())
        });
        let _ = catalog.with_items(|items| {
            let entity = items.get("2").unwrap();
            assert_eq!(entity.state, EntityState::New); // Should remain Unchanged
            Ok(())
        });
        let _ = catalog.with_items(|items| {
            let entity = items.get("3").unwrap();
            assert_eq!(entity.state, EntityState::Updated);
            Ok(())
        });
    }
}
