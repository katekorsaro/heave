#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum E<'a> {
    Bool(bool),
    Real(f64),
    SignedInt(i64),
    Text(&'a str),
    UnsignedInt(i64),
}
