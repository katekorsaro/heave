use crate::*;

pub fn run(row: &rusqlite::Row) -> rusqlite::Result<Attribute> {
    let id: String = row.get(0)?;
    let _entity_id: String = row.get(1)?;
    let signed_int: Option<i64> = row.get(2)?;
    let unsigned_int: Option<u64> = row.get(3)?;
    let real: Option<f64> = row.get(4)?;
    let text: Option<String> = row.get(5)?;
    let bool: Option<bool> = row.get(6)?;
    let value: Value = match (signed_int, unsigned_int, real, text, bool) {
        (Some(value), None, None, None, None) => Value::SignedInt(value),
        (None, Some(value), None, None, None) => Value::UnsignedInt(value),
        (None, None, Some(value), None, None) => Value::Real(value),
        (None, None, None, Some(value), None) => Value::Text(value),
        (None, None, None, None, Some(value)) => Value::Bool(value),
        _ => panic!(),
    };
    Ok(Attribute { id, value })
}
