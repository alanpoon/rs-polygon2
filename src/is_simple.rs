use std::ops::{Div, Mul, Sub};

use num_traits::Signed;

use super::line_intersection;

#[inline(always)]
fn abs_sub<T>(a: &T, b: &T) -> T
where
    T: PartialOrd,
    for<'a, 'b> &'a T: Sub<&'b T, Output = T>,
{
    if a > b {
        a - b
    } else {
        b - a
    }
}

#[inline]
pub fn is_simple<T>(points: &[[T; 2]]) -> bool
where
    T: Clone + Signed + PartialEq + PartialOrd,
    for<'a, 'b> &'a T: Div<&'b T, Output = T> + Sub<&'b T, Output = T> + Mul<&'b T, Output = T>,
{
    let n = points.len();

    if n < 4 {
        true
    } else {
        let mut a1 = [T::zero(), T::zero()];
        let mut a2 = [T::zero(), T::zero()];
        let mut b1 = [T::zero(), T::zero()];
        let mut b2 = [T::zero(), T::zero()];
        let mut c = [T::zero(), T::zero()];
        let n_minus_1 = n - 1;

        for i in 0..n {
            let pa1 = &points[i];
            let pa2 = if i == n_minus_1 {
                &points[0]
            } else {
                &points[i + 1]
            };

            a1[0] = pa1[0].clone();
            a1[1] = pa1[1].clone();
            a2[0] = pa2[0].clone();
            a2[1] = pa2[1].clone();

            for j in 0..n {
                if abs_sub::<usize>(&i, &j) < 2
                    || j == n_minus_1 && i == 0
                    || i == n_minus_1 && j == 0
                {
                    continue;
                }

                let pb1 = &points[j];
                let pb2 = if j == n_minus_1 {
                    &points[0]
                } else {
                    &points[j + 1]
                };

                b1[0] = pb1[0].clone();
                b1[1] = pb1[1].clone();
                b2[0] = pb2[0].clone();
                b2[1] = pb2[1].clone();

                if line_intersection(&mut c, &a1, &a2, &b1, &b2) {
                    return false;
                }
            }
        }

        true
    }
}

#[test]
fn test_is_simple() {
    let points = [[1, -1], [1, 1], [-1, 1], [-1, -1]];
    assert!(is_simple(&points));
}
#[test]
fn test_is_not_simple() {
    let points = [[1, -1], [-1, 1], [-1, -1], [1, 1]];
    assert!(!is_simple(&points));
}
