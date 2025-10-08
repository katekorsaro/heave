# heave 󱅝

A Rust EAV data model implementation.

Heave is a Rust library that provides a flexible and extensible implementation of the Entity-Attribute-Value (EAV) data model. It allows you to manage data with a dynamic schema, making it suitable for scenarios where attributes of entities are not known at compile time. The library includes support for persisting the EAV data to a SQLite database.

## Typed Entities

Typed entities may be used leveraging the `EAV` trait to map to and from schemaless entities.

## EAV

Learn more on EAV data model [here](https://en.wikipedia.org/wiki/Entity%E2%80%93attribute%E2%80%93value_model).

## Workspace Structure

- **01.workspace**: this folder contains all workspace crates

## heave crate structure

- **src/fun**: contains all functions
- **src/imp**: contains all implementation (impl blocks)
- **src/mcr**: contains all macros
- **src/str**: contains all structures
- **src/trt**: contains all traits
- **src/tst**: contains all tests
- **lib.rs**: renames and re-exports code compnents

## Example

Here is an example of how to define a typed entity, persist it to a SQLite database, and load it back.

```rust
use heave::{Catalog, EAV, Entity};
use std::fs;

// Define a typed entity.
#[derive(Clone, Debug, PartialEq)]
struct Product {
    id: String,
    name: String,
    model: Option<String>,
    price: u64,
    in_stock: bool,
}

// Implement the EAV trait to define the class of the entity.
impl EAV for Product {
    fn class() -> &'static str {
        "product"
    }
}

// Implement From<Entity> to map a schemaless entity to a typed one.
impl From<Entity> for Product {
    fn from(entity: Entity) -> Self {
        Product {
            id: entity.id.clone(),
            name: entity.unwrap("name"),
            model: entity.unwrap_opt("model"),
            price: entity.unwrap_or("price", 0),
            in_stock: entity.unwrap("in_stock"),
        }
    }
}

// Implement From<Product> to map a typed entity to a schemaless one.
impl From<Product> for Entity {
    fn from(value: Product) -> Self {
        let mut entity = Entity::new::<Product>()
            .with_id(&value.id)
            .with_attribute("name", value.name)
            .with_attribute("price", value.price)
            .with_attribute("in_stock", value.in_stock);
        if let Some(model) = value.model {
            entity.set("model", model);
        }
        entity
    }
}

fn main() {
    let db_path = "example.db";

    // Initialize a new catalog and database.
    let mut catalog = Catalog::new(db_path);
    catalog.init();

    // Create a new typed object.
    let product = Product {
        id: "prod-123".to_string(),
        name: String::from("PenguinX Laptop"),
        model: None,
        price: 135000,
        in_stock: true,
    };
    let product_id = product.id.clone();

    // Insert the object and persist it to the database.
    catalog.insert(product);
    catalog.persist();

    // Create a new catalog to simulate a different application instance.
    let mut new_catalog = Catalog::new(db_path);

    // Load the entity from the database.
    new_catalog.load_by_id(&product_id);

    // Retrieve the typed object from the catalog.
    let read_product = new_catalog.get::<Product>(&product_id).unwrap();

    println!("Successfully persisted and loaded: {:?}", read_product);

    // Clean up the database file.
    fs::remove_file(db_path).unwrap();
}
```

## Contributing

Contributions are welcome! If you'd like to contribute to heave, please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature (feature/my_awesome_feature) or bug fix (fix/my_awesome_fix).
3. Make your changes and commit them with a clear message.
4. Push your branch to your fork.
5. Open a pull request to the main repository.

Please ensure that your code adheres to the existing style and that all tests pass before submitting a pull request.

## Credits

- **Project Icon:** by [SmashIcons](https://www.flaticon.com/authors/smashicons)
