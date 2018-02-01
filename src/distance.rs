use vec2;
use number_traits::{Num, Sqrt};

#[inline]
pub fn distance<T>(a: &[T; 2], b: &[T; 2]) -> T
where
    T: Copy + Num + Sqrt,
{
    let mut tmp = [T::zero(); 2];
    vec2::sub(&mut tmp, b, a);
    vec2::length(&tmp)
}

#[test]
fn distance_test() {
    assert_eq!(distance(&[0.0, 0.0], &[0.0, 1.0]), 1.0);
    assert_eq!(distance(&[-100, -100], &[100, 100]), 282);
}
