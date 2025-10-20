use crate::*;
use rusqlite::*;

pub fn run<'a>(filter: &'a Filter) -> Result<Vec<Box<dyn ToSql + 'a>>, FailedTo> {
    let mut params: Vec<Box<dyn ToSql>> = Vec::new();
    for condition in filter.conditions() {
        let (_, comparison, condition) = condition;
        match (comparison, condition) {
            // BOOL
            (Comparison::Equal, Condition::Bool(value)) => params.push(Box::new(value)),
            // SIGNED INT
            (
                Comparison::Equal
                | Comparison::Greater
                | Comparison::Lesser
                | Comparison::GreaterOrEqual
                | Comparison::LesserOrEqual,
                Condition::SignedInt(value),
            ) => params.push(Box::new(value)),
            // UNSIGNED INT
            (
                Comparison::Equal
                | Comparison::Greater
                | Comparison::Lesser
                | Comparison::GreaterOrEqual
                | Comparison::LesserOrEqual,
                Condition::UnsignedInt(value),
            ) => params.push(Box::new(value)),
            // REAL
            (
                Comparison::Equal
                | Comparison::Greater
                | Comparison::Lesser
                | Comparison::GreaterOrEqual
                | Comparison::LesserOrEqual,
                Condition::Real(value),
            ) => params.push(Box::new(value)),
            // TEXT
            (Comparison::IsExactly, Condition::Text(value)) => params.push(Box::new(value)),
            (Comparison::StartsWith, Condition::Text(value)) => {
                params.push(Box::new(format!("{}%", value)))
            }
            (Comparison::EndsWith, Condition::Text(value)) => {
                params.push(Box::new(format!("%{}", value)))
            }
            (Comparison::Contains, Condition::Text(value)) => {
                params.push(Box::new(format!("%{}%", value)))
            }
            // ERROR
            _ => return Err(FailedTo::ComposeFilter),
        }
    }
    Ok(params)
}
