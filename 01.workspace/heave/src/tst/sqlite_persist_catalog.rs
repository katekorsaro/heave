#[cfg(test)]
mod tests {
    use crate::*;
    use rusqlite::*;
    use std::{collections::HashMap, fs, path::Path};
    fn setup_db(db_path: &Path) -> Connection {
        let _ = fs::remove_file(db_path);
        fun::sqlite_init_db::run(db_path).unwrap();
        Connection::open(db_path).unwrap()
    }
    fn count_rows(conn: &Connection, table: &str, where_clause: &str) -> i64 {
        let mut stmt = conn
            .prepare(&format!(
                "SELECT COUNT(*) FROM {} WHERE {}",
                table, where_clause
            ))
            .unwrap();
        stmt.query_row([], |row| row.get(0)).unwrap()
    }
    #[test]
    fn persist_should_insert_new_entities_and_attributes() {
        // Verifies that entities marked as 'New' in the catalog are inserted into the database, along with all their attributes.
        let db_path = Path::new("test_insert.db");
        let conn = setup_db(db_path);
        let catalog = Catalog::new(db_path.to_str().unwrap());
        let mut entity = Entity {
            id: "e1".to_string(),
            class: "c1".to_string(),
            subclass: None,
            attributes: HashMap::new(),
            state: EntityState::New,
            ref_date: None,
        };
        entity.attributes.insert(
            "a1".to_string(),
            Attribute {
                id: "a1".to_string(),
                value: Value::Text("v1".to_string()),
            },
        );
        let _ = catalog.on_items(|items| {
            items.insert("e1".to_string(), entity);
            Ok(())
        });
        let is_ok = catalog
            .with_items(|items| Ok(sqlite::persist::catalog(db_path, items).is_ok()))
            .unwrap();
        assert!(is_ok);
        assert_eq!(count_rows(&conn, "entity", "id = 'e1'"), 1);
        assert_eq!(count_rows(&conn, "attribute", "entity_id = 'e1'"), 1);
        fs::remove_file(db_path).unwrap();
    }
    #[test]
    fn persist_should_delete_entities_marked_for_deletion() {
        // Ensures that entities marked as 'ToDelete' are correctly removed from the database.
        let db_path = Path::new("test_delete.db");
        let conn = setup_db(db_path);
        conn.execute("INSERT INTO entity (id, class) VALUES ('e1', 'c1')", [])
            .unwrap();
        let catalog = Catalog::new(db_path.to_str().unwrap());
        let entity = Entity {
            id: "e1".to_string(),
            class: "c1".to_string(),
            subclass: None,
            attributes: HashMap::new(),
            state: EntityState::ToDelete,
            ref_date: None,
        };
        let _ = catalog.on_items(|items| {
            items.insert("e1".to_string(), entity);
            Ok(())
        });
        let is_ok = catalog
            .with_items(|items| Ok(sqlite::persist::catalog(db_path, items).is_ok()))
            .unwrap();
        assert!(is_ok);
        assert_eq!(count_rows(&conn, "entity", "id = 'e1'"), 0);
        fs::remove_file(db_path).unwrap();
    }
    #[test]
    fn persist_should_handle_a_mix_of_new_and_deleted_entities() {
        // Tests the function's ability to handle a batch operation involving both new entities to be inserted and existing ones to be deleted.
        let db_path = Path::new("test_mix.db");
        let conn = setup_db(db_path);
        conn.execute("INSERT INTO entity (id, class) VALUES ('e1', 'c1')", [])
            .unwrap();
        let catalog = Catalog::new(db_path.to_str().unwrap());
        let to_delete = Entity {
            id: "e1".to_string(),
            class: "c1".to_string(),
            subclass: None,
            attributes: HashMap::new(),
            state: EntityState::ToDelete,
            ref_date: None,
        };
        let to_add = Entity {
            id: "e2".to_string(),
            class: "c2".to_string(),
            subclass: None,
            attributes: HashMap::new(),
            state: EntityState::New,
            ref_date: None,
        };
        let _ = catalog.on_items(|items| {
            items.insert("e1".to_string(), to_delete);
            Ok(())
        });
        let _ = catalog.on_items(|items| {
            items.insert("e2".to_string(), to_add);
            Ok(())
        });
        let is_ok = catalog
            .with_items(|items| Ok(sqlite::persist::catalog(db_path, items).is_ok()))
            .unwrap();
        assert!(is_ok);
        assert_eq!(count_rows(&conn, "entity", "id = 'e1'"), 0);
        assert_eq!(count_rows(&conn, "entity", "id = 'e2'"), 1);
        fs::remove_file(db_path).unwrap();
    }
    #[test]
    fn persist_should_not_affect_unmodified_entities() {
        // Verifies that entities in the catalog that are not marked as 'New' or 'ToDelete' remain untouched in the database.
        let db_path = Path::new("test_unmodified.db");
        let conn = setup_db(db_path);
        conn.execute("INSERT INTO entity (id, class) VALUES ('e1', 'c1')", [])
            .unwrap();
        let catalog = Catalog::new(db_path.to_str().unwrap());
        let unmodified = Entity {
            id: "e1".to_string(),
            class: "c1".to_string(),
            subclass: None,
            attributes: HashMap::new(),
            state: EntityState::Loaded,
            ref_date: None,
        };
        let _ = catalog.on_items(|items| {
            items.insert("e1".to_string(), unmodified);
            Ok(())
        });
        let is_ok = catalog
            .with_items(|items| Ok(sqlite::persist::catalog(db_path, items).is_ok()))
            .unwrap();
        assert!(is_ok);
        assert_eq!(count_rows(&conn, "entity", "id = 'e1'"), 1);
        fs::remove_file(db_path).unwrap();
    }
    #[test]
    fn persist_should_rollback_transaction_on_failure() {
        // Ensures that if any part of the persistence process fails, the entire transaction is rolled back, leaving the database state unchanged.
        let db_path = Path::new("test_rollback.db");
        let conn = setup_db(db_path);
        conn.execute(
            "INSERT INTO entity (id, class) VALUES ('e_existing', 'c1')",
            [],
        )
        .unwrap();
        let catalog = Catalog::new(db_path.to_str().unwrap());
        let mut new_entity = Entity {
            id: "e_new".to_string(),
            class: "c1".to_string(),
            subclass: None,
            attributes: HashMap::new(),
            state: EntityState::New,
            ref_date: None,
        };
        new_entity.attributes.insert(
            "a1".to_string(),
            Attribute {
                id: "a1".to_string(),
                value: Value::Text("v1".to_string()),
            },
        );
        let _ = catalog.on_items(|items| {
            items.insert("e_new".to_string(), new_entity);
            Ok(())
        });
        // Corrupt the DB to cause a failure during the transaction
        conn.execute("DROP TABLE attribute", []).unwrap();
        drop(conn);
        let is_err = catalog
            .with_items(|items| Ok(sqlite::persist::catalog(db_path, items).is_err()))
            .unwrap();
        assert!(is_err);
        // Re-open connection to check state
        let conn = Connection::open(db_path).unwrap();
        // The new entity should not have been inserted due to rollback
        assert_eq!(count_rows(&conn, "entity", "id = 'e_new'"), 0);
        // The existing entity should still be there
        assert_eq!(count_rows(&conn, "entity", "id = 'e_existing'"), 1);
        fs::remove_file(db_path).unwrap();
    }
}
