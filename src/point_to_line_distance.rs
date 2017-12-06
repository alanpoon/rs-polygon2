use number_traits::{Num, Sqrt};

use super::Intersection;


#[inline]
pub fn point_to_line_distance<T>(p: &[T; 2], a: &[T; 2], b: &[T; 2]) -> T
    where T: Copy + Num + Sqrt,
{
    let mut intersection = Intersection::new();
    point_to_line_intersection(p, a, b, 0, &mut intersection);
    intersection.distance
}

#[inline]
pub fn point_to_line_intersection<T>(p: &[T; 2], a: &[T; 2], b: &[T; 2], edge: usize, intersection: &mut Intersection<T>)
    where T: Copy + Num + Sqrt,
{
	let x = p[0];
    let y = p[1];
    let x1 = a[0];
    let y1 = a[1];
    let x2 = b[0];
    let y2 = b[1];

    let a = x - x1;
    let b = y - y1;
    let c = x2 - x1;
    let d = y2 - y1;

    let dot = a * c + b * d;
    let len_sq = c * c + d * d;
    let param = dot / len_sq;

    let (xx, yy) = if param <= T::zero() || (x1 == x2 && x1 == y2) {
        (x1, y1)
    } else if param >= T::one() {
        (x2, y2)
    } else {
        (x1 + param * c, y1 + param * d)
    };

    let dx = x - xx;
    let dy = y - yy;
    let distance = (dx * dx + dy * dy).sqrt();

    if distance < intersection.distance {
		intersection.distance = distance;
		intersection.edge = edge;
		intersection.point[0] = xx;
		intersection.point[1] = yy;
	}
}

#[test]
fn test_point_to_line_distance() {
    assert_eq!(point_to_line_distance(&[0, 0], &[0, 0], &[0, 1]), 0);
    assert_eq!(point_to_line_distance(&[0, 1], &[0, 0], &[0, 1]), 0);
    assert_eq!(point_to_line_distance(&[1, 1], &[0, 0], &[0, 1]), 1);
    assert_eq!(point_to_line_distance(&[-1, 0], &[0, 0], &[0, 1]), 1);
}
