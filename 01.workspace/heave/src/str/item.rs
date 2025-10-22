#[cfg(test)]
use crate::*;

#[cfg(test)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct O {
    pub id: String,
    pub name: String,
    pub price: u64,
    pub discount: f64,
    pub sell_trend: i64,
    pub in_stock: bool,
    pub subclass: Option<String>,
}
#[cfg(test)]
impl EAV for Item {
    fn class() -> &'static str {
        "item"
    }
}
#[cfg(test)]
impl From<Item> for Entity {
    fn from(value: Item) -> Entity {
        let mut entity = Entity::new::<Item>()
            .with_id(&value.id)
            .with_subclass("subitem")
            .with_attribute("name", value.name)
            .with_attribute("price", value.price)
            .with_attribute("discount", value.discount)
            .with_attribute("sell_trend", value.sell_trend)
            .with_attribute("in_stock", value.in_stock);
        if let Some(subclass) = value.subclass {
            entity = entity.with_subclass(&subclass);
        }
        entity
    }
}
#[cfg(test)]
impl From<Entity> for Item {
    fn from(entity: Entity) -> Self {
        Self {
            id: entity.id.clone(),
            name: entity.unwrap("name").expect("name is always present"),
            price: entity.unwrap("price").expect("price is always present"),
            discount: entity
                .unwrap("discount")
                .expect("discount is always present"),
            sell_trend: entity
                .unwrap("sell_trend")
                .expect("sell_trend is always present"),
            in_stock: entity
                .unwrap("in_stock")
                .expect("in_stock is always present"),
            subclass: entity.subclass,
        }
    }
}
