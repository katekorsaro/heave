#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Hash)]
pub enum E {
    Bool(bool),
    SignedInt(i64),
    UnsignedInt(i64),
}
