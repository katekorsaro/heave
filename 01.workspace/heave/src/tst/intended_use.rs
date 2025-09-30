#[cfg(test)]
mod tests {
    use crate::*;
    #[derive(Debug, Clone, PartialEq)]
    struct Product {
        pub id: String,
        pub name: String,
        pub price: u64,
    }
    impl EAV for Product {}
    impl From<Product> for Entity {
        fn from(value: Product) -> Entity {
            Entity::default()
                .with_class("product")
                .with_id(&value.id)
                .with_attribute("name", value.name)
                .with_attribute("price", value.price)
        }
    }
    impl From<Entity> for Product {
        fn from(entity: Entity) -> Self {
            Product {
                id: entity.id.clone(),
                name: entity.unwrap("name"),
                price: entity.unwrap("price"),
            }
        }
    }
    #[test]
    fn check_001() {
        // Demonstrates the costruction of a new entity instance
        let _product = Entity::new("product")
            .with_attribute("name", "laptop")
            .with_attribute("price", 200000u64)
            .with_attribute("discount", 2.5f64)
            .with_attribute("in_stock", true);
    }
    #[test]
    fn check_002() {
        // Demonstrate attribute value reading
        let product = Entity::new("product")
            .with_attribute("name", "laptop")
            .with_attribute("price", 200000u64);
        let name = product.value_of("name");
        let price = product.value_of("price");
        assert_eq!(name, Some(&Value::Text(String::from("laptop"))));
        assert_eq!(price, Some(&Value::UnsignedInt(200000)));
    }
    #[test]
    fn check_003() {
        // Demonstrate attribute value setting
        let mut product = Entity::new("product");
        // new product
        product.set("price", 200000u64);
        // set price
        let price = product.value_of("price");
        assert_eq!(price, Some(&Value::UnsignedInt(200000)));
        // set in stock
        product.set("in_stock", true);
        let in_stock = product.value_of("in_stock");
        assert_eq!(in_stock, Some(&Value::Bool(true)));
        // set in stock again
        let prev_in_stock = product.set("in_stock", false);
        let in_stock = product.value_of("in_stock");
        assert_eq!(prev_in_stock, Some(Value::Bool(true)));
        assert_eq!(in_stock, Some(&Value::Bool(false)));
    }
    #[test]
    fn check_004() {
        // new entities
        let product = Entity::new("product");
        let category = Entity::new("category");
        // id clones for testing purposes
        let product_id = product.id.clone();
        let category_id = category.id.clone();
        // new relation as entity
        let product_has_category = Entity::new("product_has_category")
            .with_attribute("product", product)
            .with_attribute("category", category);
        assert_eq!(
            product_has_category.value_of("product"),
            Some(&Value::Text(product_id))
        );
        assert_eq!(
            product_has_category.value_of("category"),
            Some(&Value::Text(category_id))
        );
    }
    #[test]
    fn check_005() {
        let tag = Entity::new("tag").with_attribute("label", "new");
        let tag_id = tag.id.clone();
        let entity = Entity::new("product")
            .with_attribute("name", "laptop")
            .with_attribute("price", 200000u64)
            .with_attribute("delta", -50i64)
            .with_attribute("in_stock", true)
            .with_attribute("discount", 5.2f64)
            .with_attribute("tag", tag);
        let name: String = entity.unwrap("name");
        let price: u64 = entity.unwrap("price");
        let delta: i64 = entity.unwrap("delta");
        let in_stock: bool = entity.unwrap("in_stock");
        let discount: f64 = entity.unwrap("discount");
        let tag: Entity = entity.unwrap("tag");
        assert_eq!(name, "laptop".to_string());
        assert_eq!(price, 200000u64);
        assert_eq!(delta, -50i64);
        assert!(in_stock);
        assert_eq!(discount, 5.2f64);
        assert_eq!(tag.id, tag_id);
    }
    #[test]
    fn check_006() {
        let product = Product {
            id: "abcdef".to_string(),
            name: "laptop".to_string(),
            price: 200000u64,
        };
        let expected_product = product.clone();
        let entity: Entity = product.into();
        let converted_product = Product::from(entity);
        assert_eq!(expected_product, converted_product);
    }
    #[test]
    fn check_007() {
        let mut catalog = Catalog::new("");
        let product = Product {
            id: short_uuid::short!().to_string(),
            name: "laptop".to_string(),
            price: 200000u64,
        };
        catalog.insert(product);
    }
    #[test]
    fn check_008() {
        let mut catalog = Catalog::new("");
        let product01 = Product {
            id: short_uuid::short!().to_string(),
            name: "laptop".to_string(),
            price: 200000u64,
        };
        let product02 = Product {
            id: short_uuid::short!().to_string(),
            name: "desktop".to_string(),
            price: 150000u64,
        };
        let products = vec![product01, product02];
        catalog.insert_many(products);
    }
    #[test]
    fn check_009() {
        let mut catalog = Catalog::new("");
        let product = Product {
            id: short_uuid::short!().to_string(),
            name: "laptop".to_string(),
            price: 200000u64,
        };
        let expected_product = product.clone();
        catalog.insert(product);
        let read_product: Product = catalog.get(&expected_product.id).unwrap();
        assert_eq!(read_product, expected_product);
    }
}
