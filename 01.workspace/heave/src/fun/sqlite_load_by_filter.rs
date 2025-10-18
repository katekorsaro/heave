use crate::*;
use rusqlite::*;

pub fn run(path: &path::Path, filter: &Filter) -> Result<Vec<Entity>, FailedTo> {
    let mut entities = Vec::<Entity>::new();
    let mut connection = Connection::open(path).map_err(|_| sqlite::FailedTo::OpenConnection)?;
    let mut transaction = connection
        .transaction()
        .map_err(|_| sqlite::FailedTo::BeginTransaction)?;
    transaction.set_drop_behavior(DropBehavior::Commit);
    let select_entity_by_filter = sqlite::build::statement(filter)?;
    let params = sqlite::build::params(filter)?;
    let params: Vec<&dyn ToSql> = params.iter().map(|p| p.as_ref() as &dyn ToSql).collect();
    let mut statement = transaction
        .prepare(&select_entity_by_filter)
        .map_err(|_| sqlite::FailedTo::PrepareStatement)?;
    let result = statement
        .query_map(&params[..], sqlite::map::row_to_entity)
        .map_err(|_| sqlite::FailedTo::ExecuteQuery)?;
    for entity in result {
        let mut entity = entity.map_err(|_| FailedTo::MapEntity)?;
        sqlite::load::attributes(&transaction, &mut entity)?;
        entity.state = EntityState::Loaded;
        entities.push(entity);
    }
    Ok(entities)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Filter, fun};
    use std::{fs, path::Path};
    fn setup_db(db_path: &Path) {
        let _ = fs::remove_file(db_path);
        fun::sqlite_init_db::run(db_path).unwrap();
        let conn = Connection::open(db_path).unwrap();
        conn.execute("INSERT INTO entity (id, class) VALUES ('e1', 'c1')", [])
            .unwrap();
        conn.execute(
            "INSERT INTO attribute (id, entity_id, value_bool) VALUES ('active', 'e1', 1)",
            [],
        )
        .unwrap();
        conn.execute("INSERT INTO entity (id, class) VALUES ('e2', 'c1')", [])
            .unwrap();
        conn.execute(
            "INSERT INTO attribute (id, entity_id, value_bool) VALUES ('active', 'e2', 0)",
            [],
        )
        .unwrap();
        conn.execute("INSERT INTO entity (id, class) VALUES ('e3', 'c2')", [])
            .unwrap();
        conn.execute(
            "INSERT INTO attribute (id, entity_id, value_bool) VALUES ('active', 'e3', 1)",
            [],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO attribute (id, entity_id, value_bool) VALUES ('deleted', 'e3', 1)",
            [],
        )
        .unwrap();
    }
    // Verifies that entities matching the filter criteria are correctly loaded.
    #[test]
    fn loads_entities_with_matching_filter() {
        let db_path = Path::new("test_load_matching.db");
        setup_db(db_path);
        let filter = Filter::new().with_bool("active", true);
        let entities = run(db_path, &filter).unwrap();
        assert_eq!(entities.len(), 2);
        let ids: Vec<_> = entities.iter().map(|e| e.id.clone()).collect();
        assert!(ids.contains(&"e1".to_string()));
        assert!(ids.contains(&"e3".to_string()));
        fs::remove_file(db_path).unwrap();
    }
    // Ensures that an empty vector is returned when no entities match the filter.
    #[test]
    fn returns_empty_vec_for_no_matching_entities() {
        let db_path = Path::new("test_load_no_matching.db");
        setup_db(db_path);
        let filter = Filter::new().with_bool("non_existent_attr", true);
        let entities = run(db_path, &filter).unwrap();
        assert!(entities.is_empty());
        fs::remove_file(db_path).unwrap();
    }
    // Tests that all entities are returned when an empty filter is provided.
    #[test]
    fn handles_empty_filter() {
        let db_path = Path::new("test_load_empty_filter.db");
        setup_db(db_path);
        let filter = Filter::new();
        let entities = run(db_path, &filter).unwrap();
        assert_eq!(entities.len(), 3);
        fs::remove_file(db_path).unwrap();
    }
    // Checks that the function returns an appropriate error on a database connection failure.
    #[test]
    fn fails_gracefully_on_db_error() {
        let invalid_path = Path::new("non_existing/path");
        let filter = Filter::new();
        let result = run(invalid_path, &filter);
        assert!(result.is_err());
    }
    // Confirms that loaded entities include all their associated attributes.
    #[test]
    fn fully_loads_entities_with_attributes() {
        let db_path = Path::new("test_load_fully.db");
        setup_db(db_path);
        let filter = Filter::new().with_bool("deleted", true);
        let entities = run(db_path, &filter).unwrap();
        assert_eq!(entities.len(), 1);
        let entity = &entities[0];
        assert_eq!(entity.id, "e3");
        assert_eq!(entity.attributes.len(), 2);
        assert!(entity.attributes.contains_key("active"));
        assert!(entity.attributes.contains_key("deleted"));
        fs::remove_file(db_path).unwrap();
    }
}
