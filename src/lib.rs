#![no_std]
#![warn(missing_docs)]

//! A macro with two main purposes:
//! - attaching static properties to `enum` variants
//! - reducing the size of pointers to static records
//! 
//! The advantage in both cases is that the `enum` itself contains none of the 
//! static data, and can be as small as a byte.
//!
//! # Example
//!
//! (More complex enums are also supported. See [`enum_properties`#examples] 
//! for details.)
//!
//! ```rust
//! use enum_properties::enum_properties;
//! 
//! struct SolidProperties {
//!     verts: i32,
//!     edges: i32,
//!     faces: i32,
//! }
//! 
//! enum_properties! {
//!     #[derive(Clone, Copy, Debug)]
//!     enum PlatonicSolid: SolidProperties {
//!         Tetrahedron {
//!             verts: 4,
//!             edges: 6,
//!             faces: 4,
//!         },
//!         Cube {
//!             verts: 8,
//!             edges: 12,
//!             faces: 6,
//!         },
//!         Octahedron {
//!             verts: 6,
//!             edges: 12,
//!             faces: 8,
//!         },
//!         Dodecahedron {
//!             verts: 20,
//!             edges: 30,
//!             faces: 12,
//!         },
//!         Icosahedron {
//!             verts: 12,
//!             edges: 30,
//!             faces: 20,
//!         },
//!     }
//! }
//! 
//! fn main() {
//!     let cube = PlatonicSolid::Cube;
//!     assert_eq!(cube.verts - cube.edges + cube.faces, 2);
//! }
//! ```
//! 


// Lazy static reexport so we can use it from our macro
#[cfg(feature = "lazy_static")]
#[doc(hidden)]
pub use lazy_static;


