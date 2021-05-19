use enum_properties::enum_properties;

pub struct FruitProperties {
    pub name: &'static str,
    pub description: &'static str,
    pub weight: f32,
}

enum_properties! {
    #[repr(u8)]
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
    pub enum Fruit: FruitProperties {
        Apple {
            name: "apple",
            description: "Keeps the doctor away.",
            weight: 0.1,
        } = 0,
        Orange {
            name: "orange",
            description: "Round and refreshing.",
            weight: 0.13,
        } = 2,
        Banana {
            name: "banana",
            description: "Elongated and yellow.",
            weight: 0.12,
        } = 3,
    }
}

fn main() {
    println!(
        "An {} weighs about {} kg.",
        Fruit::Apple.name,
        Fruit::Apple.weight
    );
    assert_eq!(Fruit::Apple as u8, 0);
    assert_eq!(Fruit::Orange as u8, 2);
    assert_eq!(Fruit::Banana as u8, 3);
}
