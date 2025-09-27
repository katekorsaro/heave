use crate::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct O {
    pub class: String,
    pub attributes: std::collections::HashMap<String, Attribute>,
}
