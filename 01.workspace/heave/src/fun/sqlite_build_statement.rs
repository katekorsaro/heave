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
            (Comparison::Equal, Condition::Bool(_)) => {
                compose_fragment(name, "value_bool", "=", i + 1)
            }
            (Comparison::Equal, Condition::SignedInt(_)) => {
                compose_fragment(name, "value_int", "=", i + 1)
            }
            (Comparison::Greater, Condition::SignedInt(_)) => {
                compose_fragment(name, "value_int", ">", i + 1)
            }
            (Comparison::Lesser, Condition::SignedInt(_)) => {
                compose_fragment(name, "value_int", "<", i + 1)
            }
            _ => todo!(),
        };
        statement.push_str(&fragment);
    }
    Ok(statement)
}
