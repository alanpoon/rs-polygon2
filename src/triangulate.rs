use alloc::vec::Vec;

use number_traits::Num;

use super::{point_in_triangle, is_triangle_convex};


#[inline]
pub fn triangulate<T>(points: &[[T; 2]]) -> Vec<usize>
    where T: Copy + Num,
{
    let len = points.len();
    let mut tgs = Vec::new();

    if len < 3 {
        tgs
    } else {
        let mut avl = Vec::with_capacity(len);

        for i in 0..len {
            avl.push(i);
        }

        let mut i = 0;
        let mut al = len;
        while al > 3 {
            let i0 = avl[i % al];
			let i1 = avl[(i + 1) % al];
			let i2 = avl[(i + 2) % al];

			let a = &points[i0];
			let b = &points[i1];
			let c = &points[i2];

            let mut ear_found = false;
            if is_triangle_convex(a, b, c) {
                ear_found = true;

                for j in 0..al {
                    let vi = avl[j];

					if vi != i0 && vi != i1 && vi != i2 {
                        if point_in_triangle(&points[vi], a, b, c) {
                            ear_found = false;
                            break;
                        }
                    }
                }
            }

            if ear_found {
                tgs.push(i0);
                tgs.push(i1);
                tgs.push(i2);
				avl.remove((i + 1) % al);
				al -= 1;
				i = 0;
            } else if i > 3 * al {
                break;
            } else {
                i += 1;
            }
        }

        tgs.push(avl[0]);
        tgs.push(avl[1]);
        tgs.push(avl[2]);

        tgs
    }
}

#[test]
fn test_triangulate() {
    let points = [[1, -1], [1, 1], [-1, 1], [-1, -1]];
    let tgs = triangulate(&points);
    assert_eq!(tgs, [0, 1, 2, 0, 2, 3]);
}
