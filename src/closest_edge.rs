use number_traits::{Sqrt, Num};

use super::{point_to_line_intersection, Intersection};


#[inline]
pub fn closest_edge<T>(p: &[T; 2], points: &[[T; 2]]) -> Intersection<T>
    where T: Copy + Num + Sqrt,
{
    let n = points.len();
    let px = p[0];
    let py = p[1];
    let mut b1 = [T::zero(); 2];
    let mut b2 = [T::zero(); 2];

    let mut intersection = Intersection::new();

    let mut i = 0;
    while i < n {
        let p1 = &points[i];
        let p2 = &points[i + 1];

        b1[0] = p1[0];
        b1[1] = p1[1];

        b2[0] = p2[0];
        b2[1] = p2[1];

        point_to_line_intersection(p, &b1, &b2, i, &mut intersection);

        i += 2;
    }

    b1[0] = b2[0];
    b1[1] = b2[1];
    b2[0] = points[0][0];
    b2[1] = points[0][1];
    point_to_line_intersection(p, &b1, &b2, 0, &mut intersection);

	intersection.normal[0] = (px - intersection.point[0]) / intersection.distance;
	intersection.normal[1] = (py - intersection.point[1]) / intersection.distance;

    intersection
}


#[test]
fn test_closest_edge() {
    let points = [[1.0, -1.0], [1.0, 1.0], [-1.0, 1.0], [-1.0, -1.0]];

    let intersection = closest_edge(&[2.0, -1.0], &points);
    assert_eq!(intersection, Intersection::from((0, 1.0, [1.0, -1.0], [1.0, 0.0])));

    let intersection = closest_edge(&[-2.0, 1.0], &points);
    assert_eq!(intersection, Intersection::from((2, 1.0, [-1.0, 1.0], [-1.0, 0.0])));
}
