# spatial

A library for generic spatial data structures.

## Quadtree

In order for an object to be inserted into a quadtree, the
`quadtree::Index`-trait must be implemented.

```rust
use quadtree;

struct Object {
    x: u16,
    y: u16
}

impl quadtree::Index<u16> for Object {
    fn x(&self) -> u16 {
        self.x
    }

    fn y(&self) -> u16 {
        self.y
    }
}
```

To construct a quadtree, a bounding volume is needed.

```rust
// arguments are in format `(x, y), (width, height)`
let volume = quadtree::Volume::new((0, 0), (640, 480));
let mut tree = quadtree::Quadtree::new(volume);
```

Now the quadtree is ready for insertion and querying.

```rust
if tree.insert(Object { x: 68, y: 194 }) {
    println!("object inserted successfully!");
}

let objects = tree.get_in_volume(quadtree::Volume::new((0, 0), (200, 200)));
```
