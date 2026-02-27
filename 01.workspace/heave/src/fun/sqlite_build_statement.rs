use crate::*;

const BASE_SELECT: &str = r#"SELECT * FROM entity"#;
const INNER_JOIN_FRAGMENT: &str = r#"
    INNER JOIN attribute as attribute_{index}
    ON entity.id = attribute_{index}.entity_id
    AND attribute_{index}.id = '{attribute_id}'
    AND attribute_{index}.{field} {op} ?{index}
"#;
const WHERE: &str = r#" WHERE 1=1"#;

fn compose_fragment(name: &str, field: &str, op: &str, index: usize) -> String {
    INNER_JOIN_FRAGMENT
        .replace("{attribute_id}", name)
        .replace("{field}", field)
        .replace("{op}", op)
        .replace("{index}", &index.to_string())
}

fn from_condition(
    i: usize,
    name: &str,
    comparison: &Comparison,
    condition: &Condition,
) -> Result<String, FailedTo> {
    let fragment = match (comparison, condition) {
        // BOOL
        (Comparison::Equal, Condition::Bool(_)) => compose_fragment(name, "value_bool", "=", i),
        (_, Condition::Bool(_)) => return Err(FailedTo::ComposeFilter),
        // SIGNED INT
        (Comparison::Equal, Condition::SignedInt(_)) => compose_fragment(name, "value_int", "=", i),
        (Comparison::Greater, Condition::SignedInt(_)) => {
            compose_fragment(name, "value_int", ">", i)
        }
        (Comparison::Lesser, Condition::SignedInt(_)) => {
            compose_fragment(name, "value_int", "<", i)
        }
        (Comparison::GreaterOrEqual, Condition::SignedInt(_)) => {
            compose_fragment(name, "value_int", ">=", i)
        }
        (Comparison::LesserOrEqual, Condition::SignedInt(_)) => {
            compose_fragment(name, "value_int", "<=", i)
        }
        (_, Condition::SignedInt(_)) => return Err(FailedTo::ComposeFilter),
        // UNSIGNED INT
        (Comparison::Equal, Condition::UnsignedInt(_)) => {
            compose_fragment(name, "value_uint", "=", i)
        }
        (Comparison::Greater, Condition::UnsignedInt(_)) => {
            compose_fragment(name, "value_uint", ">", i)
        }
        (Comparison::Lesser, Condition::UnsignedInt(_)) => {
            compose_fragment(name, "value_uint", "<", i)
        }
        (Comparison::GreaterOrEqual, Condition::UnsignedInt(_)) => {
            compose_fragment(name, "value_uint", ">=", i)
        }
        (Comparison::LesserOrEqual, Condition::UnsignedInt(_)) => {
            compose_fragment(name, "value_uint", "<=", i)
        }
        (_, Condition::UnsignedInt(_)) => return Err(FailedTo::ComposeFilter),
        // REAL
        (Comparison::Equal, Condition::Real(_)) => compose_fragment(name, "value_real", "=", i),
        (Comparison::Greater, Condition::Real(_)) => compose_fragment(name, "value_real", ">", i),
        (Comparison::Lesser, Condition::Real(_)) => compose_fragment(name, "value_real", "<", i),
        (Comparison::GreaterOrEqual, Condition::Real(_)) => {
            compose_fragment(name, "value_real", ">=", i)
        }
        (Comparison::LesserOrEqual, Condition::Real(_)) => {
            compose_fragment(name, "value_real", "<=", i)
        }
        (_, Condition::Real(_)) => return Err(FailedTo::ComposeFilter),
        // TEXT
        (Comparison::IsExactly, Condition::Text(_)) => compose_fragment(name, "value_text", "=", i),
        (
            Comparison::StartsWith | Comparison::EndsWith | Comparison::Contains,
            Condition::Text(_),
        ) => compose_fragment(name, "value_text", "LIKE", i),
        (_, Condition::Text(_)) => return Err(FailedTo::ComposeFilter),
    };
    Ok(fragment)
}

pub fn run(filter: &Filter) -> Result<String, FailedTo> {
    // base statement
    let mut statement = String::from(BASE_SELECT);
    let mut idx = 0;
    // for each condition add an inner join fragment
    for (i, (name, comparison, condition)) in filter.conditions().enumerate() {
        idx = i + 1;
        let fragment = from_condition(idx, name, comparison, condition)?;
        statement.push_str(&fragment);
    }
    // add a neutral where condition
    statement.push_str(WHERE);
    if filter.class().is_some() {
        idx += 1;
        let fragment = format!(" AND entity.class = ?{}", idx);
        statement.push_str(&fragment);
    }
    if filter.subclass().is_some() {
        idx += 1;
        let fragment = format!(" AND entity.subclass = ?{}", idx);
        statement.push_str(&fragment);
    }
    Ok(statement)
}
