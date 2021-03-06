use std::ops::{Div, Mul, Sub};
use num_traits::Signed;

use super::point_in_rect;

#[inline]
pub fn line_intersection<T>(
    out: &mut [T; 2],
    a1: &[T; 2],
    a2: &[T; 2],
    b1: &[T; 2],
    b2: &[T; 2],
) -> bool
where
    T: Clone + Signed + PartialEq + PartialOrd,
    for<'a, 'b> &'a T: Div<&'b T, Output = T> + Sub<&'b T, Output = T> + Mul<&'b T, Output = T>,
{
    let dax = &a1[0] - &a2[0];
    let dbx = &b1[0] - &b2[0];
    let day = &a1[1] - &a2[1];
    let dby = &b1[1] - &b2[1];

    let d = &(&dax * &dby) - &(&day * &dbx);
    if &d == &T::zero() {
        false
    } else {
        let ad = &(&a1[0] * &a2[1]) - &(&a1[1] * &a2[0]);
        let bd = &(&b1[0] * &b2[1]) - &(&b1[1] * &b2[0]);

        out[0] = &(&(&ad * &dbx) - &(&dax * &bd)) / &d;
        out[1] = &(&(&ad * &dby) - &(&day * &bd)) / &d;

        if point_in_rect(out, a1, a2) && point_in_rect(out, b1, b2) {
            true
        } else {
            false
        }
    }
}

#[test]
fn test_line_intersection() {
    let mut out = [0.0, 0.0];
    assert!(line_intersection(
        &mut out,
        &[0.0, 0.0],
        &[1.0, 1.0],
        &[1.0, 0.0],
        &[0.0, 1.0],
    ));
    assert_eq!(out, [0.5, 0.5]);

    assert!(!line_intersection(
        &mut [0.0, 0.0],
        &[1.0, 0.0],
        &[1.0, 1.0],
        &[-1.0, 0.0],
        &[-1.0, 1.0],
    ));
    assert!(!line_intersection(
        &mut [0.0, 0.0],
        &[0.0, 0.0],
        &[1.0, 1.0],
        &[2.0, 0.0],
        &[2.0, 1.0],
    ));
}
