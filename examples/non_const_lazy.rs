
use enum_properties::enum_properties_lazy;

/// Custom struct, none-copy, none-clone, not even const-init
pub struct FruitProperties {
    pub name:           String,
    pub features:       Vec<f32>,
}

enum_properties_lazy! {
    #[repr(u8)]
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
    pub enum Fruit: FruitProperties {
        Apple {
            name: "apple".to_string(),
            features: Vec::new(),
        },
        Orange {
            name: {
                // Allows arbitrarily complex runtime initializers, thanks
                // to the magic of lazy_static
                let mut builder = String::new();
                builder.push_str("orange");
                builder
            },
            features: {
                let mut builder = Vec::new();
                builder.push(0.13);
                builder.push(0.42);
                builder
            },
        },
        Banana {
            name: "banana".into(),
            features: vec![0.12, 0.34, 0.32],
        },
    }
}

fn main() {
    println!("An {} weighs, {:?}.",
        Fruit::Apple.name,
        Fruit::Apple.features
    );
}
