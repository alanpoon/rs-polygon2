use number_traits::Num;

use super::line_intersection;

#[inline]
pub fn is_simple<T>(points: &[[T; 2]]) -> bool
where
    T: Copy + Num + ::core::fmt::Debug,
{
    let n = points.len();

    if n < 4 {
        true
    } else {
        let mut a1 = [T::zero(); 2];
        let mut a2 = [T::zero(); 2];
        let mut b1 = [T::zero(); 2];
        let mut b2 = [T::zero(); 2];
        let mut c = [T::zero(); 2];
        let n_minus_1 = n - 1;

        for i in 0..n {
            let pa1 = &points[i];
            let pa2 = if i == n_minus_1 {
                &points[0]
            } else {
                &points[i + 1]
            };

            a1[0] = pa1[0];
            a1[1] = pa1[1];
            a2[0] = pa2[0];
            a2[1] = pa2[1];

            for j in 0..n {
                if Num::abs_diff(&i, &j) < 2 || j == n_minus_1 && i == 0 || i == n_minus_1 && j == 0
                {
                    continue;
                }

                let pb1 = &points[j];
                let pb2 = if j == n_minus_1 {
                    &points[0]
                } else {
                    &points[j + 1]
                };

                b1[0] = pb1[0];
                b1[1] = pb1[1];
                b2[0] = pb2[0];
                b2[1] = pb2[1];

                if line_intersection(&a1, &a2, &b1, &b2, &mut c) {
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
