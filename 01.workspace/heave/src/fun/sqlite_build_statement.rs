use crate::*;

const BASE_SELECT: &str = r#"SELECT * FROM entity"#;
const INNER_JOIN_FRAGMENT: &str = r#"
    INNER JOIN attribute
    ON entity.id = attribute.entity_id
    AND attribute.id = '{attribute_id}'
    AND {field} {op} ?{index}
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
            _ => todo!(),
        };
        statement.push_str(&fragment);
    }
    Ok(statement)
}
