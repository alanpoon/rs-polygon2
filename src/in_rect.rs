use number_traits::Num;

#[inline]
pub fn in_rect<T>(p: &[T; 2], a: &[T; 2], b: &[T; 2]) -> bool
where
    T: Copy + Num,
{
    let minx = a[0].min(&b[0]);
    let maxx = a[0].max(&b[0]);
    let miny = a[1].min(&b[1]);
    let maxy = a[1].max(&b[1]);

    if minx == maxx {
        miny < p[1] && p[1] < maxy
    } else if miny == maxy {
        minx < p[0] && p[0] < maxx
    } else {
        (minx < p[0] && p[0] < maxx && miny < p[1] && p[1] < maxy)
    }
}

#[test]
fn test_in_rect() {
    assert!(in_rect(&[5, 5], &[0, 0], &[10, 10]));
    assert!(!in_rect(&[-1, -1], &[0, 0], &[10, 10]));
}
