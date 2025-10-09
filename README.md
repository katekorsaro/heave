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
use heave::*;

// Define a struct named `Product` to represent a product.
struct Product {
    // `id` is a public field of type `String` to uniquely identify the product.
    pub id: String,
    // `name` is a public field of type `String` for the product's name.
    pub name: String,
    // `model` is a public optional field of type `String` for the product's model.
    pub model: Option<String>,
    // `price` is a public field of type `i64` for the product's price.
    pub price: i64,
}

// Implement the `EAV` trait for the `Product` struct.
impl EAV for Product {
    // `class` is a function that returns the class name of the entity.
    fn class() -> &'static str {
        "product"
    }
}

// Implement the `From<Product>` trait for the `Entity` struct.
impl From<Product> for Entity {
    // `from` is a function that converts a `Product` into an `Entity`.
    fn from(value: Product) -> Entity {
        // Create a new `Entity` for the `Product` class.
        Entity::new::<Product>()
            // Set the entity's ID from the product's ID.
            .with_id(&value.id)
            // Add the "name" attribute with the product's name.
            .with_attribute("name", value.name)
            // Add the optional "model" attribute with the product's model.
            .with_opt_attribute("model", value.model)
            // Add the "price" attribute with the product's price.
            .with_attribute("price", value.price)
    }
}

// Implement the `From<Entity>` trait for the `Product` struct.
impl From<Entity> for Product {
    // `from` is a function that converts an `Entity` into a `Product`.
    fn from(value: Entity) -> Self {
        // Create a new `Product` from the entity's attributes.
        Self {
            // Set the product's ID from the entity's ID.
            id: value.id.clone(),
            // Unwrap the "name" attribute to get the product's name.
            name: value.unwrap("name"),
            // Unwrap the optional "model" attribute to get the product's model.
            model: value.unwrap_opt("model"),
            // Unwrap the "price" attribute to get the product's price.
            price: value.unwrap("price"),
        }
    }
}

fn main() {
    // Define the path for the SQLite database file.
    let db_path = "./simple_product.sqlite3";
    // Create a new `Catalog` instance with the specified database path.
    let mut catalog = Catalog::new(db_path);
    // Initialize the catalog, which sets up the database.
    catalog.init().unwrap();
    // Create a new `Product` instance representing a laptop.
    let new_laptop = Product {
        id: "LT001".to_string(),
        name: "SuperPenguin".to_string(),
        model: Some("Mark III.2".to_string()),
        price: 125000,
    };
    // Insert the new laptop into the catalog. Note that at this time the product is in memory.
    catalog.insert(new_laptop);
    // Persist the changes in the catalog to the database.
    catalog.persist().unwrap();
    // Remove the SQLite database file.
    std::fs::remove_file(db_path).unwrap();
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
