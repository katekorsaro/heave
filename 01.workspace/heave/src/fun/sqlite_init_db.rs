use crate::*;
use rusqlite::*;

pub fn run(path: &path::Path) {
    let init_statement = r#"
        CREATE TABLE IF NOT EXISTS entity (
            id TEXT PRIMARY KEY,
            class TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS attribute (
            id TEXT PRIMARY KEY,
            value_int INTEGER,
            value_uint INTEGER,
            value_real REAL,
            value_text TEXT,
            value_bool BOOL,
            entity_id TEXT,
            CONSTRAINT fk_entity_id FOREIGN KEY (entity_id) REFERENCES entity (id) ON DELETE CASCADE ON UPDATE CASCADE
        );
        "#;
    let connection = Connection::open(path).unwrap();
    let _result = connection.execute_batch(init_statement);
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    #[test]
    fn test_call() {
        let tempfile = tempfile::NamedTempFile::new().unwrap();
        let path = tempfile.path();
        run(path);
    }
}
