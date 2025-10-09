use crate::*;
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
    INSERT INTO entity (id, class, ref_date)
    VALUES (?1, ?2, ?3);
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
    let attribute_values = (&attribute.id, &entity.id, &attribute.value.to_string());
    let insert_attribute_statement =
        INSERT_ATTRIBUTE_STATEMENT_TEMPLATE.replace("{column}", column);
    transaction
        .execute(&insert_attribute_statement, attribute_values)
        .map_err(|_| sqlite::FailedTo::ExecuteStatement)?;
    Ok(())
}

fn delete_entity(entity: &Entity, transaction: &rusqlite::Transaction) -> Result<(), FailedTo> {
    let entity_id = [&entity.id];
    transaction
        .execute(DELETE_ENTITY_STATEMENT, entity_id)
        .map_err(|_| sqlite::FailedTo::ExecuteStatement)?;
    Ok(())
}

fn write_entity(entity: &Entity, transaction: &rusqlite::Transaction) -> Result<(), FailedTo> {
    let entity_id = [&entity.id];
    let entity_values = (&entity.id, &entity.class, entity.ref_date);
    transaction
        .execute(DELETE_ENTITY_STATEMENT, entity_id)
        .map_err(|_| sqlite::FailedTo::ExecuteStatement)?;
    transaction
        .execute(INSERT_ENTITY_STATEMENT, entity_values)
        .map_err(|_| sqlite::FailedTo::ExecuteStatement)?;
    for attribute in entity.attributes.values() {
        write_attribute(attribute, entity, transaction)?;
    }
    Ok(())
}

pub fn run(path: &path::Path, catalog: &Catalog) -> result::Result<(), FailedTo> {
    let mut connection = Connection::open(path).map_err(|_| sqlite::FailedTo::OpenConnection)?;
    let transaction = connection
        .transaction()
        .map_err(|_| sqlite::FailedTo::BeginTransaction)?;
    for entity in catalog
        .items
        .values()
        .filter(|item| item.state == EntityState::New || item.state == EntityState::ToDelete)
    {
        match entity.state {
            EntityState::New => write_entity(entity, &transaction)?,
            EntityState::ToDelete => delete_entity(entity, &transaction)?,
            _ => unreachable!(),
        }
    }
    transaction
        .commit()
        .map_err(|_| sqlite::FailedTo::CommitTransaction)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn persist_should_insert_new_entities_and_attributes() {
        // Verifies that entities marked as 'New' in the catalog are inserted into the database, along with all their attributes.
        todo!();
    }
    #[test]
    fn persist_should_delete_entities_marked_for_deletion() {
        // Ensures that entities marked as 'ToDelete' are correctly removed from the database.
        todo!();
    }
    #[test]
    fn persist_should_handle_a_mix_of_new_and_deleted_entities() {
        // Tests the function's ability to handle a batch operation involving both new entities to be inserted and existing ones to be deleted.
        todo!();
    }
    #[test]
    fn persist_should_not_affect_unmodified_entities() {
        // Verifies that entities in the catalog that are not marked as 'New' or 'ToDelete' remain untouched in the database.
        todo!();
    }
    #[test]
    fn persist_should_rollback_transaction_on_failure() {
        // Ensures that if any part of the persistence process fails, the entire transaction is rolled back, leaving the database state unchanged.
        todo!();
    }
}
