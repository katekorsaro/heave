use crate::*;
use rusqlite::*;

const SELECT_ENTITY_BY_ID: &str = r#"
    SELECT id, class, subclass, ref_date FROM entity
    WHERE id = ?1;
"#;

pub fn run(path: &path::Path, entity_id: &str) -> Result<Option<Entity>, FailedTo> {
    let mut connection = Connection::open(path).map_err(|_| sqlite::FailedTo::OpenConnection)?;
    let mut transaction = connection
        .transaction()
        .map_err(|sqlite_error| sqlite::FailedTo::BeginTransaction(sqlite_error))?;
    transaction.set_drop_behavior(DropBehavior::Commit);
    let mut entity = transaction
        .query_one(SELECT_ENTITY_BY_ID, [entity_id], sqlite::map::row_to_entity)
        .optional()
        .map_err(|sqlite_error| sqlite::FailedTo::ExecuteQuery(sqlite_error))?;
    if let Some(ref mut entity) = entity {
        sqlite::load::attributes(&transaction, entity)?;
        entity.state = EntityState::Loaded;
    }
    Ok(entity)
}
