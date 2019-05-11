use std::ops::{Add, Div, Mul, Neg, Sub};
use num_traits::Signed;

#[inline]
pub fn contains_point<T>(points: &[[T; 2]], p: &[T; 2]) -> bool
where
    T: Clone + Signed + PartialOrd,
    for<'a, 'b> &'a T: Div<&'b T, Output = T>
        + Sub<&'b T, Output = T>
        + Add<&'b T, Output = T>
        + Mul<&'b T, Output = T>
        + Neg<Output = T>,
{
    let n = points.len();
    let px = &p[0];
    let py = &p[1];

    let a = points[n - 1].clone();
    let mut b = points[n - 2].clone();
    let mut ax;
    let mut ay = &a[1] - &py;
    let mut bx = &b[0] - &px;
    let mut by = &b[1] - &py;

    let mut lup = &by > &ay;
    for i in 0..n {
        // ax = bx;
        ay = by;
        b = points[i].clone();
        bx = &b[0] - &px;
        by = &b[1] - &py;

        if ay == by {
            continue;
        }
        lup = by > ay;
    }

    let mut depth = 0;
    for i in 0..n {
        ax = bx;
        ay = by;
        let b = &points[i];
        bx = &b[0] - &px;
        by = &b[1] - &py;

        if &ay < &T::zero() && &by < &T::zero() {
            // both "up" or both "down"
            continue;
        }
        if &ay > &T::zero() && &by > &T::zero() {
            // both "up" or both "down"
            continue;
        }
        if &ax < &T::zero() && &bx < &T::zero() {
            // both points on the left
            continue;
        }

        if &ay == &by && (if &ax < &bx {
            ax.clone()
        } else {
            bx.clone()
        }) <= T::zero()
        {
            return true;
        }
        if &ay == &by {
            continue;
        }

        let lx = &ax + &(&(&(&bx - &ax) * &-&ay) / &(&by - &ay));
        if &lx == &T::zero() {
            // point on edge
            return true;
        }
        if &lx > &T::zero() {
            depth += 1;
        }
        if &ay == &T::zero() && lup && &by > &ay {
            // hit vertex, both up
            depth -= 1;
        }
        if &ay == &T::zero() && !lup && &by < &ay {
            // hit vertex, both down
            depth -= 1;
        }

        lup = &by > &ay;
    }

    return (depth & 1) == 1;
}

#[test]
fn test_contains_point() {
    let points = [[1, -1], [1, 1], [-1, 1], [-1, -1]];
    assert!(contains_point(&points, &[0, 0]));
    assert!(contains_point(&points, &[1, 0]));
    assert!(!contains_point(&points, &[2, 0]));
}
