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
}
