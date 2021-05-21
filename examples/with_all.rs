use enum_properties::enum_properties;

pub struct FruitProperties {
    pub name: &'static str,
    pub description: &'static str,
    pub weight: f32,
}

enum_properties! {
    #[repr(u8)]
    #[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
    pub enum Fruit: FruitProperties {
        Apple {
            name: "apple",
            description: "Keeps the doctor away.",
            weight: 0.1,
        },
        Orange {
            name: "orange",
            description: "Round and refreshing.",
            weight: 0.13,
        } (usize),
        Banana {
            name: "banana",
            description: "Elongated and yellow.",
            weight: 0.12,
        } {
            length: f32,
        },
    }
}

fn main() {
    let fruits = [
        Fruit::Apple,
        Fruit::Orange(10),
        Fruit::Banana { length: 18.0 },
    ];

    for &fruit in &fruits {
        print!("{}s weigh about {} kg, ", fruit.name, fruit.weight);
        match fruit {
            Fruit::Apple => {
                println!("got a nice worm");
            }
            Fruit::Orange(segment_count) => {
                println!("this one is made of {} segments.", segment_count);
            }
            Fruit::Banana { length } => {
                println!("this one is {} cm long.", length);
            }
        }
    }
}
