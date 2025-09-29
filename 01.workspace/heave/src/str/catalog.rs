use crate::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct O {
    path: String,
    items: std::collections::HashMap<String, Entity>,
}

impl O {
    pub fn new(path: &str) -> Self {
        Self {
            path: String::from(path),
            ..O::default()
        }
    }
    pub fn insert(&mut self, object: impl ToEAV) {
        let entity = object.to_eav();
        self.items.insert(entity.id.clone(), entity);
    }
}

// impl std::fmt::Display for O {
// fn fmt(&self, _f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
// todo!();
// }
// }

// #[cfg(test)]
// mod unit_tests {
// use super::*;
// }
