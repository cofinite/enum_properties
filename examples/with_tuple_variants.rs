use enum_properties::enum_properties;

pub struct FruitProperties {
    pub name: &'static str,
    pub description: &'static str,
    pub weight: f32,
}

enum_properties! {
    #[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
    pub enum Fruit: FruitProperties {
        Apple {
            name: "apple",
            description: "Keeps the doctor away.",
            weight: 0.1,
        } (u8),
        Orange {
            name: "orange",
            description: "Round and refreshing.",
            weight: 0.13,
        } (f32),
        Banana {
            name: "banana",
            description: "Elongated and yellow.",
            weight: 0.12,
        } (&'static str),
    }
}

fn main() {
    println!(
        "An {} weighs about {} kg.",
        Fruit::Apple(1).name,
        Fruit::Apple(1).weight
    );
    let _ = Fruit::Apple(1);
    let _ = Fruit::Orange(12.0);
    let _ = Fruit::Banana("Tasty!");
}
