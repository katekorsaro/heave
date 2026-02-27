use crate::*;
use collections::*;
use rusqlite::*;

fn column(value: &Value) -> &'static str {
    match value {
        Value::SignedInt(_) => "value_int",
        Value::UnsignedInt(_) => "value_uint",
        Value::Real(_) => "value_real",
        Value::Text(_) => "value_text",
        Value::Bool(_) => "value_bool",
    }
}

const DELETE_ENTITY_STATEMENT: &str = r#"
    DELETE FROM entity
    WHERE entity.id = ?1;
"#;

const INSERT_ENTITY_STATEMENT: &str = r#"
    INSERT INTO entity (id, class, subclass, ref_date)
    VALUES (?1, ?2, ?3, ?4);
"#;

const INSERT_ATTRIBUTE_STATEMENT_TEMPLATE: &str = r#"
    INSERT INTO attribute (id, entity_id, {column})
    VALUES (?1, ?2, ?3);
"#;

fn write_attribute(
    attribute: &Attribute,
    entity: &Entity,
    transaction: &rusqlite::Transaction,
) -> result::Result<(), FailedTo> {
    let column = column(&attribute.value);
    let attribute_values = (&attribute.id, &entity.id, &attribute.value);
    let insert_attribute_statement =
        INSERT_ATTRIBUTE_STATEMENT_TEMPLATE.replace("{column}", column);
    transaction
        .execute(&insert_attribute_statement, attribute_values)
        .map_err(|sqlite_error| sqlite::FailedTo::ExecuteStatement(sqlite_error))?;
    Ok(())
}

fn delete_entity(entity: &Entity, transaction: &rusqlite::Transaction) -> Result<(), FailedTo> {
    let entity_id = [&entity.id];
    transaction
        .execute(DELETE_ENTITY_STATEMENT, entity_id)
        .map_err(|sqlite_error| sqlite::FailedTo::ExecuteStatement(sqlite_error))?;
    Ok(())
}

fn write_entity(entity: &Entity, transaction: &rusqlite::Transaction) -> Result<(), FailedTo> {
    let entity_id = [&entity.id];
    let entity_values = (&entity.id, &entity.class, &entity.subclass, entity.ref_date);
    transaction
        .execute(DELETE_ENTITY_STATEMENT, entity_id)
        .map_err(|sqlite_error| sqlite::FailedTo::ExecuteStatement(sqlite_error))?;
    transaction
        .execute(INSERT_ENTITY_STATEMENT, entity_values)
        .map_err(|sqlite_error| sqlite::FailedTo::ExecuteStatement(sqlite_error))?;
    for attribute in entity.attributes.values() {
        write_attribute(attribute, entity, transaction)?;
    }
    Ok(())
}

pub fn run(path: &path::Path, items: &HashMap<String, Entity>) -> result::Result<(), FailedTo> {
    let mut connection = Connection::open(path).map_err(|_| sqlite::FailedTo::OpenConnection)?;
    let transaction = connection
        .transaction()
        .map_err(|sqlite_error| sqlite::FailedTo::BeginTransaction(sqlite_error))?;
    for entity in items.values().filter(|item| {
        item.state == EntityState::New
            || item.state == EntityState::Updated
            || item.state == EntityState::ToDelete
    }) {
        match entity.state {
            EntityState::New | EntityState::Updated => write_entity(entity, &transaction)?,
            EntityState::ToDelete => delete_entity(entity, &transaction)?,
            _ => unreachable!(),
        }
    }
    transaction
        .commit()
        .map_err(|sqlite_error| sqlite::FailedTo::CommitTransaction(sqlite_error))?;
    Ok(())
}
