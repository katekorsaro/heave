use crate::*;

impl rusqlite::ToSql for Value {
    fn to_sql(&self) -> std::result::Result<rusqlite::types::ToSqlOutput<'_>, rusqlite::Error> {
        match self {
            Value::Bool(value) => Ok(rusqlite::types::ToSqlOutput::from(*value as i64)),
            Value::Real(value) => Ok(rusqlite::types::ToSqlOutput::from(*value)),
            Value::SignedInt(value) => Ok(rusqlite::types::ToSqlOutput::from(*value)),
            Value::Text(value) => Ok(rusqlite::types::ToSqlOutput::from(value.to_string())),
            Value::UnsignedInt(value) => Ok(rusqlite::types::ToSqlOutput::from(*value as i64)),
        }
    }
}
