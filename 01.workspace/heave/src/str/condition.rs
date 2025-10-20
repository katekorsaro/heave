/// Represents the value part of a `Filter` condition.
///
/// Each variant holds a specific data type to be used in a comparison when
/// querying the database.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum E<'a> {
    /// A boolean value (`true` or `false`).
    Bool(bool),
    /// A floating-point number (`f64`).
    Real(f64),
    /// A signed integer (`i64`).
    SignedInt(i64),
    /// A text value (`&str`).
    Text(&'a str),
    /// An unsigned integer (`u64`), stored as `i64` for SQLite compatibility.
    UnsignedInt(i64),
}
