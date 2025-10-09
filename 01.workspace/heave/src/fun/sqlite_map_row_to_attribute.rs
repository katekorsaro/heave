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
        _ => panic!(),
    };
    Ok(Attribute { id, value })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn map_row_to_attribute_with_signed_int_value() {
        // Verifies that a database row with a signed integer value is correctly mapped to an Attribute with a Value::SignedInt.
        todo!();
    }
    #[test]
    fn map_row_to_attribute_with_unsigned_int_value() {
        // Verifies that a database row with an unsigned integer value is correctly mapped to an Attribute with a Value::UnsignedInt.
        todo!();
    }
    #[test]
    fn map_row_to_attribute_with_real_value() {
        // Verifies that a database row with a real (float) value is correctly mapped to an Attribute with a Value::Real.
        todo!();
    }
    #[test]
    fn map_row_to_attribute_with_text_value() {
        // Verifies that a database row with a text value is correctly mapped to an Attribute with a Value::Text.
        todo!();
    }
    #[test]
    fn map_row_to_attribute_with_bool_value() {
        // Verifies that a database row with a boolean value is correctly mapped to an Attribute with a Value::Bool.
        todo!();
    }
    #[test]
    #[should_panic]
    fn map_row_to_attribute_should_panic_on_multiple_values() {
        // Ensures that the function panics if a row contains data in more than one 'value' column, which indicates data corruption.
        todo!();
    }
    #[test]
    fn map_row_to_attribute_should_return_error_on_type_mismatch() {
        // Checks that a rusqlite::Error is returned if a column's type does not match the expected schema (e.g., text in an integer column).
        todo!();
    }
}
