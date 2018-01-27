use number_traits::Num;

#[inline]
pub fn area<T>(points: &[[T; 2]]) -> T
where
    T: Copy + Num,
{
    let n = points.len();

    if n < 3 {
        T::zero()
    } else {
        let mut sum = T::zero();
        let n_minus_1 = n - 1;

        for i in 0..n_minus_1 {
            let a = &points[i];
            let b = &points[i + 1];
            sum += (a[0] - b[0]) * (b[1] + a[1]);
        }
        let a = &points[n_minus_1];
        let b = &points[0];
        sum += (a[0] - b[0]) * (b[1] + a[1]);

        sum / T::from_usize(2)
    }
}

#[test]
fn test_area() {
    let points = [[1, -1], [1, 1], [-1, 1], [-1, -1]];
    assert_eq!(area(&points), 4);

    let points = [[0.5, -0.5], [0.5, 0.5], [-0.5, 0.5], [-0.5, -0.5]];
    assert_eq!(area(&points), 1.0);
}
