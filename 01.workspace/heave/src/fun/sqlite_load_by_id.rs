use crate::*;
use rusqlite::*;

const SELECT_ENTITY_BY_ID: &str = r#"
    SELECT * FROM entity
    WHERE id = ?1;
"#;

const SELECT_ATTRIBUTE_BY_FK: &str = r#"
    SELECT * FROM attribute
    WHERE entity_id = ?1;
"#;

pub fn run(path: &path::Path, entity_id: &str) -> Option<Entity> {
    let connection = Connection::open(path).unwrap();
    let result = connection
        .query_one(SELECT_ENTITY_BY_ID, [entity_id], sqlite::map::row_to_entity)
        .optional();
    let mut entity = result.unwrap();
    if let Some(ref mut entity) = entity {
        let mut select_attributes_statement = connection.prepare(SELECT_ATTRIBUTE_BY_FK).unwrap();
        let attributes = select_attributes_statement
            .query_map([&entity.id], sqlite::map::row_to_attribute)
            .unwrap();
        for attribute in attributes {
            let attribute = attribute.unwrap();
            entity.attributes.insert(attribute.id.clone(), attribute);
        }
    }
    entity
}

// #[cfg(test)]
// mod unit_tests {
// use super::*;
// }
