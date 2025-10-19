use crate::*;
use rusqlite::*;

pub fn run<'a>(filter: &'a Filter) -> Result<Vec<Box<dyn ToSql + 'a>>, FailedTo> {
    let mut params: Vec<Box<dyn ToSql>> = Vec::new();
    for condition in filter.conditions() {
        let (_, _, condition) = condition;
        match condition {
            Condition::Bool(value) => params.push(Box::new(value)),
            Condition::SignedInt(value) => params.push(Box::new(value)),
            Condition::UnsignedInt(value) => params.push(Box::new(value)),
            Condition::Text(value) => params.push(Box::new(value)),
        }
    }
    Ok(params)
}
