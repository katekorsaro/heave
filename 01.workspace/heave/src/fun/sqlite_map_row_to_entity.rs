use crate::*;

pub fn run(row: &rusqlite::Row) -> rusqlite::Result<Entity> {
    let id: String = row.get(0)?;
    let class: String = row.get(1)?;
    let ref_date: u64 = row.get(2)?;
    Ok(Entity::default()
        .with_id(&id)
        .with_class(&class)
        .with_ref_date(ref_date))
}

// #[cfg(test)]
// mod unit_tests {
// use super::*;
// }
