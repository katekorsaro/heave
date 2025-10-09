use crate::*;

pub fn run(row: &rusqlite::Row) -> rusqlite::Result<Entity> {
    let id: String = row.get(0)?;
    let class: String = row.get(1)?;
    let ref_date: Option<u64> = row.get(2)?;
    let entity = Entity {
        id,
        state: EntityState::Loaded,
        class,
        ref_date,
        attributes: std::collections::HashMap::new(),
    };
    Ok(entity)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn map_row_to_entity_should_correctly_map_valid_row() {
        // Verifies that a valid database row is correctly mapped to an Entity struct with all fields populated.
        todo!();
    }
    #[test]
    fn map_row_to_entity_should_handle_null_ref_date() {
        // Ensures that a row with a NULL 'ref_date' is successfully mapped to an Entity with 'ref_date' as None.
        todo!();
    }
    #[test]
    fn map_row_to_entity_should_return_error_on_type_mismatch() {
        // Checks that a rusqlite::Error is returned if a column's type does not match the expected schema.
        todo!();
    }
}
