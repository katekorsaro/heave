use crate::*;
use rusqlite::*;

pub fn run(path: &path::Path) -> result::Result<(), FailedTo> {
    let init_statement = r#"
        CREATE TABLE IF NOT EXISTS entity (
            id TEXT PRIMARY KEY,
            class TEXT NOT NULL,
            subclass TEXT,
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
        CREATE INDEX IF NOT EXISTS entity_subclass ON entity (subclass);
        CREATE INDEX IF NOT EXISTS attribute_id ON attribute (id);
        CREATE INDEX IF NOT EXISTS attribute_entity_id ON attribute (entity_id);
        "#;
    let connection = Connection::open(path)
        .map_err(sqlite::FailedTo::OpenConnection)?;
    connection
        .execute_batch(init_statement)
        .map_err(sqlite::FailedTo::ExecuteBatch)?;
    Ok(())
}
