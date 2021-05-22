
use enum_properties::enum_properties;

pub struct FruitProperties {
    pub name:           &'static str,
    pub description:    &'static str,
    pub weight:         f32,
}

pub const DEFAULT_FRUIT_PROPERTIES: FruitProperties = FruitProperties {
    name: "",
    description: "Round and refreshing.",
    weight: 0.1,
};

enum_properties! {
    // Just like a normal `enum`, an enum defined within an invocation of 
    // `enum_properties` can have attributes:
    #[doc = "Fruit"]
    #[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
    pub enum Fruit: FruitProperties {
        // Variants can be unit-like, followed only by their static properties:
        #[doc = "Fruit::Apple"] // Variants can also have attributes
        Apple {
            name: "apple",
            description: "Keeps the doctor away."
        },
        // Variants can also be tuple-like, containing some non-static data.
        // The tuple is defined after the static properties.
        #[doc = "Fruit::Orange"]
        Orange {
            name: "orange",
            weight: 0.13,
        } (
            // Members of a tuple-like variant can also have attributes:
            #[doc = "Fruit::Orange(segment_count)"]
            usize,
        ),
        // Variants can also be struct-like, containing some non-static data.
        // The struct is defined after the static properties.
        #[doc = "Fruit::Banana"]
        Banana {
            name: "banana",
            description: "Elongated and yellow.",
            weight: 0.12,
        } {
            // Members of a struct-like variant can also have attributes:
            #[doc = "Fruit::Banana { length }"]
            length: f32,
        },
        // This syntax specifies defaults, such that each variant does not have
        // to define all of its static properties. Properties left undefined 
        // are drawn from this value instead.
        .. DEFAULT_FRUIT_PROPERTIES
    }
}

fn main() {
    let fruits = [
        Fruit::Apple,
        Fruit::Orange(10),
        Fruit::Banana { length: 18.0 },
    ];
    
    for &fruit in &fruits {
        println!("Name: {}", fruit.name);
        println!("Weight: {} kg", fruit.weight);
        print!("Description: {}", fruit.description);
        
        match fruit {
            Fruit::Apple => {
                println!(" This one is like all the others.");
            }
            Fruit::Orange(segment_count) => {
                println!(" This one is made of {} segments.", segment_count);
            }
            Fruit::Banana { length } => {
                println!(" This one is {} cm long.", length);
            }
        }
    }
    println!("");
}
