use heave::*;
use std::path::Path;

struct Laptop {
    pub id: String,
    pub model: String,
    pub price: u64,
}
struct Display {
    pub id: String,
    pub model: String,
    pub resolution: f64,
    pub price: u64,
}
struct Mouse {
    pub id: String,
    pub model: String,
    pub wireless: bool,
    pub price: u64,
}
enum Product {
    None,
    Laptop(Laptop),
    Display(Display),
    Mouse(Mouse),
}
impl EAV for Product {
    fn class() -> &'static str {
        "product"
    }
}
impl From<Entity> for Product {
    fn from(value: Entity) -> Self {
        if let Some(ref subclass) = value.subclass {
            match subclass.as_ref() {
                "laptop" => Product::Laptop(Laptop {
                    id: value.id.clone(),
                    model: value.unwrap("model").expect("model is mandatory"),
                    price: value.unwrap("price").expect("price is mandatory"),
                }),
                "display" => Product::Display(Display {
                    id: value.id.clone(),
                    model: value.unwrap("model").expect("model is mandatory"),
                    price: value.unwrap("price").expect("price is mandatory"),
                    resolution: value.unwrap("resolution").expect("resolution is mandatory"),
                }),
                "mouse" => Product::Mouse(Mouse {
                    id: value.id.clone(),
                    model: value.unwrap("model").expect("model is mandatory"),
                    price: value.unwrap("price").expect("price is mandatory"),
                    wireless: value.unwrap("wireless").expect("wireless is mandatory"),
                }),
                _ => unreachable!(),
            }
        } else {
            Product::None
        }
    }
}
impl From<Product> for Entity {
    fn from(value: Product) -> Self {
        match value {
            Product::Laptop(value) => Entity::new::<Product>()
                .with_id(&value.id)
                .with_subclass("laptop")
                .with_attribute("model", value.model)
                .with_attribute("price", value.price),
            Product::Display(value) => Entity::new::<Product>()
                .with_id(&value.id)
                .with_subclass("display")
                .with_attribute("model", value.model)
                .with_attribute("resolution", value.resolution)
                .with_attribute("price", value.price),
            Product::Mouse(value) => Entity::new::<Product>()
                .with_id(&value.id)
                .with_subclass("mouse")
                .with_attribute("model", value.model)
                .with_attribute("wireless", value.wireless)
                .with_attribute("price", value.price),
            _ => unreachable!(),
        }
    }
}
fn main() -> Result<(), FailedTo> {
    let db_path = "working_with_many_types.db";

    // Clean up previous runs if file exists
    if Path::new(db_path).exists() {
        std::fs::remove_file(db_path).unwrap();
    }

    // 1. Initialize and Persist Data
    println!("== 1. Storing different product types ==");
    let mut catalog = Catalog::new(db_path);
    catalog.init()?;

    let products_to_add = vec![
        Product::Laptop(Laptop {
            id: "laptop_01".to_string(),
            model: "Titan".to_string(),
            price: 1500,
        }),
        Product::Display(Display {
            id: "display_01".to_string(),
            model: "CrystalClear".to_string(),
            resolution: 4.0, // 4K
            price: 600,
        }),
        Product::Mouse(Mouse {
            id: "mouse_01".to_string(),
            model: "SwiftClick".to_string(),
            wireless: true,
            price: 80,
        }),
        Product::Laptop(Laptop {
            id: "laptop_02".to_string(),
            model: "Nomad".to_string(),
            price: 950,
        }),
    ];

    catalog.insert_many(products_to_add)?;
    catalog.persist()?;
    println!("✅ 4 products saved to the database.\n");

    // 2. Load data using filters
    println!("== 2. Loading products using class and subclass filters ==");

    // Load only laptops
    let mut laptop_catalog = Catalog::new(db_path);
    let laptop_filter = Filter::new()
        .with_class(Product::class())
        .with_subclass("laptop");

    laptop_catalog.load_by_filter(&laptop_filter)?;

    let laptops: Vec<Product> = laptop_catalog.list_by_class().map(|p| p.unwrap()).collect();

    println!("✅ Loaded {} laptop(s) using filter.", laptops.len());
    assert_eq!(laptops.len(), 2);
    for p in laptops {
        if let Product::Laptop(laptop) = p {
            println!(
                "  - Laptop: {} ({}) - ${}",
                laptop.id, laptop.model, laptop.price
            );
        }
    }
    println!();

    println!("== 3. Loading products using attribute filters ==");
    // Load expensive products (price > 1000)
    let mut expensive_catalog = Catalog::new(db_path);
    let expensive_filter = Filter::new()
        .with_class(Product::class())
        .with_unsigned_int("price", Comparison::Greater, 1000);

    expensive_catalog.load_by_filter(&expensive_filter)?;

    let expensive_products: Vec<Product> = expensive_catalog
        .list_by_class()
        .map(|p| p.unwrap())
        .collect();

    println!(
        "✅ Loaded {} product(s) with price > $1000.",
        expensive_products.len()
    );
    assert_eq!(expensive_products.len(), 1);

    for p in expensive_products {
        match p {
            Product::Laptop(laptop) => {
                println!(
                    "  - Found Laptop: {} ({}) - ${}",
                    laptop.id, laptop.model, laptop.price
                );
                assert_eq!(laptop.id, "laptop_01");
            }
            _ => panic!("Expected a laptop!"),
        }
    }
    println!();

    println!("== 4. Loading all product types ==");
    let mut all_products_catalog = Catalog::new(db_path);
    let all_products_filter = Filter::new().with_class(Product::class());
    all_products_catalog.load_by_filter(&all_products_filter)?;

    let all_products: Vec<Product> = all_products_catalog
        .list_by_class()
        .map(|p| p.unwrap())
        .collect();

    println!("✅ Loaded {} total products.", all_products.len());
    assert_eq!(all_products.len(), 4);

    for p in all_products {
        match p {
            Product::Laptop(laptop) => {
                println!(
                    "  - Found Laptop: {} ({}) - ${}",
                    laptop.id, laptop.model, laptop.price
                );
            }
            Product::Display(display) => {
                println!(
                    "  - Found Display: {} ({}) - ${}",
                    display.id, display.model, display.price
                );
            }
            Product::Mouse(mouse) => {
                println!(
                    "  - Found Mouse: {} ({}) - ${}",
                    mouse.id, mouse.model, mouse.price
                );
            }
            Product::None => panic!("Product::None should not be loaded"),
        }
    }
    println!();

    // Clean up the created database file
    std::fs::remove_file(db_path).unwrap();

    Ok(())
}
