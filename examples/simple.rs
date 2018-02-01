extern crate polygon2;

fn main() {
    let points = [[0.5, -0.5], [0.5, 0.5], [-0.5, 0.5], [-0.5, -0.5]];

    assert_eq!(polygon2::triangulate(&points), [0, 1, 2, 0, 2, 3]);
    assert_eq!(polygon2::area(&points), 1.0);
    assert_eq!(polygon2::is_simple(&points), true);
    assert_eq!(polygon2::is_convex(&points), true);

    assert_eq!(polygon2::contains_point(&points, &[0.0, 0.0]), true);
    assert_eq!(polygon2::contains_point(&points, &[1.0, 1.0]), false);

    assert_eq!(polygon2::closest_edge(&points, &[1.0, 0.0]).edge, 0);
    assert_eq!(polygon2::closest_edge(&points, &[0.0, 1.0]).edge, 1);
    assert_eq!(polygon2::closest_edge(&points, &[-1.0, 0.0]).edge, 2);
    assert_eq!(polygon2::closest_edge(&points, &[0.0, -1.0]).edge, 3);

    let subject = [[1.0, 0.0], [1.0, 1.0], [0.0, 1.0], [0.0, 0.0]];
    let clip = [[0.5, -0.5], [0.5, 0.5], [-0.5, 0.5], [-0.5, -0.5]];

    let difference_polygons = polygon2::difference(&subject, &clip);
    let intersection_polygons = polygon2::intersection(&subject, &clip);
    let union_polygons = polygon2::union(&subject, &clip);

    assert_eq!(polygon2::area(&difference_polygons[0]), 0.75);
    assert_eq!(polygon2::area(&intersection_polygons[0]), 0.25);
    assert_eq!(polygon2::area(&union_polygons[0]), 1.75);
}
