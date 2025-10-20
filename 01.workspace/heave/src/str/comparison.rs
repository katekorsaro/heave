/// Defines the comparison operators used in a `Filter` condition.
///
/// These operators are used to compare attribute values in the database when
/// loading entities via `load_by_filter`.
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Hash)]
pub enum E {
    /// Exact equality (`==`). Applicable to all value types.
    Equal,
    /// Greater than (`>`). Applicable to numeric types.
    Greater,
    /// Greater than or equal to (`>=`). Applicable to numeric types.
    GreaterOrEqual,
    /// Lesser than (`<`). Applicable to numeric types.
    Lesser,
    /// Lesser than or equal to (`<=`). Applicable to numeric types.
    LesserOrEqual,
    /// Case-insensitive exact match. Applicable to text values.
    IsExactly,
    /// Case-insensitive prefix search (`LIKE 'value%'`). Applicable to text values.
    StartsWith,
    /// Case-insensitive suffix search (`LIKE '%value'`). Applicable to text values.
    EndsWith,
    /// Case-insensitive substring search (`LIKE '%value%'`). Applicable to text values.
    Contains,
}
