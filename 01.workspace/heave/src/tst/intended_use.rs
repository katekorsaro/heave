#[cfg(test)]
mod tests {
    use crate::*;
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
}
