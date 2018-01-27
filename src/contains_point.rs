use number_traits::Signed;

#[inline]
pub fn contains_point<T>(p: &[T; 2], points: &[[T; 2]]) -> bool
where
    T: Copy + Signed,
{
    let n = points.len();
    let px = p[0];
    let py = p[1];

    let a = points[n - 1];
    let mut b = points[n - 2];
    let mut ax;
    let mut ay = a[1] - py;
    let mut bx = b[0] - px;
    let mut by = b[1] - py;

    let mut lup = by > ay;
    for i in 0..n {
        // ax = bx;
        ay = by;
        b = points[i];
        bx = b[0] - px;
        by = b[1] - py;

        if ay == by {
            continue;
        }
        lup = by > ay;
    }

    let mut depth = 0;
    for i in 0..n {
        ax = bx;
        ay = by;
        let b = points[i];
        bx = b[0] - px;
        by = b[1] - py;

        if ay < T::zero() && by < T::zero() {
            // both "up" or both "down"
            continue;
        }
        if ay > T::zero() && by > T::zero() {
            // both "up" or both "down"
            continue;
        }
        if ax < T::zero() && bx < T::zero() {
            // both points on the left
            continue;
        }

        if ay == by && ax.min(&bx) <= T::zero() {
            return true;
        }
        if ay == by {
            continue;
        }

        let lx = ax + (bx - ax) * (-ay) / (by - ay);
        if lx == T::zero() {
            // point on edge
            return true;
        }
        if lx > T::zero() {
            depth += 1;
        }
        if ay == T::zero() && lup && by > ay {
            // hit vertex, both up
            depth -= 1;
        }
        if ay == T::zero() && !lup && by < ay {
            // hit vertex, both down
            depth -= 1;
        }

        lup = by > ay;
    }

    return (depth & 1) == 1;
}

#[test]
fn test_contains_point() {
    let points = [[1, -1], [1, 1], [-1, 1], [-1, -1]];
    assert!(contains_point(&[0, 0], &points));
    assert!(contains_point(&[1, 0], &points));
    assert!(!contains_point(&[2, 0], &points));
}
