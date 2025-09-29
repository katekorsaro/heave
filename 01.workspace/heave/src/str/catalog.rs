use crate::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct O {
    path: String,
    pub(crate) items: std::collections::HashMap<String, Entity>,
}

impl O {
    pub fn new(path: &str) -> Self {
        sqlite::init::db(path::Path::new(path));
        Self {
            path: String::from(path),
            ..O::default()
        }
    }
    pub fn init(&self) {
        let path = path::Path::new(&self.path);
        sqlite::init::db(path);
    }
    pub fn insert(&mut self, object: impl EAV) {
        let entity = object.to_eav();
        self.items.insert(entity.id.clone(), entity);
    }
    pub fn insert_many(&mut self, objects: Vec<impl EAV>) {
        for object in objects {
            let entity = object.to_eav();
            self.items.insert(entity.id.clone(), entity);
        }
    }
    pub fn get<T>(&self, id: &str) -> Option<T>
    where
        T: FromEAV,
    {
        let entity = self.items.get(id);
        entity.map(|e| T::from_eav(e.clone()))
    }
    pub fn persist(&self) {
        let path = path::Path::new(&self.path);
        sqlite::persist::catalog(path, self);
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
