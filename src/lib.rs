#![no_std]
#![warn(missing_docs)]

//! A macro with two main uses:
//! - attaching static properties to `enum` variants
//! - using `enum` variants as "pointers" to static records
//! 
//! The advantage in both cases is that the `enum` itself contains no data, and
//! can be as small as a byte.
//! 
//! # Example
//! 
//! ```rust
//! use enum_properties::enum_properties;
//! 
//! struct SolidProperties {
//!     verts: u32,
//!     edges: u32,
//!     faces: u32,
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
//! fn main {
//!     let cube = PlatonicSolid::Cube;
//!     assert_eq!(cube.verts - cube.edges + cube.faces, 2);
//! }
//! ```
//! 

/// Defines a new `enum` and implements `Deref` for it.
/// 
/// # Syntax
/// ```ignore
/// enum_properties! {
///     [attributes]
///     [pub] enum MyEnum: MyProperties {
///         Variant1 {
///             field1: value1,
///             field2: value2,
///             ...
///         },
///         Variant2 {
///             ...
///         },
///         ...
///     }
/// }
/// ```
/// `MyEnum` will `Deref` to a variant-specific static `MyProperties`, as 
/// defined in the macro invocation.
/// 
#[macro_export]
macro_rules! enum_properties {
    (
        $(#[$($m:tt)*])*
        $public:vis enum $Enum:ident : $EnumProperties:ident {
            $($variant:ident {
                $($field:ident : $value:expr),* $(,)?
            }),* $(,)?
        }
    ) => {
        $(#[$($m)*])*
        $public enum $Enum {
            $($variant),*
        }
        
        impl core::ops::Deref for $Enum {
            type Target = $EnumProperties;
            fn deref(&self) -> &Self::Target {
                match self {
                    $($Enum::$variant => &$EnumProperties {
                        $($field: $value),*
                    }),*
                }
            }
        }
    }
}

