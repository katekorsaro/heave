use crate::*;
use rusqlite::*;

pub fn run(path: &path::Path, filter: &Filter) -> Result<Vec<Entity>, FailedTo> {
    let mut entities = Vec::<Entity>::new();
    let mut connection = Connection::open(path).map_err(|_| sqlite::FailedTo::OpenConnection)?;
    let mut transaction = connection
        .transaction()
        .map_err(|_| sqlite::FailedTo::BeginTransaction)?;
    transaction.set_drop_behavior(DropBehavior::Commit);
    let select_entity_by_filter = sqlite::build::statement(filter)?;
    let params = sqlite::build::params(filter)?;
    let params: Vec<&dyn ToSql> = params.iter().map(|p| p.as_ref() as &dyn ToSql).collect();
    let mut statement = transaction
        .prepare(&select_entity_by_filter)
        .map_err(|_| sqlite::FailedTo::PrepareStatement)?;
    let result = statement
        .query_map(&params[..], sqlite::map::row_to_entity)
        .map_err(|_| sqlite::FailedTo::ExecuteQuery)?;
    for entity in result {
        let mut entity = entity.map_err(|_| FailedTo::MapEntity)?;
        sqlite::load::attributes(&transaction, &mut entity)?;
        entity.state = EntityState::Loaded;
        entities.push(entity);
    }
    Ok(entities)
}
