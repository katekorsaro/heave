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
    #[test]
    fn load_by_id_should_fetch_correct_entity() {
        // Verifies that the correct entity is retrieved from the database when a valid ID is provided.
        todo!();
    }
    #[test]
    fn load_by_id_should_return_none_for_non_existent_id() {
        // Ensures that `Ok(None)` is returned when querying for an ID that does not exist in the database.
        todo!();
    }
    #[test]
    fn load_by_id_should_load_entity_with_all_attributes() {
        // Checks that the retrieved entity includes all of its associated attributes.
        todo!();
    }
    #[test]
    fn load_by_id_should_fail_gracefully_on_db_error() {
        // Tests that an error is returned if the database query fails for any reason.
        todo!();
    }
}
