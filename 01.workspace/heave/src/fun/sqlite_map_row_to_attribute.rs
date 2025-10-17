use crate::*;

pub fn run(row: &rusqlite::Row) -> rusqlite::Result<Attribute> {
    let id: String = row.get(0)?;
    let _entity_id: String = row.get(1)?;
    let signed_int: Option<i64> = row.get(2)?;
    let unsigned_int: Option<u64> = row.get(3)?;
    let real: Option<f64> = row.get(4)?;
    let text: Option<String> = row.get(5)?;
    let bool: Option<bool> = row.get(6)?;
    let value: Value = match (signed_int, unsigned_int, real, text, bool) {
        (Some(value), None, None, None, None) => Value::SignedInt(value),
        (None, Some(value), None, None, None) => Value::UnsignedInt(value),
        (None, None, Some(value), None, None) => Value::Real(value),
        (None, None, None, Some(value), None) => Value::Text(value),
        (None, None, None, None, Some(value)) => Value::Bool(value),
        _ => {
            return Err(rusqlite::types::FromSqlError::InvalidType.into());
        }
    };
    Ok(Attribute { id, value })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::{Connection, Result};
    fn get_row_from_query(conn: &Connection, query: &str) -> Result<Attribute, rusqlite::Error> {
        let mut stmt = conn.prepare(query).unwrap();
        stmt.query_row([], run)
    }
    #[test]
    fn map_row_to_attribute_with_signed_int_value() {
        // Verifies that a database row with a signed integer value is correctly mapped to an Attribute with a Value::SignedInt.
        let conn = Connection::open_in_memory().unwrap();
        let attr =
            get_row_from_query(&conn, "SELECT 'a1', 'e1', -42, NULL, NULL, NULL, NULL").unwrap();
        assert_eq!(attr.id, "a1");
        assert_eq!(attr.value, Value::SignedInt(-42));
    }
    #[test]
    fn map_row_to_attribute_with_unsigned_int_value() {
        // Verifies that a database row with an unsigned integer value is correctly mapped to an Attribute with a Value::UnsignedInt.
        let conn = Connection::open_in_memory().unwrap();
        let attr =
            get_row_from_query(&conn, "SELECT 'a2', 'e1', NULL, 42, NULL, NULL, NULL").unwrap();
        assert_eq!(attr.id, "a2");
        assert_eq!(attr.value, Value::UnsignedInt(42));
    }
    #[test]
    fn map_row_to_attribute_with_real_value() {
        // Verifies that a database row with a real (float) value is correctly mapped to an Attribute with a Value::Real.
        let conn = Connection::open_in_memory().unwrap();
        let attr = get_row_from_query(&conn, "SELECT 'a3', 'e1', NULL, NULL, 12.2345, NULL, NULL")
            .unwrap();
        assert_eq!(attr.id, "a3");
        assert_eq!(attr.value, Value::Real(12.2345));
    }
    #[test]
    fn map_row_to_attribute_with_text_value() {
        // Verifies that a database row with a text value is correctly mapped to an Attribute with a Value::Text.
        let conn = Connection::open_in_memory().unwrap();
        let attr = get_row_from_query(&conn, "SELECT 'a4', 'e1', NULL, NULL, NULL, 'hello', NULL")
            .unwrap();
        assert_eq!(attr.id, "a4");
        assert_eq!(attr.value, Value::Text("hello".to_string()));
    }
    #[test]
    fn map_row_to_attribute_with_bool_value() {
        // Verifies that a database row with a boolean value is correctly mapped to an Attribute with a Value::Bool.
        let conn = Connection::open_in_memory().unwrap();
        let attr =
            get_row_from_query(&conn, "SELECT 'a5', 'e1', NULL, NULL, NULL, NULL, 1").unwrap();
        assert_eq!(attr.id, "a5");
        assert_eq!(attr.value, Value::Bool(true));
    }
    #[test]
    #[should_panic]
    fn map_row_to_attribute_should_panic_on_multiple_values() {
        // Ensures that the function panics if a row contains data in more than one 'value' column, which indicates data corruption.
        let conn = Connection::open_in_memory().unwrap();
        // This should panic because both signed_int and text have values.
        get_row_from_query(&conn, "SELECT 'a6', 'e1', -42, NULL, NULL, 'hello', NULL").unwrap();
    }
    #[test]
    fn map_row_to_attribute_should_return_error_on_type_mismatch() {
        // Checks that a rusqlite::Error is returned if a column's type does not match the expected schema (e.g., text in an integer column).
        let conn = Connection::open_in_memory().unwrap();
        // Trying to get a text value as an integer should fail.
        let result = get_row_from_query(
            &conn,
            // The value 'not a number' is in the signed_int column position.
            "SELECT 'a7', 'e1', 'not a number', NULL, NULL, NULL, NULL",
        );
        assert!(result.is_err());
    }
}
