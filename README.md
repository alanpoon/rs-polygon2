polygon2
=====

polygon 2d functions

```rust
extern crate polygon2;

use polygon2::*;

fn main() {
    let points = [[0.5, -0.5], [0.5, 0.5], [-0.5, 0.5], [-0.5, -0.5]];

    assert_eq!(triangulate(&points), [0, 1, 2, 0, 2, 3]);
    assert_eq!(area(&points), 1.0);
    assert_eq!(is_simple(&points), true);
    assert_eq!(is_convex(&points), true);

    assert_eq!(contains_point(&points, &[0.0, 0.0]), true);
    assert_eq!(contains_point(&points, &[1.0, 1.0]), false);

    assert_eq!(closest_edge(&points, &[1.0, 0.0]).edge, 0);
    assert_eq!(closest_edge(&points, &[0.0, 1.0]).edge, 1);
    assert_eq!(closest_edge(&points, &[-1.0, 0.0]).edge, 2);
    assert_eq!(closest_edge(&points, &[0.0, -1.0]).edge, 3);

    let subject = [[1.0, 0.0], [1.0, 1.0], [0.0, 1.0], [0.0, 0.0]];
    let clip = [[0.5, -0.5], [0.5, 0.5], [-0.5, 0.5], [-0.5, -0.5]];

    let difference_polygons = difference(&subject, &clip);
    let intersection_polygons = intersection(&subject, &clip);
    let union_polygons = union(&subject, &clip);

    assert_eq!(area(&difference_polygons[0]), 0.75);
    assert_eq!(area(&intersection_polygons[0]), 0.25);
    assert_eq!(area(&union_polygons[0]), 1.75);
}
```
