
use enum_properties::enum_properties;

pub struct FruitProperties {
    pub name:           &'static str,
    pub description:    &'static str,
    pub weight:         f32,
}

enum_properties! {
    #[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
    pub enum Fruit: FruitProperties {
        Apple {
            name: "apple",
            description: "Keeps the doctor away.",
            weight: 0.1,
        } (usize),
        Orange {
            name: "orange",
            description: "Round and refreshing.",
            weight: 0.13,
        } (usize),
        Banana {
            name: "banana",
            description: "Elongated and yellow.",
            weight: 0.12,
        } (f32),
    }
}

fn main() {
    let fruits = [
        Fruit::Apple(0),
        Fruit::Orange(10),
        Fruit::Banana(18.0),
    ];
    
    for &fruit in &fruits {
        println!("Name: {}", fruit.name);
        println!("Weight: {} kg", fruit.weight);
        print!("Description: {}", fruit.description);
        
        match fruit {
            Fruit::Apple(worm_count) => {
                println!(" This one has {} worms.", worm_count);
            }
            Fruit::Orange(segment_count) => {
                println!(" This one is made of {} segments.", segment_count);
            }
            Fruit::Banana(length) => {
                println!(" This one is {} cm long.", length);
            }
        }
        println!("");
    }
}
