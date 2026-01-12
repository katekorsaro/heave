use crate::*;

pub fn run(row: &rusqlite::Row) -> rusqlite::Result<Entity> {
    let id: String = row.get(0)?;
    let class: String = row.get(1)?;
    let subclass: Option<String> = row.get(2)?;
    let ref_date: Option<u32> = row.get(3)?;
    let entity = Entity {
        id,
        state: EntityState::Loaded,
        class,
        subclass,
        ref_date,
        attributes: std::collections::HashMap::new(),
    };
    Ok(entity)
}
