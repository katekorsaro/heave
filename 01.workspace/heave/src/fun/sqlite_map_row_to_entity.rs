use crate::*;

pub fn run(row: &rusqlite::Row) -> rusqlite::Result<Entity> {
    let id: String = row.get(0)?;
    let class: String = row.get(1)?;
    Ok(Entity::default().with_id(&id).with_class(&class))
}

// #[cfg(test)]
// mod unit_tests {
// use super::*;
// }
