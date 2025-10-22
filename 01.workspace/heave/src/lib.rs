//! A lightweight and intuitive Entity-Attribute-Value (EAV) database library for Rust.
//!
//! This library provides a simple way to persist and query semi-structured data using an EAV model on top of a SQLite database. It is designed to be easy to use, with a focus on simplicity and developer experience.
//!
//! ## What is the Entity-Attribute-Value (EAV) Model?
//!
//! EAV is a data model that stores data as three-part tuples:
//!
//! - **Entity:** The object being described (e.g., a specific product, a user).
//! - **Attribute:** A property of the entity (e.g., "color", "price", "email").
//! - **Value:** The value of the attribute for that entity (e.g., "blue", 19.99, "user@example.com").
//!
//! This model is highly flexible, allowing you to add new attributes to entities without changing the underlying database schema. It's particularly useful for scenarios with sparse data (where many attributes are optional) or when the data structure is expected to evolve over time.
//!
//! ## How `heave` Works
//!
//! The library revolves around a few key components:
//!
//! - **`EAV` Trait:** Any struct you want to store in the database must implement this trait. It provides the necessary conversions between your custom type and the generic `Entity` representation used by the library.
//! - **`Catalog`:** The main entry point for interacting with the database. It manages a connection to a SQLite file and holds an in-memory cache of entities you are working with.
//! - **`Entity`:** A generic container for an object's data, holding its ID, class (type), and a map of attributes to `Value`s.
//! - **`Value`:** An enum that can represent different data types (e.g., `String`, `i64`, `f64`, `bool`).
//! - **`Filter`:** A builder for creating complex queries to load data from the database based on specific conditions.
//!
//! ## Usage Example
//!
//! Here's a complete, runnable example of how to define a type, persist it, and query it back.
//!
//! ### 1. Define Your Struct and Implement `EAV`
//!
//! First, define the struct you want to save. Then, implement the `EAV` trait
//! and the necessary `From` and `TryFrom` conversions.
//!
//! ```rust,no_run
//! use heave::*;
//! use std::convert::{From, TryFrom};
//! use std::result::Result;
//!
//! // Define a simple struct representing a product.
//! #[derive(Debug, Default, PartialEq, Clone)]
//! struct Product {
//!     pub id: String,
//!     pub name: String,
//!     pub price: u64,
//!     pub in_stock: bool,
//! }
//!
//! // Implement the EAV trait to define the "class" of this entity.
//! impl EAV for Product {
//!     fn class() -> &'static str {
//!         "product"
//!     }
//! }
//!
//! // Convert our Product into a generic Entity.
//! impl From<Product> for Entity {
//!     fn from(p: Product) -> Self {
//!         Entity::new::<Product>()
//!             .with_id(&p.id)
//!             .with_attribute("name", p.name)
//!             .with_attribute("price", p.price)
//!             .with_attribute("in_stock", p.in_stock)
//!     }
//! }
//!
//! // Convert a generic Entity back into our Product.
//! impl TryFrom<Entity> for Product {
//!     type Error = FailedTo;
//!
//!     fn try_from(entity: Entity) -> Result<Self, Self::Error> {
//!         Ok(Self {
//!             id: entity.id.clone(),
//!             name: entity.unwrap("name").map_err(|_| FailedTo::ConvertEntity)?,
//!             price: entity.unwrap("price").map_err(|_| FailedTo::ConvertEntity)?,
//!             in_stock: entity.unwrap("in_stock").map_err(|_| FailedTo::ConvertEntity)?,
//!         })
//!     }
//! }
//!
//! fn main() -> Result<(), FailedTo> {
//!     let db_path = "my_products.db";
//!
//!     // Clean up previous runs if file exists
//!     if std::path::Path::new(db_path).exists() {
//!         std::fs::remove_file(db_path).unwrap();
//!     }
//!
//!     // == 1. Initialize and Persist Data ==
//!     let mut catalog = Catalog::new(db_path);
//!     catalog.init()?;
//!
//!     let products_to_add = vec![
//!         Product {
//!             id: "p1".to_string(),
//!             name: "Laptop".to_string(),
//!             price: 1200,
//!             in_stock: true,
//!         },
//!         Product {
//!             id: "p2".to_string(),
//!             name: "Mouse".to_string(),
//!             price: 25,
//!             in_stock: true,
//!         },
//!         Product {
//!             id: "p3".to_string(),
//!             name: "Keyboard".to_string(),
//!             price: 75,
//!             in_stock: false,
//!         },
//!     ];
//!
//!     catalog.insert_many(products_to_add)?;
//!     catalog.persist()?;
//!     println!("✅ Products saved to the database.");
//!
//!     // == 2. Load and Query Data ==
//!     let mut query_catalog = Catalog::new(db_path);
//!
//!     // Load a single product by its ID.
//!     query_catalog.load_by_id("p1")?;
//!     let laptop: Product = query_catalog.get("p1")?.unwrap();
//!     println!("✅ Loaded by ID: {:?}", laptop);
//!     assert_eq!(laptop.name, "Laptop");
//!
//!     // Load products matching a filter (in stock, price < 100)
//!     let filter = Filter::new()
//!         .with_bool("in_stock", true)
//!         .with_unsigned_int("price", Comparison::Lesser, 100);
//!
//!     let mut filtered_catalog = Catalog::new(db_path);
//!     filtered_catalog.load_by_filter(&filter)?;
//!     let cheap_products: Vec<Product> = filtered_catalog.list_by_class().map(|p| p.unwrap()).collect();
//!     println!("✅ Found {} cheap, in-stock product(s).", cheap_products.len());
//!     assert_eq!(cheap_products.len(), 1);
//!     assert_eq!(cheap_products[0].id, "p2");
//!
//!     // == 3. Update and Delete Data ==
//!     let mut update_catalog = Catalog::new(db_path);
//!     update_catalog.load_by_class::<Product>()?;
//!
//!     // Update the price of the laptop
//!     let mut laptop_to_update: Product = update_catalog.get("p1")?.unwrap();
//!     laptop_to_update.price = 1150;
//!     update_catalog.upsert(laptop_to_update)?;
//!
//!     // Delete the keyboard
//!     update_catalog.delete("p3");
//!
//!     // Persist all changes
//!     update_catalog.persist()?;
//!     println!("✅ Laptop price updated and keyboard deleted.");
//!
//!     // == 4. Verify Final State ==
//!     let mut final_catalog = Catalog::new(db_path);
//!     final_catalog.load_by_class::<Product>()?;
//!
//!     let final_count = final_catalog.list_by_class::<Product>().count();
//!     println!("✅ Final product count: {}", final_count);
//!     assert_eq!(final_count, 2);
//!
//!     let updated_laptop: Product = final_catalog.get("p1")?.unwrap();
//!     println!("✅ Verified updated laptop price: {}", updated_laptop.price);
//!     assert_eq!(updated_laptop.price, 1150);
//!
//!     let deleted_keyboard: Option<Product> = final_catalog.get("p3")?;
//!     println!("✅ Verified keyboard is deleted.");
//!     assert!(deleted_keyboard.is_none());
//!
//!     // Clean up the created database file
//!     std::fs::remove_file(db_path).unwrap();
//!
//!     Ok(())
//! }
//! ```
use std::*;

