use crate::*;
use rusqlite::*;

fn column(value: &Value) -> &'static str {
    match value {
        Value::SignedInt(_) => "value_int",
        Value::UnsignedInt(_) => "value_uint",
        Value::Real(_) => "value_real",
        Value::Text(_) => "value_text",
        Value::Bool(_) => "value_bool",
    }
}

const DELETE_ENTITY_STATEMENT: &str = r#"
    DELETE FROM entity
    WHERE entity.id = ?1;
"#;

const INSERT_ENTITY_STATEMENT: &str = r#"
    INSERT INTO entity (id, class, ref_date)
    VALUES (?1, ?2, ?3);
"#;

const INSERT_ATTRIBUTE_STATEMENT_TEMPLATE: &str = r#"
    INSERT INTO attribute (id, entity_id, {column})
    VALUES (?1, ?2, ?3);
"#;

fn write_attribute(
    attribute: &Attribute,
    entity: &Entity,
    transaction: &rusqlite::Transaction,
) -> result::Result<(), FailedTo> {
    let column = column(&attribute.value);
    let attribute_values = (&attribute.id, &entity.id, &attribute.value.to_string());
    let insert_attribute_statement =
        INSERT_ATTRIBUTE_STATEMENT_TEMPLATE.replace("{column}", column);
    transaction
        .execute(&insert_attribute_statement, attribute_values)
        .map_err(|_| sqlite::FailedTo::ExecuteStatement)?;
    Ok(())
}

fn delete_entity(entity: &Entity, transaction: &rusqlite::Transaction) -> Result<(), FailedTo> {
    let entity_id = [&entity.id];
    transaction
        .execute(DELETE_ENTITY_STATEMENT, entity_id)
        .map_err(|_| sqlite::FailedTo::ExecuteStatement)?;
    Ok(())
}

fn write_entity(entity: &Entity, transaction: &rusqlite::Transaction) -> Result<(), FailedTo> {
    let entity_id = [&entity.id];
    let entity_values = (&entity.id, &entity.class, entity.ref_date);
    transaction
        .execute(DELETE_ENTITY_STATEMENT, entity_id)
        .map_err(|_| sqlite::FailedTo::ExecuteStatement)?;
    transaction
        .execute(INSERT_ENTITY_STATEMENT, entity_values)
        .map_err(|_| sqlite::FailedTo::ExecuteStatement)?;
    for attribute in entity.attributes.values() {
        write_attribute(attribute, entity, transaction)?;
    }
    Ok(())
}

pub fn run(path: &path::Path, catalog: &Catalog) -> result::Result<(), FailedTo> {
    let mut connection = Connection::open(path).map_err(|_| sqlite::FailedTo::OpenConnection)?;
    let transaction = connection
        .transaction()
        .map_err(|_| sqlite::FailedTo::BeginTransaction)?;
    for entity in catalog.items.values().filter(|item| {
        item.state == EntityState::New
            || item.state == EntityState::Updated
            || item.state == EntityState::ToDelete
    }) {
        match entity.state {
            EntityState::New | EntityState::Updated => write_entity(entity, &transaction)?,
            EntityState::ToDelete => delete_entity(entity, &transaction)?,
            _ => unreachable!(),
        }
    }
    transaction
        .commit()
        .map_err(|_| sqlite::FailedTo::CommitTransaction)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
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
        let mut catalog = Catalog::new(db_path.to_str().unwrap());
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
        catalog.items.insert("e1".to_string(), entity);
        assert!(run(db_path, &catalog).is_ok());
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
        let mut catalog = Catalog::new(db_path.to_str().unwrap());
        let entity = Entity {
            id: "e1".to_string(),
            class: "c1".to_string(),
            subclass: None,
            attributes: HashMap::new(),
            state: EntityState::ToDelete,
            ref_date: None,
        };
        catalog.items.insert("e1".to_string(), entity);
        assert!(run(db_path, &catalog).is_ok());
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
        let mut catalog = Catalog::new(db_path.to_str().unwrap());
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
        catalog.items.insert("e1".to_string(), to_delete);
        catalog.items.insert("e2".to_string(), to_add);
        assert!(run(db_path, &catalog).is_ok());
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
        let mut catalog = Catalog::new(db_path.to_str().unwrap());
        let unmodified = Entity {
            id: "e1".to_string(),
            class: "c1".to_string(),
            subclass: None,
            attributes: HashMap::new(),
            state: EntityState::Loaded,
            ref_date: None,
        };
        catalog.items.insert("e1".to_string(), unmodified);
        assert!(run(db_path, &catalog).is_ok());
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
        let mut catalog = Catalog::new(db_path.to_str().unwrap());
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
        catalog.items.insert("e_new".to_string(), new_entity);
        // Corrupt the DB to cause a failure during the transaction
        conn.execute("DROP TABLE attribute", []).unwrap();
        drop(conn);
        let result = run(db_path, &catalog);
        assert!(result.is_err());
        // Re-open connection to check state
        let conn = Connection::open(db_path).unwrap();
        // The new entity should not have been inserted due to rollback
        assert_eq!(count_rows(&conn, "entity", "id = 'e_new'"), 0);
        // The existing entity should still be there
        assert_eq!(count_rows(&conn, "entity", "id = 'e_existing'"), 1);
        fs::remove_file(db_path).unwrap();
    }
}
