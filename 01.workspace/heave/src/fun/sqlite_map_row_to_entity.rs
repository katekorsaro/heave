use crate::*;

pub fn run(row: &rusqlite::Row) -> rusqlite::Result<Entity> {
    let id: String = row.get(0)?;
    let class: String = row.get(1)?;
    let ref_date: Option<u64> = row.get(2)?;
    let entity = Entity {
        id,
        state: EntityState::Loaded,
        class,
        ref_date,
        attributes: std::collections::HashMap::new(),
    };
    Ok(entity)
}

// #[cfg(test)]
// mod unit_tests {
// use super::*;
// }
