use crate::*;

/// TODO: INSERT DOCUMENTATION HERE
pub trait T where
    Self: FromEAV,
    Self: ToEAV,
{
}

// #[cfg(test)]
// mod unit_tests {}
