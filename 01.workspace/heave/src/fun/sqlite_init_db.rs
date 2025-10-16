use crate::*;
use rusqlite::*;

pub fn run(path: &path::Path) -> result::Result<(), FailedTo> {
    let init_statement = r#"
        CREATE TABLE IF NOT EXISTS entity (
            id TEXT PRIMARY KEY,
            class TEXT NOT NULL,
            ref_date INTEGER
        );
        CREATE TABLE IF NOT EXISTS attribute (
            id TEXT,
            entity_id TEXT,
            value_int INTEGER,
            value_uint INTEGER,
            value_real REAL,
            value_text TEXT,
            value_bool BOOL,
            CONSTRAINT pk_id PRIMARY KEY (id, entity_id),
            CONSTRAINT fk_entity_id FOREIGN KEY (entity_id) REFERENCES entity (id) ON DELETE CASCADE ON UPDATE CASCADE
        );
        CREATE INDEX IF NOT EXISTS entity_class ON entity (class);
        CREATE INDEX IF NOT EXISTS attribute_id ON attribute (id);
        "#;
    let connection = Connection::open(path).map_err(|_| sqlite::FailedTo::OpenConnection)?;
    connection
        .execute_batch(init_statement)
        .map_err(|_| sqlite::FailedTo::ExecuteBatch)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;
    #[test]
    fn init_db_should_create_tables_and_indexes_on_new_db() {
        // This test verifies that a new database is correctly initialized with the necessary tables ('entity', 'attribute') and indexes.
        let db_path = Path::new("test_new.db");
        let _ = fs::remove_file(db_path); // Ensure the file doesn't exist
        assert!(run(db_path).is_ok());
        let conn = Connection::open(db_path).unwrap();
        let mut stmt = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name IN ('entity', 'attribute')")
            .unwrap();
        let tables: Vec<String> = stmt
            .query_map([], |row| row.get(0))
            .unwrap()
            .map(|r| r.unwrap())
            .collect();
        assert_eq!(tables.len(), 2);
        assert!(tables.contains(&"entity".to_string()));
        assert!(tables.contains(&"attribute".to_string()));
        let mut stmt = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='index' AND name IN ('entity_class', 'attribute_id')")
            .unwrap();
        let indexes: Vec<String> = stmt
            .query_map([], |row| row.get(0))
            .unwrap()
            .map(|r| r.unwrap())
            .collect();
        assert_eq!(indexes.len(), 2);
        assert!(indexes.contains(&"entity_class".to_string()));
        assert!(indexes.contains(&"attribute_id".to_string()));
        let _ = fs::remove_file(db_path);
    }
    #[test]
    fn init_db_should_be_idempotent_and_not_fail_on_existing_db() {
        // This test ensures that initializing an already existing and initialized database does not cause errors.
        let db_path = Path::new("test_idempotent.db");
        let _ = fs::remove_file(db_path);
        assert!(run(db_path).is_ok());
        assert!(run(db_path).is_ok());
        let _ = fs::remove_file(db_path);
    }
    #[test]
    fn init_db_should_fail_gracefully_on_invalid_path() {
        // This test checks that the function returns an error when provided with an invalid or inaccessible file path.
        // An invalid path, like a directory, should cause an error
        let invalid_path = Path::new("/");
        assert!(run(invalid_path).is_err());
    }
}
