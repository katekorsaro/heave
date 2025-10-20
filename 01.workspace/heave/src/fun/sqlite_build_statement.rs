use crate::*;

const BASE_SELECT: &str = r#"SELECT * FROM entity"#;
const INNER_JOIN_FRAGMENT: &str = r#"
    INNER JOIN attribute as attribute_{index}
    ON entity.id = attribute_{index}.entity_id
    AND attribute_{index}.id = '{attribute_id}'
    AND attribute_{index}.{field} {op} ?{index}
"#;

fn compose_fragment(name: &str, field: &str, op: &str, index: usize) -> String {
    INNER_JOIN_FRAGMENT
        .replace("{attribute_id}", name)
        .replace("{field}", field)
        .replace("{op}", op)
        .replace("{index}", &index.to_string())
}

pub fn run(filter: &Filter) -> Result<String, FailedTo> {
    let mut statement = String::from(BASE_SELECT);
    for (i, (name, comparison, condition)) in filter.conditions().enumerate() {
        let fragment = match (comparison, condition) {
            // BOOL
            (Comparison::Equal, Condition::Bool(_)) => {
                compose_fragment(name, "value_bool", "=", i + 1)
            }
            (_, Condition::Bool(_)) => return Err(FailedTo::ComposeFilter),
            // SIGNED INT
            (Comparison::Equal, Condition::SignedInt(_)) => {
                compose_fragment(name, "value_int", "=", i + 1)
            }
            (Comparison::Greater, Condition::SignedInt(_)) => {
                compose_fragment(name, "value_int", ">", i + 1)
            }
            (Comparison::Lesser, Condition::SignedInt(_)) => {
                compose_fragment(name, "value_int", "<", i + 1)
            }
            (Comparison::GreaterOrEqual, Condition::SignedInt(_)) => {
                compose_fragment(name, "value_int", ">=", i + 1)
            }
            (Comparison::LesserOrEqual, Condition::SignedInt(_)) => {
                compose_fragment(name, "value_int", "<=", i + 1)
            }
            (_, Condition::SignedInt(_)) => return Err(FailedTo::ComposeFilter),
            // UNSIGNED INT
            (Comparison::Equal, Condition::UnsignedInt(_)) => {
                compose_fragment(name, "value_uint", "=", i + 1)
            }
            (Comparison::Greater, Condition::UnsignedInt(_)) => {
                compose_fragment(name, "value_uint", ">", i + 1)
            }
            (Comparison::Lesser, Condition::UnsignedInt(_)) => {
                compose_fragment(name, "value_uint", "<", i + 1)
            }
            (Comparison::GreaterOrEqual, Condition::UnsignedInt(_)) => {
                compose_fragment(name, "value_uint", ">=", i + 1)
            }
            (Comparison::LesserOrEqual, Condition::UnsignedInt(_)) => {
                compose_fragment(name, "value_uint", "<=", i + 1)
            }
            (_, Condition::UnsignedInt(_)) => return Err(FailedTo::ComposeFilter),
            // REAL
            (Comparison::Equal, Condition::Real(_)) => {
                compose_fragment(name, "value_real", "=", i + 1)
            }
            (Comparison::Greater, Condition::Real(_)) => {
                compose_fragment(name, "value_real", ">", i + 1)
            }
            (Comparison::Lesser, Condition::Real(_)) => {
                compose_fragment(name, "value_real", "<", i + 1)
            }
            (Comparison::GreaterOrEqual, Condition::Real(_)) => {
                compose_fragment(name, "value_real", ">=", i + 1)
            }
            (Comparison::LesserOrEqual, Condition::Real(_)) => {
                compose_fragment(name, "value_real", "<=", i + 1)
            }
            (_, Condition::Real(_)) => return Err(FailedTo::ComposeFilter),
            // TEXT
            (Comparison::IsExactly, Condition::Text(_)) => {
                compose_fragment(name, "value_text", "LIKE", i + 1)
            }
            (
                Comparison::StartsWith | Comparison::EndsWith | Comparison::Contains,
                Condition::Text(_),
            ) => compose_fragment(name, "value_text", "LIKE", i + 1),
            (_, Condition::Text(_)) => return Err(FailedTo::ComposeFilter),
        };
        statement.push_str(&fragment);
    }
    Ok(statement)
}
