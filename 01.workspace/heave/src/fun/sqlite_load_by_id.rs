use crate::*;
use rusqlite::*;

const SELECT_ENTITY_BY_ID: &str = r#"
    SELECT * FROM entity
    WHERE id = ?1;
"#;

pub fn run(path: &path::Path, entity_id: &str) -> Result<Option<Entity>, FailedTo> {
    let mut connection = Connection::open(path).map_err(|_| sqlite::FailedTo::OpenConnection)?;
    let mut transaction = connection
        .transaction()
        .map_err(|_| sqlite::FailedTo::BeginTransaction)?;
    transaction.set_drop_behavior(DropBehavior::Commit);
    let mut entity = transaction
        .query_one(SELECT_ENTITY_BY_ID, [entity_id], sqlite::map::row_to_entity)
        .optional()
        .map_err(|_| sqlite::FailedTo::ExecuteQuery)?;
    if let Some(ref mut entity) = entity {
        sqlite::load::attributes(&transaction, entity)?;
        entity.state = EntityState::Loaded;
    }
    Ok(entity)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{fun, str::value::Value};
    use std::{fs, path::Path};
    fn setup_db(db_path: &Path) {
        let _ = fs::remove_file(db_path);
        fun::sqlite_init_db::run(db_path).unwrap();
        let conn = Connection::open(db_path).unwrap();
        conn.execute("INSERT INTO entity (id, class) VALUES ('e1', 'c1')", [])
            .unwrap();
        conn.execute(
            "INSERT INTO attribute (id, entity_id, value_text) VALUES ('a1', 'e1', 'v1')",
            [],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO attribute (id, entity_id, value_int) VALUES ('a2', 'e1', 100)",
            [],
        )
        .unwrap();
        conn.execute("INSERT INTO entity (id, class) VALUES ('e2', 'c2')", [])
            .unwrap();
    }
    #[test]
    fn load_by_id_should_fetch_correct_entity() {
        // Verifies that the correct entity is retrieved from the database when a valid ID is provided.
        let db_path = Path::new("test_fetch_by_id.db");
        setup_db(db_path);
        let result = run(db_path, "e1");
        assert!(result.is_ok());
        let entity_opt = result.unwrap();
        assert!(entity_opt.is_some());
        let entity = entity_opt.unwrap();
        assert_eq!(entity.id, "e1");
        assert_eq!(entity.class, "c1");
        fs::remove_file(db_path).unwrap();
    }
    #[test]
    fn load_by_id_should_return_none_for_non_existent_id() {
        // Ensures that `Ok(None)` is returned when querying for an ID that does not exist in the database.
        let db_path = Path::new("test_non_existent_id.db");
        setup_db(db_path);
        let result = run(db_path, "non_existent");
        assert!(result.is_ok());
        let entity_opt = result.unwrap();
        assert!(entity_opt.is_none());
        fs::remove_file(db_path).unwrap();
    }
    #[test]
    fn load_by_id_should_load_entity_with_all_attributes() {
        // Checks that the retrieved entity includes all of its associated attributes.
        let db_path = Path::new("test_load_with_attributes.db");
        setup_db(db_path);
        let entity = run(db_path, "e1").unwrap().unwrap();
        assert_eq!(entity.attributes.len(), 2);
        let attr1 = entity.attributes.get("a1").unwrap();
        assert_eq!(attr1.value, Value::Text("v1".to_string()));
        let attr2 = entity.attributes.get("a2").unwrap();
        assert_eq!(attr2.value, Value::SignedInt(100));
        fs::remove_file(db_path).unwrap();
    }
    #[test]
    fn load_by_id_should_fail_gracefully_on_db_error() {
        // Tests that an error is returned if the database query fails for any reason.
        // Test case 1: Invalid path
        let invalid_path = Path::new("/");
        let result = run(invalid_path, "any_id");
        assert!(result.is_err());
        // Test case 2: Query error
        let db_path = Path::new("test_db_error.db");
        let _ = fs::remove_file(db_path);
        fun::sqlite_init_db::run(db_path).unwrap();
        let conn = Connection::open(db_path).unwrap();
        conn.execute("DROP TABLE entity", []).unwrap();
        drop(conn); // Close connection before `run` tries to open it.
        let result = run(db_path, "any_id");
        assert!(result.is_err());
        fs::remove_file(db_path).unwrap();
    }
}
