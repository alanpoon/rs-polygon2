polygon2
=====

polygon 2d functions

```rust
extern crate polygon2;


use polygon2::{area, triangulate};


fn main() {
    let points = [[1.0, -1.0], [1.0, 1.0], [-1.0, 1.0], [-1.0, -1.0]];

    assert_eq!(triangulate(&points), [0, 1, 2, 0, 2, 3]);
    assert_eq!(area(&points), 4.0);
}
```
