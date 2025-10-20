use heave::*;

// Define a struct named `Component` to represent an electronic component.
#[derive(Debug, Clone, PartialEq)]
struct Component {
    pub id: String,
    pub part_number: String,
    pub kind: String,
    pub value: u64,
    pub package: String,
    pub in_stock: bool,
}
// Implement the `EAV` trait for the `Component` struct.
impl EAV for Component {
    // `class` is a function that returns the class name of the entity.
    fn class() -> &'static str {
        "component"
    }
}
// Implement the `From<Component>` trait for the `Entity` struct.
impl From<Component> for Entity {
    // `from` is a function that converts a `Component` into an `Entity`.
    fn from(value: Component) -> Entity {
        Entity::new::<Component>()
            .with_id(&value.id)
            .with_attribute("part_number", value.part_number)
            .with_attribute("kind", value.kind)
            .with_attribute("value", value.value)
            .with_attribute("package", value.package)
            .with_attribute("in_stock", value.in_stock)
    }
}
// Implement the `From<Entity>` trait for the `Component` struct.
impl From<Entity> for Component {
    // `from` is a function that converts an `Entity` into a `Component`.
    fn from(value: Entity) -> Self {
        Self {
            id: value.id.clone(),
            part_number: value
                .unwrap("part_number")
                .expect("part_number is always present"),
            kind: value.unwrap("kind").expect("kind is always present"),
            value: value.unwrap("value").expect("value is always present"),
            package: value.unwrap("package").expect("package is always present"),
            in_stock: value
                .unwrap("in_stock")
                .expect("in_stock is always present"),
        }
    }
}

fn main() {
    // Define the path for the SQLite database file.
    let db_path = "./using_filters.sqlite3";
    // Create a new `Catalog` instance with the specified database path.
    let mut catalog = Catalog::new(db_path);
    // Initialize the catalog, which sets up the database.
    catalog.init().unwrap();
    // Create some component instances.
    let components_to_add = vec![
        // This one should be found
        Component {
            id: "R1".to_string(),
            part_number: "R-10K-0805".to_string(),
            kind: "resistor".to_string(),
            value: 10000,
            package: "smd-0805".to_string(),
            in_stock: true,
        },
        // This one should be found
        Component {
            id: "R2".to_string(),
            part_number: "R-4K7-0805".to_string(),
            kind: "resistor".to_string(),
            value: 4700,
            package: "smd-0805".to_string(),
            in_stock: true,
        },
        // This one should NOT be found (wrong kind)
        Component {
            id: "C1".to_string(),
            part_number: "C-100n-0603".to_string(),
            kind: "capacitor".to_string(),
            value: 100,
            package: "smd-0603".to_string(),
            in_stock: true,
        },
        // This one should NOT be found (value too low)
        Component {
            id: "R3".to_string(),
            part_number: "R-100-0805".to_string(),
            kind: "resistor".to_string(),
            value: 100,
            package: "smd-0805".to_string(),
            in_stock: true,
        },
        // This one should NOT be found (not in stock)
        Component {
            id: "R4".to_string(),
            part_number: "R-22K-TH".to_string(),
            kind: "resistor".to_string(),
            value: 22000,
            package: "through-hole".to_string(),
            in_stock: false,
        },
        // This one should NOT be found (wrong kind, even if other fields match)
        Component {
            id: "L1".to_string(),
            part_number: "L-10mH-TH".to_string(),
            kind: "inductor".to_string(),
            value: 10000,
            package: "through-hole".to_string(),
            in_stock: true,
        },
    ];
    // Insert the components into the catalog.
    catalog.insert_many(components_to_add.clone()).unwrap();
    // Persist the changes to the database.
    catalog.persist().unwrap();
    // Create a new catalog to ensure we are loading from the database.
    let mut new_catalog = Catalog::new(db_path);
    // Create a composite filter.
    // We are looking for resistors with a value greater than 1000 that are in stock.
    let filter = Filter::new()
        .with_text("kind", Comparison::IsExactly, "resistor")
        .with_unsigned_int("value", Comparison::Greater, 1000)
        .with_bool("in_stock", true);
    // Load entities from the database using the filter.
    new_catalog.load_by_filter(&filter).unwrap();
    // Get the list of loaded components.
    let loaded_components: Vec<Component> = new_catalog
        .list_by_class::<Component>()
        .map(|c| c.unwrap())
        .collect();
    // Print the loaded components
    println!(
        "Found {} components matching the filter:",
        loaded_components.len()
    );
    for component in &loaded_components {
        println!(
            "- ID: {}, Part Number: {}, Kind: {}, Value: {}, Package: {}, In Stock: {}",
            component.id,
            component.part_number,
            component.kind,
            component.value,
            component.package,
            component.in_stock
        );
    }
    // Verify that we have loaded the correct number of components.
    assert_eq!(loaded_components.len(), 2);
    // Verify that the correct components were loaded.
    let ids: Vec<String> = loaded_components.iter().map(|c| c.id.clone()).collect();
    assert!(ids.contains(&"R1".to_string()));
    assert!(ids.contains(&"R2".to_string()));
    // Clean up the database file.
    std::fs::remove_file(db_path).unwrap();
}
