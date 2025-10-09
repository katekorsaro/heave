use crate::*;
use rusqlite::*;

const SELECT_ENTITY_BY_CLASS: &str = r#"
    SELECT * FROM entity
    WHERE class = ?1;
"#;

pub fn run(path: &path::Path, entity_class: &str) -> Result<Vec<Entity>, FailedTo> {
    let mut entities = Vec::<Entity>::new();
    let mut connection = Connection::open(path).map_err(|_| sqlite::FailedTo::OpenConnection)?;
    let mut transaction = connection
        .transaction()
        .map_err(|_| sqlite::FailedTo::BeginTransaction)?;
    transaction.set_drop_behavior(DropBehavior::Commit);
    let mut statement = transaction
        .prepare(SELECT_ENTITY_BY_CLASS)
        .map_err(|_| sqlite::FailedTo::PrepareStatement)?;
    let result = statement
        .query_map([entity_class], sqlite::map::row_to_entity)
        .map_err(|_| sqlite::FailedTo::ExecuteQuery)?;
    for entity in result {
        let mut entity = entity.map_err(|_| FailedTo::MapEntity)?;
        sqlite::load::attributes(&transaction, &mut entity)?;
        entity.state = EntityState::Loaded;
        entities.push(entity);
    }
    Ok(entities)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn load_by_class_should_fetch_all_entities_for_a_given_class() {
        // Verifies that all entities belonging to a specific class are retrieved from the database.
        todo!();
    }
    #[test]
    fn load_by_class_should_return_empty_vec_for_non_existent_class() {
        // Ensures that an empty vector is returned when querying for a class that has no entities in the database.
        todo!();
    }
    #[test]
    fn load_by_class_should_fully_load_entities_with_attributes() {
        // Checks that the retrieved entities are complete, including all their associated attributes.
        todo!();
    }
    #[test]
    fn load_by_class_should_fail_gracefully_on_db_connection_error() {
        // Tests that the function returns an appropriate error if the database connection cannot be established.
        todo!();
    }
}
