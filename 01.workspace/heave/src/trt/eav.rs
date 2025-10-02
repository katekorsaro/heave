use crate::*;

/// TODO: INSERT DOCUMENTATION HERE
pub trait T where
    Self: From<Entity>,
    Self: Into<Entity>,
{
    fn class() -> &'static str;
}

// #[cfg(test)]
// mod unit_tests {}
