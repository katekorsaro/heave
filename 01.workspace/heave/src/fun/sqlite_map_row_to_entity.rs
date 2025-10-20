use crate::*;

pub fn run(row: &rusqlite::Row) -> rusqlite::Result<Entity> {
    let id: String = row.get(0)?;
    let class: String = row.get(1)?;
    let subclass: Option<String> = row.get(2)?;
    let ref_date: Option<u64> = row.get(3)?;
    let entity = Entity {
        id,
        state: EntityState::Loaded,
        class,
        subclass,
        ref_date,
        attributes: std::collections::HashMap::new(),
    };
    Ok(entity)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::{Connection, Result};
    fn get_entity_from_query(conn: &Connection, query: &str) -> Result<Entity, rusqlite::Error> {
        let mut stmt = conn.prepare(query).unwrap();
        stmt.query_row([], run)
    }
    #[test]
    fn map_row_to_entity_should_correctly_map_valid_row() {
        // Verifies that a valid database row is correctly mapped to an Entity struct with all fields populated.
        let conn = Connection::open_in_memory().unwrap();
        let entity = get_entity_from_query(&conn, "SELECT 'e1', 'c1', 's1', 12345").unwrap();
        assert_eq!(entity.id, "e1");
        assert_eq!(entity.class, "c1");
        assert_eq!(entity.subclass, Some("s1".to_string()));
        assert_eq!(entity.ref_date, Some(12345));
        assert_eq!(entity.state, EntityState::Loaded);
        assert!(entity.attributes.is_empty());
    }
    #[test]
    fn map_row_to_entity_should_handle_null_ref_date() {
        // Ensures that a row with a NULL 'ref_date' is successfully mapped to an Entity with 'ref_date' as None.
        let conn = Connection::open_in_memory().unwrap();
        let entity = get_entity_from_query(&conn, "SELECT 'e2', 'c2', 's2', NULL").unwrap();
        assert_eq!(entity.id, "e2");
        assert_eq!(entity.class, "c2");
        assert_eq!(entity.subclass, Some("s2".to_string()));
        assert_eq!(entity.ref_date, None);
    }
    #[test]
    fn map_row_to_entity_should_handle_null_subclass() {
        // Ensures that a row with a NULL 'subclass' is successfully mapped to an Entity with 'subclass' as None.
        let conn = Connection::open_in_memory().unwrap();
        let entity = get_entity_from_query(&conn, "SELECT 'e2', 'c2', NULL, NULL").unwrap();
        assert_eq!(entity.id, "e2");
        assert_eq!(entity.class, "c2");
        assert_eq!(entity.subclass, None);
        assert_eq!(entity.ref_date, None);
    }
    #[test]
    fn map_row_to_entity_should_return_error_on_type_mismatch() {
        // Checks that a rusqlite::Error is returned if a column's type does not match the expected schema.
        let conn = Connection::open_in_memory().unwrap();
        // Trying to get an integer as a string for the class should fail.
        let result = get_entity_from_query(&conn, "SELECT 'e3', 123, NULL");
        assert!(result.is_err());
    }
}
