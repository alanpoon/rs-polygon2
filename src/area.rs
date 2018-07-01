use core::ops::{Add, Div, Mul, Sub};

use num_traits::{FromPrimitive, Signed};

#[inline]
pub fn area<T>(points: &[[T; 2]]) -> T
where
    T: Signed + FromPrimitive,
    for<'a, 'b> &'a T: Div<&'b T, Output = T>
        + Sub<&'b T, Output = T>
        + Add<&'b T, Output = T>
        + Mul<&'b T, Output = T>,
{
    let n = points.len();

    if n < 3 {
        T::zero()
    } else {
        let mut sum = T::zero();
        let n_minus_1 = n - 1;

        for i in 0..n_minus_1 {
            let a = &points[i];
            let b = &points[i + 1];
            sum = &sum + &(&(&a[0] - &b[0]) * &(&b[1] + &a[1]));
        }
        let a = &points[n_minus_1];
        let b = &points[0];
        sum = &sum + &(&(&a[0] - &b[0]) * &(&b[1] + &a[1]));

        &sum / &T::from_usize(2).unwrap()
    }
}

#[test]
fn test_area() {
    let points = [[1, -1], [1, 1], [-1, 1], [-1, -1]];
    assert_eq!(area(&points), 4);

    let points = [[0.5, -0.5], [0.5, 0.5], [-0.5, 0.5], [-0.5, -0.5]];
    assert_eq!(area(&points), 1.0);
}
