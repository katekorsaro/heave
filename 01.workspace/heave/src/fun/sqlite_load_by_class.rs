use crate::*;
use rusqlite::*;

const SELECT_ENTITY_BY_CLASS: &str = r#"
    SELECT * FROM entity
    WHERE class = ?1;
"#;

pub fn run(path: &path::Path, entity_class: &str) -> Vec<Entity> {
    let mut entities = Vec::<Entity>::new();
    let connection = Connection::open(path).unwrap();
    let mut statement = connection.prepare(SELECT_ENTITY_BY_CLASS).unwrap();
    let result = statement
        .query_map([entity_class], sqlite::map::row_to_entity)
        .unwrap();
    for entity in result {
        let mut entity = entity.unwrap();
        sqlite::load::attributes(&connection, &mut entity);
        entities.push(entity);
    }
    entities
}
