use crate::*;
use rusqlite::*;

pub fn run(path: &path::Path) -> result::Result<(), FailedTo> {
    let init_statement = r#"
        CREATE TABLE IF NOT EXISTS entity (
            id TEXT PRIMARY KEY,
            class TEXT NOT NULL,
            ref_date INTEGER
        );
        CREATE TABLE IF NOT EXISTS attribute (
            id TEXT,
            entity_id TEXT,
            value_int INTEGER,
            value_uint INTEGER,
            value_real REAL,
            value_text TEXT,
            value_bool BOOL,
            CONSTRAINT pk_id PRIMARY KEY (id, entity_id),
            CONSTRAINT fk_entity_id FOREIGN KEY (entity_id) REFERENCES entity (id) ON DELETE CASCADE ON UPDATE CASCADE
        );
        CREATE INDEX IF NOT EXISTS entity_class ON entity (class);
        CREATE INDEX IF NOT EXISTS attribute_id ON attribute (id);
        "#;
    let connection = Connection::open(path).map_err(|_| sqlite::FailedTo::OpenConnection)?;
    connection
        .execute_batch(init_statement)
        .map_err(|_| sqlite::FailedTo::ExecuteBatch)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn init_db_should_create_tables_and_indexes_on_new_db() {
        // This test verifies that a new database is correctly initialized with the necessary tables ('entity', 'attribute') and indexes.
        todo!();
    }
    #[test]
    fn init_db_should_be_idempotent_and_not_fail_on_existing_db() {
        // This test ensures that initializing an already existing and initialized database does not cause errors.
        todo!();
    }
    #[test]
    fn init_db_should_fail_gracefully_on_invalid_path() {
        // This test checks that the function returns an error when provided with an invalid or inaccessible file path.
        todo!();
    }
}
