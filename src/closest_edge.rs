use std::ops::{Add, Div, Mul, Sub};

use num_traits::{Bounded, Float};
use vec2;

use super::{point_to_line_intersection, Intersection};

#[inline]
pub fn closest_edge<T>(points: &[[T; 2]], p: &[T; 2]) -> Intersection<T>
where
    T: Clone + Float + Bounded + PartialOrd,
    for<'a, 'b> &'a T: Div<&'b T, Output = T>
        + Sub<&'b T, Output = T>
        + Add<&'b T, Output = T>
        + Mul<&'b T, Output = T>,
{
    let n = points.len();
    let n_minus_1 = n - 1;
    let px = p[0].clone();
    let py = p[1].clone();

    let mut intersection = Intersection::new();

    let mut i = 0;
    while i < n {
        let p1 = &points[i];
        let p2 = if i == n_minus_1 {
            &points[0]
        } else {
            &points[i + 1]
        };

        point_to_line_intersection(&mut intersection, p, p1, p2, i);

        i += 1;
    }

    if &intersection.distance != &T::zero() {
        intersection.normal[0] = &(&px - &intersection.point[0]) / &intersection.distance;
        intersection.normal[1] = &(&py - &intersection.point[1]) / &intersection.distance;
    } else {
        intersection.normal[0] = T::zero();
        intersection.normal[1] = T::one();
    }

    intersection
}

#[test]
fn test_closest_edge() {
    let points = [[0.5, -0.5], [0.5, 0.5], [-0.5, 0.5], [-0.5, -0.5]];

    let intersection = closest_edge(&points, &[1.0, 0.0]);
    assert_eq!(
        intersection,
        Intersection::from((0, 0.5, [0.5, 0.0], [1.0, 0.0]))
    );

    let intersection = closest_edge(&points, &[0.0, 1.0]);
    assert_eq!(
        intersection,
        Intersection::from((1, 0.5, [0.0, 0.5], [0.0, 1.0]))
    );

    let intersection = closest_edge(&points, &[-1.0, 0.0]);
    assert_eq!(
        intersection,
        Intersection::from((2, 0.5, [-0.5, 0.0], [-1.0, 0.0]))
    );

    let intersection = closest_edge(&points, &[0.0, -1.0]);
    assert_eq!(
        intersection,
        Intersection::from((3, 0.5, [0.0, -0.5], [0.0, -1.0]))
    );
}

#[inline]
pub fn closest_edge_offset_angle<T>(
    points: &[[T; 2]],
    offset: &[T; 2],
    angle: T,
    p: &[T; 2],
) -> Intersection<T>
where
    T: Clone + Float + Bounded + PartialOrd,
    for<'a, 'b> &'a T: Div<&'b T, Output = T>
        + Sub<&'b T, Output = T>
        + Add<&'b T, Output = T>
        + Mul<&'b T, Output = T>,
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
        vec2::transform_angle(&mut tmp2, &tmp1, &angle);
        vec2::add(&mut b1, offset, &tmp2);

        tmp1[0] = p2[0];
        tmp1[1] = p2[1];
        vec2::transform_angle(&mut tmp2, &tmp1, &angle);
        vec2::add(&mut b2, offset, &tmp2);

        point_to_line_intersection(&mut intersection, p, &b1, &b2, i);

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
fn test_closest_edge_offset_angle() {
    let points = [[0.5, -0.5], [0.5, 0.5], [-0.5, 0.5], [-0.5, -0.5]];

    let intersection = closest_edge_offset_angle(&points, &[0.5, 0.5], 0.0, &[1.5, 0.5]);
    assert_eq!(
        intersection,
        Intersection::from((0, 0.5, [1.0, 0.5], [1.0, 0.0]))
    );

    let intersection = closest_edge_offset_angle(&points, &[0.5, 0.5], 0.0, &[0.5, 1.5]);
    assert_eq!(
        intersection,
        Intersection::from((1, 0.5, [0.5, 1.0], [0.0, 1.0]))
    );

    let intersection = closest_edge_offset_angle(&points, &[0.5, 0.5], 0.0, &[-0.5, 0.5]);
    assert_eq!(
        intersection,
        Intersection::from((2, 0.5, [0.0, 0.5], [-1.0, 0.0]))
    );

    let intersection = closest_edge_offset_angle(&points, &[0.5, 0.5], 0.0, &[0.5, -0.5]);
    assert_eq!(
        intersection,
        Intersection::from((3, 0.5, [0.5, 0.0], [0.0, -1.0]))
    );
}
