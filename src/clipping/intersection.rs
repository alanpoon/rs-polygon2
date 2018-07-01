use alloc::vec::Vec;
use core::ops::{Add, Div, Mul, Sub};

use num_traits::Signed;

use super::greiner_hormann::Polygon;

#[inline]
pub fn intersection<T>(subject: &[[T; 2]], clip: &[[T; 2]]) -> Vec<Vec<[T; 2]>>
where
    T: Clone + Signed + PartialEq + PartialOrd,
    for<'a, 'b> &'a T: Div<&'b T, Output = T>
        + Sub<&'b T, Output = T>
        + Add<&'b T, Output = T>
        + Mul<&'b T, Output = T>,
{
    let mut subject_polygon = Polygon::from(subject);
    let mut clip_polygon = Polygon::from(clip);
    subject_polygon.intersection(&mut clip_polygon)
}

#[test]
fn intersection_test() {
    let subject = [[1.0, 0.0], [1.0, 1.0], [0.0, 1.0], [0.0, 0.0]];
    let clip = [[0.5, -0.5], [0.5, 0.5], [-0.5, 0.5], [-0.5, -0.5]];
    assert_eq!(
        intersection(&subject, &clip),
        [[[0.0, 0.0], [0.5, 0.0], [0.5, 0.5], [0.0, 0.5]]]
    );
}
