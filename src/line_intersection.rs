use number_traits::Num;

use super::in_rect;

#[inline]
pub fn line_intersection<T>(
    a1: &[T; 2],
    a2: &[T; 2],
    b1: &[T; 2],
    b2: &[T; 2],
    out: &mut [T; 2],
) -> bool
where
    T: Copy + Num,
{
    let dax = a1[0] - a2[0];
    let dbx = b1[0] - b2[0];
    let day = a1[1] - a2[1];
    let dby = b1[1] - b2[1];

    let d = dax * dby - day * dbx;
    if d == T::zero() {
        // parallel
        false
    } else {
        let ad = a1[0] * a2[1] - a1[1] * a2[0];
        let bd = b1[0] * b2[1] - b1[1] * b2[0];

        out[0] = (ad * dbx - dax * bd) / d;
        out[1] = (ad * dby - day * bd) / d;

        if in_rect(out, a1, a2) && in_rect(out, b1, b2) {
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
        &[0.0, 0.0],
        &[1.0, 1.0],
        &[1.0, 0.0],
        &[0.0, 1.0],
        &mut out
    ));
    assert_eq!(out, [0.5, 0.5]);

    assert!(!line_intersection(
        &[1.0, 0.0],
        &[1.0, 1.0],
        &[-1.0, 0.0],
        &[-1.0, 1.0],
        &mut [0.0, 0.0]
    ));
    assert!(!line_intersection(
        &[0.0, 0.0],
        &[1.0, 1.0],
        &[2.0, 0.0],
        &[2.0, 1.0],
        &mut [0.0, 0.0]
    ));
}
