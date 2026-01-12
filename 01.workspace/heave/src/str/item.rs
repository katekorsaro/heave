#[cfg(test)]
use crate::*;

#[cfg(test)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct O {
    pub id: String,
    pub first_seen: u32,
    pub name: String,
    pub price: u32,
    pub discount: f64,
    pub sell_trend: i64,
    pub in_stock: bool,
    pub subclass: Option<String>,
    pub category: Option<String>,
    pub tag: String,
    pub supplier_code: u32,
    pub supplier_rank: i32,
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
            .with_ref_date(value.first_seen)
            .with_attribute("name", value.name)
            .with_attribute("price", value.price)
            .with_attribute("discount", value.discount)
            .with_attribute("sell_trend", value.sell_trend)
            .with_attribute("in_stock", value.in_stock)
            .with_opt_attribute("category", value.category)
            .with_attribute("tag", value.tag)
            .with_attribute("supplier_code", value.supplier_code)
            .with_attribute("supplier_rank", value.supplier_rank);
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
            id: entity.id(),
            first_seen: entity.ref_date.expect("ref date is always present"),
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
            subclass: entity.subclass(),
            category: entity
                .unwrap_opt("category")
                .expect("category is always optinally present"),
            tag: entity
                .unwrap_or("tag", "-".to_string())
                .expect("tag is always present"),
            supplier_code: entity
                .unwrap("supplier_code")
                .expect("supplier code is always present"),
            supplier_rank: entity
                .unwrap("supplier_rank")
                .expect("supplier rank is always present"),
        }
    }
}
