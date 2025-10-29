#[cfg(test)]
mod tests {
    use crate::*;
    use rusqlite::*;
    use std::{fs, path::Path};
    fn setup_db(db_path: &Path) {
        let _ = fs::remove_file(db_path);
        sqlite::init::db(db_path).unwrap();
        let conn = Connection::open(db_path).unwrap();
        // Class 'c1'
        conn.execute("INSERT INTO entity (id, class) VALUES ('e1_c1', 'c1')", [])
            .unwrap();
        conn.execute(
            "INSERT INTO attribute (id, entity_id, value_text) VALUES ('a1', 'e1_c1', 'v1')",
            [],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO attribute (id, entity_id, value_int) VALUES ('a2', 'e1_c1', 1)",
            [],
        )
        .unwrap();
        conn.execute("INSERT INTO entity (id, class) VALUES ('e2_c1', 'c1')", [])
            .unwrap();
        // Class 'c2'
        conn.execute("INSERT INTO entity (id, class) VALUES ('e1_c2', 'c2')", [])
            .unwrap();
    }
    #[test]
    fn load_by_class_should_fetch_all_entities_for_a_given_class() {
        // Verifies that all entities belonging to a specific class are retrieved from the database.
        let db_path = Path::new("test_fetch_by_class.db");
        setup_db(db_path);
        let result = sqlite::load::by_class(db_path, "c1");
        assert!(result.is_ok());
        let entities = result.unwrap();
        assert_eq!(entities.len(), 2);
        let ids: Vec<_> = entities.iter().map(|e| e.id.clone()).collect();
        assert!(ids.contains(&"e1_c1".to_string()));
        assert!(ids.contains(&"e2_c1".to_string()));
        fs::remove_file(db_path).unwrap();
    }
    #[test]
    fn load_by_class_should_return_empty_vec_for_non_existent_class() {
        // Ensures that an empty vector is returned when querying for a class that has no entities in the database.
        let db_path = Path::new("test_non_existent_class.db");
        setup_db(db_path);
        let result = sqlite::load::by_class(db_path, "non_existent");
        assert!(result.is_ok());
        let entities = result.unwrap();
        assert!(entities.is_empty());
        fs::remove_file(db_path).unwrap();
    }
    #[test]
    fn load_by_class_should_fully_load_entities_with_attributes() {
        // Checks that the retrieved entities are complete, including all their associated attributes.
        let db_path = Path::new("test_fully_load.db");
        setup_db(db_path);
        let entities = sqlite::load::by_class(db_path, "c1").unwrap();
        let entity1 = entities.iter().find(|e| e.id == "e1_c1").unwrap();
        assert_eq!(entity1.attributes.len(), 2);
        assert!(entity1.attributes.contains_key("a1"));
        assert!(entity1.attributes.contains_key("a2"));
        let attr1 = entity1.attributes.get("a1").unwrap();
        assert_eq!(attr1.value, Value::Text("v1".to_string()));
        let attr2 = entity1.attributes.get("a2").unwrap();
        assert_eq!(attr2.value, Value::SignedInt(1));
        fs::remove_file(db_path).unwrap();
    }
    #[test]
    fn load_by_class_should_fail_gracefully_on_db_connection_error() {
        // Tests that the function returns an appropriate error if the database connection cannot be established.
        let invalid_path = Path::new("/"); // A directory is not a valid database file
        let result = sqlite::load::by_class(invalid_path, "any_class");
        assert!(result.is_err());
    }
}
