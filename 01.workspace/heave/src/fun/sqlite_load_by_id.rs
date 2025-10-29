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
