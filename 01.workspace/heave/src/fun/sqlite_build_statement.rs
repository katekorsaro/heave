use crate::*;

const BASE_SELECT: &str = r#"SELECT * FROM entity"#;

pub fn run(filter: &Filter) -> Result<String, FailedTo> {
    let mut statement = String::from(BASE_SELECT);
    for (i, (name, _comparison, condition)) in filter.conditions().enumerate() {
        let fragment = match (_comparison, condition) {
            (_, Condition::Bool(_)) => format!(
                " INNER JOIN attribute ON entity.id = attribute.entity_id AND attribute.id = '{}' AND value_bool = ?{}",
                name,
                i + 1
            ),
            (Comparison::Equal, Condition::SignedInt(_)) => format!(
                " INNER JOIN attribute ON entity.id = attribute.entity_id AND attribute.id = '{}' AND value_int = ?{}",
                name,
                i + 1
            ),
            _ => todo!(),
        };
        statement.push_str(&fragment);
    }
    Ok(statement)
}
