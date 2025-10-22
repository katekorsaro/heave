#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn new_should_create_catalog_with_path_and_empty_items() {
        // Should create a new Catalog with the given path and an empty 'items' map.
        let path = "test.db";
        let catalog = Catalog::new(path);
        assert_eq!(catalog.path, path);
        assert!(catalog.items.is_empty());
    }
}
