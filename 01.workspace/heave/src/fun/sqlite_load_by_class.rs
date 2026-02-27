use crate::*;
use rusqlite::*;

const SELECT_ENTITY_BY_CLASS: &str = r#"
    SELECT id, class, subclass, ref_date FROM entity
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
