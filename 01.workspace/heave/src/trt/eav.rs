use crate::*;

/// TODO: INSERT DOCUMENTATION HERE
pub trait T where
    Self: From<Entity>,
    Self: ToEAV,
{
}

// #[cfg(test)]
// mod unit_tests {}
