use crate::*;
use rusqlite::*;

const SELECT_ENTITY_BY_ID: &str = r#"
    SELECT * FROM entity
    WHERE id = ?1;
"#;

pub fn run(path: &path::Path, entity_id: &str) -> Option<Entity> {
    let mut connection = Connection::open(path).unwrap();
    let mut transaction = connection.transaction().unwrap();
    transaction.set_drop_behavior(DropBehavior::Commit);
    let result = transaction
        .query_one(SELECT_ENTITY_BY_ID, [entity_id], sqlite::map::row_to_entity)
        .optional();
    let mut entity = result.unwrap();
    if let Some(ref mut entity) = entity {
        sqlite::load::attributes(&transaction, entity);
        entity.state = EntityState::Loaded;
    }
    entity
}
