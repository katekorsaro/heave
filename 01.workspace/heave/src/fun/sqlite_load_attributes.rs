use crate::*;
use rusqlite::*;

const SELECT_ATTRIBUTE_BY_FK: &str = r#"
    SELECT
        id,
        entity_id,
        value_int,
        value_uint,
        value_real,
        value_text,
        value_bool
    FROM attribute
    WHERE entity_id = ?1;
"#;

pub fn run(transaction: &Transaction, entity: &mut Entity) -> Result<(), FailedTo> {
    let mut select_attributes_statement = transaction
        .prepare(SELECT_ATTRIBUTE_BY_FK)
        .map_err(sqlite::FailedTo::PrepareStatement)?;
    let attributes = select_attributes_statement
        .query_map([&entity.id], sqlite::map::row_to_attribute)
        .map_err(sqlite::FailedTo::ExecuteQuery)?;
    for attribute in attributes {
        let attribute = attribute.map_err(|_| FailedTo::MapAttribute)?;
        entity.attributes.insert(attribute.id.clone(), attribute);
    }
    Ok(())
}
