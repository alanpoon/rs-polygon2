use number_traits::Num;

#[inline]
pub fn is_convex<T>(points: &[[T; 2]]) -> bool
where
    T: Copy + Num,
{
    let n = points.len();
    if n < 3 {
        true
    } else {
        let mut i = 0;
        let l = n - 2;

        while i < l {
            if !is_triangle_convex(&points[i], &points[i + 1], &points[i + 2]) {
                return false;
            } else {
                i += 3;
            }
        }

        if !is_triangle_convex(&points[l], &points[l + 1], &points[0]) {
            return false;
        }
        if !is_triangle_convex(&points[l + 1], &points[0], &points[1]) {
            return false;
        }

        true
    }
}

#[test]
fn test_is_convex() {
    let convex_points = [[1, -1], [1, 1], [-1, 1], [1, -1], [-1, 1], [-1, -1]];
    assert!(is_convex(&convex_points));

    let concave_points = [[1, -1], [-1, 1], [1, 1], [1, -1], [-1, -1], [-1, 1]];
    assert!(!is_convex(&concave_points));
}

#[inline]
pub fn is_triangle_convex<T>(a: &[T; 2], b: &[T; 2], c: &[T; 2]) -> bool
where
    T: Copy + Num,
{
    ((a[1] - b[1]) * (c[0] - b[0]) + (b[0] - a[0]) * (c[1] - b[1])) >= T::zero()
}

#[test]
fn test_is_triangle_convex() {
    assert!(is_triangle_convex(&[0, 1], &[-1, 0], &[1, 0]));
    assert!(!is_triangle_convex(&[0, 1], &[1, 0], &[-1, 0]));
}
