# enum_properties
[![Documentation](https://docs.rs/enum_properties/badge.svg)](https://docs.rs/enum_properties/)

A macro for declaring static properties on enum variants. See the [documentation](https://docs.rs/enum_properties/) for more information.

## Example

```rust
use enum_properties::enum_properties;

struct SolidProperties {
    verts: u32,
    edges: u32,
    faces: u32,
}

enum_properties! {
    #[derive(Clone, Copy, Debug)]
    enum PlatonicSolid: SolidProperties {
        Tetrahedron {
            verts: 4,
            edges: 6,
            faces: 4,
        },
        Cube {
            verts: 8,
            edges: 12,
            faces: 6,
        },
        Octahedron {
            verts: 6,
            edges: 12,
            faces: 8,
        },
        Dodecahedron {
            verts: 20,
            edges: 30,
            faces: 12,
        },
        Icosahedron {
            verts: 12,
            edges: 30,
            faces: 20,
        },
    }
}

fn main {
    let cube = PlatonicSolid::Cube;
    assert_eq!(cube.verts - cube.edges + cube.faces, 2);
}
```
