use crate::*;

/// TODO: INSERT DOCUMENTATION HERE
pub trait T where
    Self: From<Entity>,
    Self: Into<Entity>,
{
}

// #[cfg(test)]
// mod unit_tests {}
