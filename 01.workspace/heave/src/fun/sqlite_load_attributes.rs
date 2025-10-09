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
    #[test]
    fn load_attributes_should_populate_entity_from_db() {
        // Verifies that attributes for a given entity are correctly loaded from the database and added to the entity's attributes map.
        todo!();
    }
    #[test]
    fn load_attributes_should_handle_entities_with_no_attributes() {
        // Ensures that the function completes without error and without adding attributes for an entity that has none in the database.
        todo!();
    }
    #[test]
    fn load_attributes_should_return_error_on_query_failure() {
        // Checks that an error is returned if the database query to select attributes fails.
        todo!();
    }
}
