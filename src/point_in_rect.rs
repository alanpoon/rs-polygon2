use num_traits::Signed;

#[inline]
pub fn point_in_rect<T>(p: &[T; 2], a: &[T; 2], b: &[T; 2]) -> bool
where
    T: Clone + Signed + PartialEq + PartialOrd,
{
    let (minx, maxx) = if &a[0] < &b[0] {
        (&a[0], &b[0])
    } else {
        (&b[0], &a[0])
    };
    let (miny, maxy) = if &a[1] < &b[1] {
        (&a[1], &b[1])
    } else {
        (&b[1], &a[1])
    };

    if minx == maxx {
        miny < &p[1] && &p[1] < maxy
    } else if &miny == &maxy {
        minx < &p[0] && &p[0] < maxx
    } else {
        (minx < &p[0] && &p[0] < maxx && miny < &p[1] && &p[1] < maxy)
    }
}

#[test]
fn test_point_in_rect() {
    assert!(point_in_rect(&[5, 5], &[0, 0], &[10, 10]));
    assert!(!point_in_rect(&[-1, -1], &[0, 0], &[10, 10]));
}
