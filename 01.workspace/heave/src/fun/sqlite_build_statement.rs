use crate::*;

const BASE_SELECT: &str = r#"SELECT * FROM entity WHERE 1=1"#;

pub fn run(filter: &Filter) -> Result<String, FailedTo> {
    let mut statement = String::from(BASE_SELECT);
    for (i, (name, _comparison, condition)) in filter.conditions().enumerate() {
        let fragment = match (_comparison, condition) {
            (_, Condition::Bool(_)) => format!(
                " AND id IN (SELECT entity_id FROM attribute WHERE id = '{}' AND value_bool = ?{})",
                name,
                i + 1
            ),
            (Comparison::Equal, Condition::SignedInt(_)) => format!(
                " AND id IN (SELECT entity_id FROM attribute WHERE id = '{}' AND value_int = ?{})",
                name,
                i + 1
            ),
            _ => todo!(),
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
    #[test]
    fn builds_statement_with_one_signed_int_equal_condition() {
        let filter = Filter::new().with_signed_int("level", Comparison::Equal, 10);
        let statement = run(&filter).unwrap();
        assert_eq!(
            statement,
            "SELECT * FROM entity WHERE 1=1 AND id IN (SELECT entity_id FROM attribute WHERE id = 'level' AND value_int = ?1)"
        );
    }
    #[test]
    fn builds_statement_with_multiple_signed_int_equal_conditions() {
        let filter = Filter::new()
            .with_signed_int("level", Comparison::Equal, 10)
            .with_signed_int("score", Comparison::Equal, 100);
        let statement = run(&filter).unwrap();
        assert_eq!(
            statement,
            "SELECT * FROM entity WHERE 1=1 AND id IN (SELECT entity_id FROM attribute WHERE id = 'level' AND value_int = ?1) AND id IN (SELECT entity_id FROM attribute WHERE id = 'score' AND value_int = ?2)"
        );
    }
    #[test]
    fn builds_statement_with_mixed_bool_and_signed_int_conditions() {
        let filter = Filter::new().with_bool("is_active", true).with_signed_int(
            "level",
            Comparison::Equal,
            5,
        );
        let statement = run(&filter).unwrap();
        assert_eq!(
            statement,
            "SELECT * FROM entity WHERE 1=1 AND id IN (SELECT entity_id FROM attribute WHERE id = 'is_active' AND value_bool = ?1) AND id IN (SELECT entity_id FROM attribute WHERE id = 'level' AND value_int = ?2)"
        );
    }
}