/// Defines a new `enum` and implements [`Deref`] for it.
/// 
/// The `enum` will [`Deref`] to a variant-specific [`static` item].
///
/// # Examples
///
/// To specify default properties, use the following syntax (inspired by
/// [functional update syntax]):
///
/// ```rust
/// use enum_properties::enum_properties;
/// 
/// pub struct EnemyProperties {
///     pub health:     i32,
///     pub is_solid:   bool,
///     pub is_flying:  bool,
/// }
/// 
/// const DEFAULT_ENEMY_PROPERTIES: EnemyProperties = EnemyProperties {
///     health:     10,
///     is_solid:   true,
///     is_flying:  false,
/// };
/// 
/// enum_properties! {
///     #[derive(Clone, Copy, PartialEq, Eq, Debug)]
///     pub enum EnemyKind: EnemyProperties {
///         Skeleton {
///             health: 15,
///         },
///         Ghost {
///             is_solid: false,
///             is_flying: true,
///         },
///         Bat {
///             health: 1,
///             is_flying: true,
///         },
///         ..DEFAULT_ENEMY_PROPERTIES
///     }
/// }
/// ```
///
/// Non-unit variants and custom discriminants are supported too, by inserting 
/// the static initializer directly after the variant name:
///
/// ```rust
/// use enum_properties::enum_properties;
///
/// pub struct EnemyProperties {
///     pub base_health:    i32,
///     pub is_solid:       bool,
///     pub is_flying:      bool,
/// }
///
/// const DEFAULT_ENEMY_PROPERTIES: EnemyProperties = EnemyProperties {
///     base_health:    10,
///     is_solid:       true,
///     is_flying:      false,
/// };
///
/// enum_properties! {
///     #[derive(Clone, Copy, PartialEq, Eq, Debug)]
///     pub enum EnemyKind: EnemyProperties {
///         Skeleton {
///             base_health: 15,
///         } {
///             current_health: i32,
///         },
///         Ghost {
///             is_solid: false,
///             is_flying: true,
///         } {
///             is_spooky: bool,
///         },
///         Bats {
///             base_health: 1,
///             is_flying: true,
///         } (
///             // Bat count (but please name this field in an actual program)
///             u128,
///         ),
///         ..DEFAULT_ENEMY_PROPERTIES
///     }
/// }
/// ```
///
/// [`Deref`]: https://doc.rust-lang.org/std/ops/trait.Deref.html
/// [`static` item]: https://doc.rust-lang.org/reference/items/static-items.html
/// [functional update syntax]: https://doc.rust-lang.org/reference/expressions/struct-expr.html#functional-update-syntax
///
#[macro_export]
macro_rules! enum_properties {
    (
        $(#[$($enum_attribute_token:tt)*])*
        $public:vis enum $Enum:ident : $EnumProperties:ident {
            $(
                $(#[$($variant_attribute_token:tt)*])*
                $variant:ident {
                    $($field:ident : $value:expr),* $(, $(.. $default:expr)?)?
                }
                $(
                    $(@$is_struct_variant_marker:tt)?
                    {
                        $($struct_variant_content:tt)*
                    }
                )?
                $((
                    $(
                        $(#[$($tuple_attribute_token:tt)*])*
                        $(@$tuple_variant_item_marker:tt)?
                        $tuple_variant_item:ty
                    ),* $(,)?
                ))?
                $(= $discriminant:expr)?
            ),* $(,)?
        }
    ) => {
        $(#[$($enum_attribute_token)*])*
        $public enum $Enum {
            $(
                $(#[$($variant_attribute_token)*])*
                $variant
                $({$($struct_variant_content)*})?
                $((
                    $(
                        $(#[$($tuple_attribute_token)*])*
                        $tuple_variant_item
                    ),*
                ))?
                $(= $discriminant)?
            ),*
        }
        
        impl core::ops::Deref for $Enum {
            type Target = $EnumProperties;
            fn deref(&self) -> &Self::Target {
                match self {
                    $(
                        $Enum::$variant
                            $({ .. $(@$is_struct_variant_marker)?})?
                            $(($(_ $(@$tuple_variant_item_marker)?),*))?
                        => &$EnumProperties {
                            $($field: $value),* $(, $(.. $default)?)?
                        }
                    ),*
                }
            }
        }
    };
    
    (
        $(#[$($enum_attribute_token:tt)*])*
        $public:vis enum $Enum:ident : $EnumProperties:ident {
            $(
                $(#[$($variant_attribute_token:tt)*])*
                $variant:ident {
                    $($field:ident : $value:expr),* $(,)?
                }
                $(
                    $(@$is_struct_variant_marker:tt)?
                    {
                        $($struct_variant_content:tt)*
                    }
                )?
                $((
                    $(
                        $(@$tuple_variant_item_marker:tt)?
                        $(#[$($tuple_attribute_token:tt)*])*
                        $tuple_variant_item:ty
                    ),* $(,)?
                ))?
                $(= $discriminant:expr)?
            ),* , .. $default:expr
        }
    ) => {
        $(#[$($enum_attribute_token)*])*
        $public enum $Enum {
            $(
                $(#[$($variant_attribute_token)*])*
                $variant
                $({$($struct_variant_content)*})?
                $((
                    $(
                        $(#[$($tuple_attribute_token)*])*
                        $tuple_variant_item
                    ),*
                ))?
                $(= $discriminant)?
            ),*
        }
        
        impl core::ops::Deref for $Enum {
            type Target = $EnumProperties;
            fn deref(&self) -> &Self::Target {
                match self {
                    $(
                        $Enum::$variant
                            $({ .. $(@$is_struct_variant_marker)?})?
                            $(($(_ $(@$tuple_variant_item_marker)?),*))?
                        => &$EnumProperties {
                            $($field: $value),* , .. $default
                        }
                    ),*
                }
            }
        }
    };
}



/// Defines a new `enum` and implements [`Deref`] for it.
///
/// The `enum` will [`Deref`] to a variant-specific [`lazy_static` item].
/// Unlike [`enum_properties`], this macro allows properties that require
/// runtime initialization instead of being const-init.
/// Thus [`enum_properties_lazy`] allows using `Vec` and `String` and much more.
///
/// **Notice** that this macro requires the **`lazy_static`** crate feature.
///
/// # Examples
///
/// ```rust
/// use enum_properties::enum_properties_lazy;
///
/// struct Shape {
///     name:  String,
///     verts: Vec<[i32;2]>,
/// }
///
/// enum_properties_lazy! {
///     #[derive(Clone, Copy, Debug)]
///     enum SimpleShape: Shape {
///         Point {
///             name: "Point".to_string(),
///             verts: Vec::new(),
///         },
///         Line {
///             name: {
///                 // Works with arbitrary initialization code
///                 let mut builder = String::new();
///                 builder.push_str("line");
///                 builder
///             },
///             verts: {
///                 let mut vec = Vec::new();
///                 vec.push([0,0]);
///                 vec.push([1,1]);
///                 vec
///             },
///         },
///         Square {
///             name: "square".to_string(),
///             verts: vec![[0,0], [0,1], [1,0], [1,1]],
///         },
///     }
/// }
///
/// fn main() {
///     let line = SimpleShape::Line;
///     assert_eq!(&line.verts, &[[0,0], [1,1]]);
/// }
/// ```
///
/// [`Deref`]: https://doc.rust-lang.org/std/ops/trait.Deref.html
/// [`lazy_static` item]: https://crates.io/crates/lazy_static
#[cfg(feature = "lazy_static")]
#[macro_export]
macro_rules! enum_properties_lazy {
    (
        $(#[$($enum_attribute_token:tt)*])*
        $public:vis enum $Enum:ident : $EnumProperties:ident {
            $(
                $(#[$($variant_attribute_token:tt)*])*
                $variant:ident {
                    $($field:ident : $value:expr),* $(, $(.. $default:expr)?)?
                }
                $(
                    $(@$is_struct_variant_marker:tt)?
                    {
                        $($struct_variant_content:tt)*
                    }
                )?
                $((
                    $(
                        $(#[$($tuple_attribute_token:tt)*])*
                        $(@$tuple_variant_item_marker:tt)?
                        $tuple_variant_item:ty
                    ),* $(,)?
                ))?
                $(= $discriminant:expr)?
            ),* $(,)?
        }
    ) => {
        $(#[$($enum_attribute_token)*])*
        $public enum $Enum {
            $(
                $(#[$($variant_attribute_token)*])*
                $variant
                $({$($struct_variant_content)*})?
                $((
                    $(
                        $(#[$($tuple_attribute_token)*])*
                        $tuple_variant_item
                    ),*
                ))?
                $(= $discriminant)?
            ),*
        }

        impl core::ops::Deref for $Enum {
            type Target = $EnumProperties;
            fn deref(&self) -> &Self::Target {
                match self {
                    $(
                        $Enum::$variant
                            $({ .. $(@$is_struct_variant_marker)?})?
                            $(($(_ $(@$tuple_variant_item_marker)?),*))?
                        => {
                            $crate::lazy_static::lazy_static!{
                                static ref FOO: $EnumProperties = {
                                    $EnumProperties {
                                        $($field: $value),* $(, $(.. $default)?)?
                                    }
                                };
                            }

                            &*FOO
                        }
                    ),*
                }
            }
        }
    };

    (
        $(#[$($enum_attribute_token:tt)*])*
        $public:vis enum $Enum:ident : $EnumProperties:ident {
            $(
                $(#[$($variant_attribute_token:tt)*])*
                $variant:ident {
                    $($field:ident : $value:expr),* $(,)?
                }
                $(
                    $(@$is_struct_variant_marker:tt)?
                    {
                        $($struct_variant_content:tt)*
                    }
                )?
                $((
                    $(
                        $(@$tuple_variant_item_marker:tt)?
                        $(#[$($tuple_attribute_token:tt)*])*
                        $tuple_variant_item:ty
                    ),* $(,)?
                ))?
                $(= $discriminant:expr)?
            ),* , .. $default:expr
        }
    ) => {
        $(#[$($enum_attribute_token)*])*
        $public enum $Enum {
            $(
                $(#[$($variant_attribute_token)*])*
                $variant
                $({$($struct_variant_content)*})?
                $((
                    $(
                        $(#[$($tuple_attribute_token)*])*
                        $tuple_variant_item
                    ),*
                ))?
                $(= $discriminant)?
            ),*
        }

        impl core::ops::Deref for $Enum {
            type Target = $EnumProperties;
            fn deref(&self) -> &Self::Target {
                match self {
                    $(
                        $Enum::$variant
                            $({ .. $(@$is_struct_variant_marker)?})?
                            $(($(_ $(@$tuple_variant_item_marker)?),*))?
                        => {
                            $crate::lazy_static::lazy_static!{
                                static ref FOO: $EnumProperties = {
                                    $EnumProperties {
                                        $($field: $value),* , .. $default
                                    }
                                };
                            }

                            &*FOO
                        }
                    ),*
                }
            }
        }
    };
}
