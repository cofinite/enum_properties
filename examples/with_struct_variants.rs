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
        } {
            worm_count: usize,
        },
        Orange {
            name: "orange",
            description: "Round and refreshing.",
            weight: 0.13,
        } {
            peel_area: f64,
        },
        Banana {
            name: "banana",
            description: "Elongated and yellow.",
            weight: 0.12,
        } {
            taste: &'static str,
        },
    }
}

fn main() {
    println!(
        "An {} weighs about {} kg.",
        Fruit::Apple { worm_count: 0 }.name,
        Fruit::Apple { worm_count: 0 }.weight
    );
    let _ = Fruit::Apple { worm_count: 0 };
    let _ = Fruit::Orange { peel_area: 12.0 };
    let _ = Fruit::Banana { taste: "Tasty!" };
}
