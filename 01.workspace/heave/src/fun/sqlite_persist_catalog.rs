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
    INSERT INTO entity (id, class)
    VALUES (?1, ?2);
"#;

const INSERT_ATTRIBUTE_STATEMENT_TEMPLATE: &str = r#"
    INSERT INTO attribute (id, entity_id, {column})
    VALUES (?1, ?2, ?3);
"#;

fn write_attribute(attribute: &Attribute, entity: &Entity, transaction: &rusqlite::Transaction) {
    let column = column(&attribute.value);
    let attribute_values = (&attribute.id, &entity.id, &attribute.value.to_string());
    let insert_attribute_statement =
        INSERT_ATTRIBUTE_STATEMENT_TEMPLATE.replace("{column}", column);
    let _ = transaction.execute(&insert_attribute_statement, attribute_values);
}

fn write_entity(entity: &Entity, transaction: &rusqlite::Transaction) {
    let entity_id = [&entity.id];
    let entity_values = (&entity.id, &entity.class);
    let _ = transaction.execute(DELETE_ENTITY_STATEMENT, entity_id);
    let _ = transaction.execute(INSERT_ENTITY_STATEMENT, entity_values);
    for (_key, attribute) in entity.attributes.iter() {
        write_attribute(attribute, entity, transaction);
    }
}

pub fn run(path: &path::Path, catalog: &Catalog) {
    let mut connection = Connection::open(path).unwrap();
    let transaction = connection.transaction().unwrap();
    for (_key, entity) in catalog
        .items
        .iter()
        .filter(|item| item.1.state == EntityState::New)
    {
        write_entity(entity, &transaction);
    }
    let _ = transaction.commit();
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    #[test]
    fn test_call() {
        let tempfile = tempfile::NamedTempFile::new().unwrap();
        let path = tempfile.path();
        let entity = Entity::new("test");
        let mut catalog = Catalog::new("");
        catalog.insert(entity);
        sqlite::init::db(path);
        run(path, &catalog);
    }
}
