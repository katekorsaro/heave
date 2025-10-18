use crate::*;

const BASE_SELECT: &str = r#"SELECT * FROM entity WHERE 1=1"#;

pub fn run(filter: &Filter) -> Result<String, FailedTo> {
    let mut statement = String::from(BASE_SELECT);
    for (i, (name, condition)) in filter.conditions().enumerate() {
        let fragment = match *condition {
            Condition::Bool(_) => format!(
                " AND id IN (SELECT entity_id FROM attribute WHERE id = '{}' AND value_bool = ?{})",
                name,
                i + 1
            ),
        };
        statement.push_str(&fragment);
    }
    Ok(statement)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Filter;

    #[test]
    fn builds_statement_with_no_conditions() {
        let filter = Filter::new();
        let statement = run(&filter).unwrap();
        assert_eq!(statement, BASE_SELECT);
    }

    #[test]
    fn builds_statement_with_one_bool_condition() {
        let filter = Filter::new().with_bool("is_active", true);
        let statement = run(&filter).unwrap();
        assert_eq!(
            statement,
            "SELECT * FROM entity WHERE 1=1 AND id IN (SELECT entity_id FROM attribute WHERE id = 'is_active' AND value_bool = ?1)"
        );
    }

    #[test]
    fn builds_statement_with_multiple_bool_conditions() {
        let filter = Filter::new()
            .with_bool("is_active", true)
            .with_bool("is_deleted", false);
        let statement = run(&filter).unwrap();
        assert_eq!(
            statement,
            "SELECT * FROM entity WHERE 1=1 AND id IN (SELECT entity_id FROM attribute WHERE id = 'is_active' AND value_bool = ?1) AND id IN (SELECT entity_id FROM attribute WHERE id = 'is_deleted' AND value_bool = ?2)"
        );
    }
}
