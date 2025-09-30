use crate::*;
use rusqlite::*;

const SELECT_ATTRIBUTE_BY_FK: &str = r#"
    SELECT * FROM attribute
    WHERE entity_id = ?1;
"#;

pub fn run(connection: &Connection, entity: &mut Entity) {
    let mut select_attributes_statement = connection.prepare(SELECT_ATTRIBUTE_BY_FK).unwrap();
    let attributes = select_attributes_statement
        .query_map([&entity.id], sqlite::map::row_to_attribute)
        .unwrap();
    for attribute in attributes {
        let attribute = attribute.unwrap();
        entity.attributes.insert(attribute.id.clone(), attribute);
    }
}