mod fun;
mod imp;
mod mcr;
mod str;
mod trt;
mod tst;

pub(crate) use crate::str::attribute::O as Attribute;
pub use crate::str::catalog::O as Catalog;
pub use crate::str::comparison::E as Comparison;
pub use crate::str::condition::E as Condition;
pub use crate::str::entity::O as Entity;
pub use crate::str::entity_state::EntityState;
pub use crate::str::failed_to::FailedTo;
pub use crate::str::filter::O as Filter;
pub(crate) use crate::str::value::Value;
pub use crate::trt::eav::T as EAV;

#[cfg(test)]
pub(crate) use crate::str::item::O as Item;

mod sqlite {
    pub use crate::str::sqlite_failed_to::FailedTo;
    pub mod build {
        pub use crate::fun::sqlite_build_params::run as params;
        pub use crate::fun::sqlite_build_statement::run as statement;
    }
    pub mod init {
        pub use crate::fun::sqlite_init_db::run as db;
    }
    pub mod load {
        pub use crate::fun::sqlite_load_attributes::run as attributes;
        pub use crate::fun::sqlite_load_by_class::run as by_class;
        pub use crate::fun::sqlite_load_by_filter::run as by_filter;
        pub use crate::fun::sqlite_load_by_id::run as by_id;
    }
    pub mod map {
        pub use crate::fun::sqlite_map_row_to_attribute::run as row_to_attribute;
        pub use crate::fun::sqlite_map_row_to_entity::run as row_to_entity;
    }
    pub mod persist {
        pub use crate::fun::sqlite_persist_catalog::run as catalog;
    }
}
