use number_traits::Num;


#[inline]
pub fn is_convex<T>(a: &[T; 2], b: &[T; 2], c: &[T; 2]) -> bool
    where T: Copy + Num,
{
	((a[1] - b[1]) * (c[0] - b[0]) + (b[0] - a[0]) * (c[1] - b[1])) >= T::zero()
}

#[test]
fn test_is_convex() {
    assert!(is_convex(&[0, 1], &[-1, 0], &[1, 0]));
    assert!(!is_convex(&[0, 1], &[1, 0], &[-1, 0]));
}
