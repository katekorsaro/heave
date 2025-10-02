#[cfg(test)]
mod tests {
    use crate::*;
    #[derive(Clone, Debug, PartialEq)]
    struct Product {
        id: String,
        name: String,
        model: Option<String>,
        price: u64,
        in_stock: bool,
    }
    impl EAV for Product {
        fn class() -> &'static str {
            "product"
        }
    }
    impl From<Entity> for Product {
        fn from(entity: Entity) -> Self {
            Product {
                id: entity.id.clone(),
                name: entity.unwrap("name"),
                model: entity.unwrap_opt("model"),
                price: entity.unwrap_or("price", 0),
                in_stock: entity.unwrap("in_stock"),
            }
        }
    }
    impl From<Product> for Entity {
        fn from(value: Product) -> Self {
            let mut entity = Entity::new::<Product>()
                .with_id(&value.id)
                .with_attribute("name", value.name)
                .with_attribute("price", value.price)
                .with_attribute("in_stock", value.in_stock);
            if let Some(model) = value.model {
                entity.set("model", model);
            }
            entity
        }
    }
    #[test]
    /// # Example 1
    ///
    /// - How to create a new empty catalog
    /// - How to insert a new entity
    /// - How to read the entity by id
    fn scenario_001() {
        // create a new named temp file
        let tempfile = tempfile::NamedTempFile::new().unwrap();
        let path = tempfile.path();
        let path = path.to_str().unwrap();
        // initialize a new empty catalog given an existing path
        let mut catalog = Catalog::new(path);
        // create a new object
        let product = Product {
            id: short_uuid::short!().to_string(),
            name: String::from("PenguinX Laptop"),
            model: None,
            price: 135000,
            in_stock: true,
        };
        // save some info for late comparison
        let original_product = product.clone();
        // insert the new object into catalog consuming it
        catalog.insert(product);
        // read value from catalog using the original key
        let read_product = catalog.get::<Product>(&original_product.id).unwrap();
        // assert equality between original and read
        assert_eq!(original_product, read_product);
    }
}
