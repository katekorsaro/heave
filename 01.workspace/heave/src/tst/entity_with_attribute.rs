#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    pub fn check_001() {
        let entity = Entity::new("class").with_attribute("a001", 0u64);
        assert_eq!(
            entity.attributes.get("a001"),
            Some(&Attribute {
                id: "a001".to_string(),
                value: Value::UnsignedInt(0)
            })
        );
    }
}
