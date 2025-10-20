use crate::*;
use rusqlite::*;

const SELECT_ATTRIBUTE_BY_FK: &str = r#"
    SELECT * FROM attribute
    WHERE entity_id = ?1;
"#;

pub fn run(transaction: &Transaction, entity: &mut Entity) -> Result<(), FailedTo> {
    let mut select_attributes_statement = transaction
        .prepare(SELECT_ATTRIBUTE_BY_FK)
        .map_err(|_| sqlite::FailedTo::PrepareStatement)?;
    let attributes = select_attributes_statement
        .query_map([&entity.id], sqlite::map::row_to_attribute)
        .map_err(|_| sqlite::FailedTo::ExecuteQuery)?;
    for attribute in attributes {
        let attribute = attribute.map_err(|_| FailedTo::MapAttribute)?;
        entity.attributes.insert(attribute.id.clone(), attribute);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Entity, Value, fun};
    use std::{collections::HashMap, fs, path::Path};
    fn setup_db(db_path: &Path) -> Connection {
        let _ = fs::remove_file(db_path);
        fun::sqlite_init_db::run(db_path).unwrap();
        Connection::open(db_path).unwrap()
    }
    #[test]
    fn load_attributes_should_populate_entity_from_db() {
        // Verifies that attributes for a given entity are correctly loaded from the database and added to the entity's attributes map.
        let db_path = Path::new("test_load_attributes.db");
        let mut conn = setup_db(db_path);
        let mut entity = Entity {
            id: "entity1".to_string(),
            class: "class1".to_string(),
            subclass: None,
            attributes: HashMap::new(),
            state: Default::default(),
            ref_date: None,
        };
        conn.execute(
            "INSERT INTO entity (id, class) VALUES (?1, ?2)",
            params![&entity.id, &entity.class],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO attribute (id, entity_id, value_text) VALUES ('attr1', 'entity1', 'hello')",
            [],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO attribute (id, entity_id, value_int) VALUES ('attr2', 'entity1', 42)",
            [],
        )
        .unwrap();
        let tx = conn.transaction().unwrap();
        assert!(run(&tx, &mut entity).is_ok());
        tx.commit().unwrap();
        assert_eq!(entity.attributes.len(), 2);
        let attr1 = entity.attributes.get("attr1").unwrap();
        assert_eq!(attr1.id, "attr1");
        assert_eq!(attr1.value, Value::Text("hello".to_string()));
        let attr2 = entity.attributes.get("attr2").unwrap();
        assert_eq!(attr2.id, "attr2");
        assert_eq!(attr2.value, Value::SignedInt(42));
        fs::remove_file(db_path).unwrap();
    }
    #[test]
    fn load_attributes_should_handle_entities_with_no_attributes() {
        // Ensures that the function completes without error and without adding attributes for an entity that has none in the database.
        let db_path = Path::new("test_no_attributes.db");
        let mut conn = setup_db(db_path);
        let mut entity = Entity {
            id: "entity2".to_string(),
            class: "class1".to_string(),
            subclass: None,
            attributes: HashMap::new(),
            state: Default::default(),
            ref_date: None,
        };
        conn.execute(
            "INSERT INTO entity (id, class) VALUES (?1, ?2)",
            params![&entity.id, &entity.class],
        )
        .unwrap();
        let tx = conn.transaction().unwrap();
        assert!(run(&tx, &mut entity).is_ok());
        tx.commit().unwrap();
        assert!(entity.attributes.is_empty());
        fs::remove_file(db_path).unwrap();
    }
    #[test]
    fn load_attributes_should_return_error_on_query_failure() {
        // Checks that an error is returned if the database query to select attributes fails.
        let db_path = Path::new("test_query_failure.db");
        let mut conn = setup_db(db_path);
        conn.execute("DROP TABLE attribute", []).unwrap();
        let mut entity = Entity {
            id: "entity3".to_string(),
            class: "class1".to_string(),
            subclass: None,
            attributes: HashMap::new(),
            state: Default::default(),
            ref_date: None,
        };
        let tx = conn.transaction().unwrap();
        let result = run(&tx, &mut entity);
        assert!(result.is_err());
        fs::remove_file(db_path).unwrap();
    }
}
