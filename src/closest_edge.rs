use vec2;
use number_traits::{Num, Sqrt, Trig};

use super::{point_to_line_intersection, Intersection};

#[inline]
pub fn closest_edge<T>(p: &[T; 2], points: &[[T; 2]]) -> Intersection<T>
where
    T: Copy + Num + Sqrt + ::core::fmt::Debug,
{
    let n = points.len();
    let n_minus_1 = n - 1;
    let px = p[0];
    let py = p[1];

    let mut intersection = Intersection::new();

    let mut i = 0;
    while i < n {
        let p1 = &points[i];
        let p2 = if i == n_minus_1 {
            &points[0]
        } else {
            &points[i + 1]
        };

        point_to_line_intersection(p, p1, p2, i, &mut intersection);

        i += 1;
    }

    if intersection.distance != T::zero() {
        intersection.normal[0] = (px - intersection.point[0]) / intersection.distance;
        intersection.normal[1] = (py - intersection.point[1]) / intersection.distance;
    } else {
        intersection.normal[0] = T::zero();
        intersection.normal[1] = T::one();
    }

    intersection
}

#[test]
fn test_closest_edge() {
    let points = [[0.5, -0.5], [0.5, 0.5], [-0.5, 0.5], [-0.5, -0.5]];

    let intersection = closest_edge(&[1.0, 0.0], &points);
    assert_eq!(
        intersection,
        Intersection::from((0, 0.5, [0.5, 0.0], [1.0, 0.0]))
    );

    let intersection = closest_edge(&[0.0, 1.0], &points);
    assert_eq!(
        intersection,
        Intersection::from((1, 0.5, [0.0, 0.5], [0.0, 1.0]))
    );

    let intersection = closest_edge(&[-1.0, 0.0], &points);
    assert_eq!(
        intersection,
        Intersection::from((2, 0.5, [-0.5, 0.0], [-1.0, 0.0]))
    );

    let intersection = closest_edge(&[0.0, -1.0], &points);
    assert_eq!(
        intersection,
        Intersection::from((3, 0.5, [0.0, -0.5], [0.0, -1.0]))
    );
}

#[inline]
pub fn closest_edge_offset_angle<T>(
    p: &[T; 2],
    offset: &[T; 2],
    angle: T,
    points: &[[T; 2]],
) -> Intersection<T>
where
    T: Copy + Num + Trig + Sqrt,
{
    let n = points.len();
    let n_minus_1 = n - 1;
    let px = p[0];
    let py = p[1];
    let mut b1 = [T::zero(); 2];
    let mut b2 = [T::zero(); 2];

    let mut tmp1 = [T::zero(); 2];
    let mut tmp2 = [T::zero(); 2];

    let mut intersection = Intersection::new();

    let mut i = 0;
    while i < n {
        let p1 = &points[i];
        let p2 = if i == n_minus_1 {
            &points[0]
        } else {
            &points[i + 1]
        };

        tmp1[0] = p1[0];
        tmp1[1] = p1[1];
        vec2::transform_angle(&mut tmp2, &tmp1, angle);
        vec2::add(&mut b1, offset, &tmp2);

        tmp1[0] = p2[0];
        tmp1[1] = p2[1];
        vec2::transform_angle(&mut tmp2, &tmp1, angle);
        vec2::add(&mut b2, offset, &tmp2);

        point_to_line_intersection(p, &b1, &b2, i, &mut intersection);

        i += 2;
    }

    if intersection.distance != T::zero() {
        intersection.normal[0] = (px - intersection.point[0]) / intersection.distance;
        intersection.normal[1] = (py - intersection.point[1]) / intersection.distance;
    } else {
        intersection.normal[0] = T::zero();
        intersection.normal[1] = T::one();
    }

    intersection
}

#[test]
fn test_closest_edge_offset_angle() {
    let points = [[0.5, -0.5], [0.5, 0.5], [-0.5, 0.5], [-0.5, -0.5]];

    let intersection = closest_edge_offset_angle(&[0.0, 0.0], &[0.5, 0.5], 0.0, &points);
    assert_eq!(
        intersection,
        Intersection::from((2, 0.0, [0.0, 0.0], [0.0, 1.0]))
    );
}
