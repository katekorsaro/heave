use crate::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct O {
    path: String,
    pub(crate) items: std::collections::HashMap<String, Entity>,
}

impl O {
    pub fn new(path: &str) -> Self {
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
        let mut entity = object.into();
        entity.state = EntityState::New;
        self.items.insert(entity.id.clone(), entity);
    }
    pub fn insert_many(&mut self, objects: Vec<impl EAV>) {
        for object in objects {
            let mut entity = object.into();
            entity.state = EntityState::New;
            self.items.insert(entity.id.clone(), entity);
        }
    }
    pub fn get<T>(&self, id: &str) -> Option<T>
    where
        T: From<Entity>,
    {
        let entity = self.items.get(id);
        entity.map(|e| T::from(e.clone()))
    }
    pub fn get_by_class_and_attribute<T>(
        &self,
        class: &str,
        attribute: &str,
        value: impl Into<Value> + Clone,
    ) -> Option<T>
    where
        T: From<Entity>,
    {
        let mut items = self
            .items
            .values()
            .filter(|item| item.class == class)
            .filter(|item| item.value_of(attribute) == Some(&value.clone().into()))
            .take(1)
            .map(|item| T::from(item.clone()));
        items.next()
    }
    pub fn list_by_class<T>(&self, class: &str) -> impl Iterator<Item = T>
    where
        T: From<Entity>,
    {
        self.items
            .values()
            .filter(move |item| item.class == class)
            .map(|item| T::from(item.clone()))
    }
    pub fn list_by_class_and_attribute<T>(
        &self,
        class: &str,
        attribute: &str,
        value: impl Into<Value> + Clone,
    ) -> impl Iterator<Item = T>
    where
        T: From<Entity>,
    {
        let value: Value = value.into();
        self.items
            .values()
            .filter(move |item| item.class == class)
            .filter(move |item| item.value_of(attribute) == Some(&value))
            .map(|item| T::from(item.clone()))
    }
    pub fn persist(&self) {
        let path = path::Path::new(&self.path);
        sqlite::persist::catalog(path, self);
    }
    pub fn load_by_id(&mut self, id: &str) {
        let path = path::Path::new(&self.path);
        let entity = sqlite::load::by_id(path, id);
        match entity {
            None => (),
            Some(entity) => {
                self.items.insert(entity.id.clone(), entity);
            }
        }
    }
    pub fn load_by_class(&mut self, class: &str) {
        let path = path::Path::new(&self.path);
        let entities = sqlite::load::by_class(path, class);
        for entity in entities {
            self.items.insert(entity.id.clone(), entity);
        }
    }
}
