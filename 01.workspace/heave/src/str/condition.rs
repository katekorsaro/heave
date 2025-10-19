#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Hash)]
pub enum E<'a> {
    Bool(bool),
    SignedInt(i64),
    UnsignedInt(i64),
    Text(&'a str),
}
