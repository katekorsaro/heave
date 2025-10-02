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
        impl EAV for OperationToCategory {
            fn class() -> &'static str {
                "operation_to_category"
            }
        }
        impl From<OperationToCategory> for Entity {
            fn from(value: OperationToCategory) -> Entity {
                Entity::default()
                    .with_class::<OperationToCategory>()
                    .with_id(&value.id)
                    .with_attribute("operation_id", value.operation_id)
                    .with_attribute("category_id", value.category_id)
            }
        }
        impl From<Entity> for OperationToCategory {
            fn from(entity: Entity) -> Self {
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
        impl EAV for Category {
            fn class() -> &'static str {
                "category"
            }
        }
        impl From<Category> for Entity {
            fn from(value: Category) -> Entity {
                Entity::default()
                    .with_class::<Category>()
                    .with_id(&value.id)
                    .with_attribute("label", value.label)
            }
        }
        impl From<Entity> for Category {
            fn from(entity: Entity) -> Self {
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
        impl EAV for Operation {
            fn class() -> &'static str {
                "operation"
            }
        }
        impl From<Operation> for Entity {
            fn from(value: Operation) -> Entity {
                Entity::default()
                    .with_class::<Operation>()
                    .with_id(&value.id)
                    .with_attribute("date", value.date)
                    .with_attribute("amount", value.amount)
                    .with_attribute("description", value.description)
            }
        }
        impl From<Entity> for Operation {
            fn from(entity: Entity) -> Self {
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
        let file = "./rustybudger.sqlite3";
        // clean filesystem
        if let Ok(true) = std::fs::exists(file) {
            let _ = std::fs::remove_file(file);
        }
        let mut catalog = Catalog::new(file);
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
        catalog.persist();
    }
}
