#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    pub fn check_001() {
        let entity = Entity::new("class");
        assert_eq!(entity.class, String::from("class"));
        assert_eq!(entity.attributes.len(), 0);
    }
}
