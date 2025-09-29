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

pub fn run(path: &path::Path, catalog: &Catalog) {
    let mut connection = Connection::open(path).unwrap();
    let delete_entity_statement = r#"
        DELETE FROM entity
        WHERE entity.id = ?1;
        "#;
    let insert_entity_statement = r#"
        INSERT INTO entity (id, class)
        VALUES (?1, ?2);
        "#;
    let insert_attribute_statement_template = r#"
        INSERT INTO attribute (id, entity_id, {column})
        VALUES (?1, ?2, ?3);
        "#;
    let transaction = connection.transaction().unwrap();
    for (_key, entity) in catalog.items.iter() {
        let _ = transaction.execute(delete_entity_statement, [&entity.id]);
        let _ = transaction.execute(insert_entity_statement, (&entity.id, &entity.class));
        for (_key, attribute) in entity.attributes.iter() {
            let insert_attribute_statement =
                insert_attribute_statement_template.replace("{column}", column(&attribute.value));
            let _ = transaction.execute(
                &insert_attribute_statement,
                (&attribute.id, &entity.id, &attribute.value.to_string()),
            );
        }
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
