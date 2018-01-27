extern crate polygon2;

use polygon2::{area, closest_edge, contains_point, is_convex, is_simple, triangulate};

fn main() {
    let points = [[0.5, -0.5], [0.5, 0.5], [-0.5, 0.5], [-0.5, -0.5]];

    assert_eq!(triangulate(&points), [0, 1, 2, 0, 2, 3]);
    assert_eq!(area(&points), 1.0);
    assert_eq!(is_simple(&points), true);
    assert_eq!(is_convex(&points), true);

    assert_eq!(contains_point(&[0.0, 0.0], &points), true);
    assert_eq!(contains_point(&[1.0, 1.0], &points), false);

    assert_eq!(closest_edge(&[1.0, 0.0], &points).edge, 0);
    assert_eq!(closest_edge(&[0.0, 1.0], &points).edge, 1);
    assert_eq!(closest_edge(&[-1.0, 0.0], &points).edge, 2);
    assert_eq!(closest_edge(&[0.0, -1.0], &points).edge, 3);
}
