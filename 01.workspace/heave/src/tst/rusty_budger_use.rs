#[cfg(test)]
mod tests {
    use crate::*;
    use category::*;
    use operation::*;
    use relation::*;
    pub mod relation {
        use super::*;
        pub struct OperationToCategory {
            pub id: String,
            pub operation_id: String,
            pub category_id: String,
        }
        impl EAV for OperationToCategory {}
        impl ToEAV for OperationToCategory {
            fn to_eav(self) -> Entity {
                Entity::default()
                    .with_class("operation_has_category")
                    .with_id(&self.id)
                    .with_attribute("operation_id", self.operation_id)
                    .with_attribute("category_id", self.category_id)
            }
        }
        impl FromEAV for OperationToCategory {
            fn from_eav(entity: Entity) -> Self {
                OperationToCategory {
                    id: entity.id.clone(),
                    operation_id: entity.unwrap("operation_id"),
                    category_id: entity.unwrap("category_id"),
                }
            }
        }
    }
    pub mod category {
        use super::*;
        pub struct Category {
            pub id: String,
            pub label: String,
        }
        impl EAV for Category {}
        impl ToEAV for Category {
            fn to_eav(self) -> Entity {
                Entity::default()
                    .with_class("category")
                    .with_id(&self.id)
                    .with_attribute("label", self.label)
            }
        }
        impl FromEAV for Category {
            fn from_eav(entity: Entity) -> Self {
                Category {
                    id: entity.id.clone(),
                    label: entity.unwrap("label"),
                }
            }
        }
    }
    pub mod operation {
        use super::*;
        pub struct Operation {
            pub id: String,
            pub date: u64,
            pub amount: i64,
            pub description: String,
        }
        impl EAV for Operation {}
        impl ToEAV for Operation {
            fn to_eav(self) -> Entity {
                Entity::default()
                    .with_class("operation")
                    .with_id(&self.id)
                    .with_attribute("date", self.date)
                    .with_attribute("amount", self.amount)
                    .with_attribute("description", self.description)
            }
        }
        impl FromEAV for Operation {
            fn from_eav(entity: Entity) -> Self {
                Operation {
                    id: entity.id.clone(),
                    date: entity.unwrap("date"),
                    amount: entity.unwrap("amount"),
                    description: entity.unwrap("description"),
                }
            }
        }
    }
    #[test]
    pub fn check_001() {
        let mut catalog = Catalog::new("");
        let operation_01 = Operation {
            id: short_uuid::short!().to_string(),
            date: 20250929,
            amount: -10000,
            description: "operation 1".to_string(),
        };
        let operation_02 = Operation {
            id: short_uuid::short!().to_string(),
            date: 20250930,
            amount: -20000,
            description: "operation 2".to_string(),
        };
        let category_01 = Category {
            id: short_uuid::short!().to_string(),
            label: "Need".to_string(),
        };
        let relation_01 = OperationToCategory {
            id: short_uuid::short!().to_string(),
            operation_id: operation_01.id.clone(),
            category_id: category_01.id.clone(),
        };
        let operations = vec![operation_01, operation_02];
        let categories = vec![category_01];
        let relations = vec![relation_01];
        catalog.insert_many(operations);
        catalog.insert_many(categories);
        catalog.insert_many(relations);
    }
}
