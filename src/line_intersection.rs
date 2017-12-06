use number_traits::Num;

use super::in_rect;


#[inline]
pub fn line_intersection<T>(a0: &[T; 2], a1: &[T; 2], b0: &[T; 2], b1: &[T; 2], out: &mut [T; 2]) -> bool
    where T: Copy + Num,
{
    let dax = a0[0] - a1[0];
    let dbx = b0[0] - b1[0];
    let day = a0[1] - a1[1];
    let dby = b0[1] - b1[1];

    let den = dax * dby - day * dbx;
    if den == T::zero() { // parallel
        false
    } else {
        let a = a0[0] * a1[1] - a0[1] * a1[0];
        let b = b0[0] * b1[1] - b0[1] * b1[0];

        out[0] = (a * dbx - dax * b) / den;
        out[1] = (a * dby - day * b) / den;

        if in_rect(out, a0, a1) && in_rect(out, b0, b1) {
            true
        } else {
            false
        }
    }
}

#[test]
fn test_line_intersection() {
    let mut out = [0.0, 0.0];
    assert!(line_intersection(&[0.0, 0.0], &[1.0, 1.0], &[1.0, 0.0], &[0.0, 1.0], &mut out));
    assert_eq!(out, [0.5, 0.5]);
}
