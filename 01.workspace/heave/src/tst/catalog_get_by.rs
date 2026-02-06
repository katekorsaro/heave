#[cfg(test)]
mod tests {
    use crate::*;
    fn catalog(name: &str) -> Catalog {
        let path = format!("{}.db", name);
        let fs_path = path::Path::new(&path);
        if fs_path.exists() {
            std::fs::remove_file(fs_path).unwrap();
        }
        Catalog::new(&path)
    }
    #[test]
    fn get_by_finds_item() {
        let catalog = catalog("get_by_finds_item");
        let item1 = Item {
            id: "1".to_string(),
            name: "one".to_string(),
            ..Default::default()
        };
        let item2 = Item {
            id: "2".to_string(),
            name: "two".to_string(),
            ..Default::default()
        };
        catalog
            .insert_many(vec![item1.clone(), item2.clone()])
            .unwrap();
        let found = catalog
            .get_by(|item: &Item| item.name == "two")
            .unwrap()
            .unwrap();
        assert_eq!(found, item2);
    }
    #[test]
    fn get_by_returns_none_when_no_match() {
        let catalog = catalog("get_by_returns_none_when_no_match");
        let item1 = Item {
            id: "1".to_string(),
            name: "one".to_string(),
            ..Default::default()
        };
        catalog.insert_many(vec![item1.clone()]).unwrap();
        let found = catalog.get_by(|item: &Item| item.name == "two").unwrap();
        assert!(found.is_none());
    }
    #[test]
    fn get_by_on_empty_catalog_returns_none() {
        let catalog = catalog("get_by_on_empty_catalog_returns_none");
        let found = catalog.get_by(|item: &Item| item.name == "any").unwrap();
        assert!(found.is_none());
    }
    #[test]
    fn get_by_multiple_matches() {
        let catalog = catalog("get_by_multiple_matches");
        let item1 = Item {
            id: "1".to_string(),
            name: "match".to_string(),
            price: 10,
            ..Default::default()
        };
        let item2 = Item {
            id: "2".to_string(),
            name: "match".to_string(),
            price: 20,
            ..Default::default()
        };
        catalog
            .insert_many(vec![item1.clone(), item2.clone()])
            .unwrap();
        // `get_by` should return the first match it finds. The order is not guaranteed
        // by the underlying HashMap, so we just check that it returns one of them.
        let found = catalog
            .get_by(|item: &Item| item.name == "match")
            .unwrap()
            .unwrap();
        assert!(found == item1 || found == item2);
    }
}
