use crate::*;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum E {
    Bool(bool),
    Real(f64),
    SignedInt(i64),
    Text(String),
    UnsignedInt(u64),
}
