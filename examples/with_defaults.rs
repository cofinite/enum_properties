
use enum_properties::enum_properties;

pub struct FruitProperties {
    pub name: &'static str,
    pub description: &'static str,
    pub weight: f32,
}

pub const DEFAULT_FRUIT_PROPERTIES: FruitProperties = FruitProperties {
    name: "",
    description: "Round and refreshing.",
    weight: 0.1,
};

enum_properties! {
    #[repr(u8)]
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
    pub enum Fruit: FruitProperties {
        Apple {
            name: "apple",
            description: "Keeps the doctor away.",
        },
        Orange {
            name: "orange",
            weight: 0.13,
        },
        Banana {
            name: "banana",
            description: "Elongated and yellow.",
            weight: 0.12,
        },
        .. DEFAULT_FRUIT_PROPERTIES
    }
}

fn main() {
    println!("An {} weighs about {} kg.",
        Fruit::Apple.name,
        Fruit::Apple.weight
    );
}
