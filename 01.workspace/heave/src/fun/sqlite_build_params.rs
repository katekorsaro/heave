use crate::*;
use rusqlite::*;

pub fn run(filter: &Filter) -> Result<Vec<Box<dyn ToSql>>, FailedTo> {
    let mut params: Vec<Box<dyn ToSql>> = Vec::new();
    for condition in filter.conditions() {
        match condition.2 {
            Condition::Bool(value) => params.push(Box::new(value)),
        }
    }
    Ok(params)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ensures that an empty vector is returned when the filter has no conditions.
    #[test]
    fn returns_empty_vec_for_empty_filter() {
        let filter = Filter::new();
        let params = run(&filter).unwrap();
        assert!(params.is_empty());
    }

    // Verifies that a single boolean condition is correctly converted to a ToSql parameter.
    #[test]
    fn returns_params_for_one_bool_condition() {
        let filter = Filter::new().with_bool("is_active", true);
        let params = run(&filter).unwrap();
        assert_eq!(params.len(), 1);
        assert_eq!(
            params[0].to_sql().unwrap(),
            rusqlite::types::ToSqlOutput::Owned(rusqlite::types::Value::Integer(1))
        );
    }

    // Checks that multiple boolean conditions are correctly converted and ordered.
    #[test]
    fn returns_params_for_multiple_bool_conditions() {
        let filter = Filter::new()
            .with_bool("is_active", true)
            .with_bool("is_deleted", false);
        let params = run(&filter).unwrap();
        assert_eq!(params.len(), 2);
        assert_eq!(
            params[0].to_sql().unwrap(),
            rusqlite::types::ToSqlOutput::Owned(rusqlite::types::Value::Integer(1))
        );
        assert_eq!(
            params[1].to_sql().unwrap(),
            rusqlite::types::ToSqlOutput::Owned(rusqlite::types::Value::Integer(0))
        );
    }
}
