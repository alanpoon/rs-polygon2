use number_traits::Num;


#[inline]
pub fn point_in_triangle<T>(p: &[T; 2], a: &[T; 2], b: &[T; 2], c: &[T; 2]) -> bool
    where T: Copy + Num,
{
    let v0x = c[0] - a[0];
    let v0y = c[1] - a[1];
    let v1x = b[0] - a[0];
    let v1y = b[1] - a[1];
    let v2x = p[0] - a[0];
    let v2y = p[1] - a[1];

    let dot00 = v0x * v0x + v0y * v0y;
    let dot01 = v0x * v1x + v0y * v1y;
    let dot02 = v0x * v2x + v0y * v2y;
    let dot11 = v1x * v1x + v1y * v1y;
    let dot12 = v1x * v2x + v1y * v2y;

    let denom = dot00 * dot11 - dot01 * dot01;
    let u = (dot11 * dot02 - dot01 * dot12) / denom;
    let v = (dot00 * dot12 - dot01 * dot02) / denom;

    (u >= T::zero()) && (v >= T::zero()) && (u + v < T::one())
}

#[test]
fn test_point_in_triangle() {
    assert!(point_in_triangle(&[0, 5], &[0, 10], &[-10, 0], &[10, 0]));
    assert!(!point_in_triangle(&[0, -10], &[0, 10], &[-10, 0], &[10, 0]));

    assert!(point_in_triangle(&[0.0, 0.5], &[0.0, 1.0], &[-1.0, 0.0], &[1.0, 0.0]));
    assert!(!point_in_triangle(&[0.0, -1.0], &[0.0, 1.0], &[-1.0, 0.0], &[1.0, 0.0]));
}
