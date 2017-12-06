use number_traits::Signed;


#[inline]
pub fn area<T>(points: &[[T; 2]]) -> T
    where T: Copy + Signed,
{
    let n = points.len();

    if n < 3 {
        T::zero()
    } else {
        let mut sum = T::zero();
        let n_minus_1 = n - 1;

        for i in 0..n {
            let a = &points[i];
            let b = if i == n_minus_1 { &points[0] } else { &points[i + 1] };
            sum += (b[0] - a[0]) * (a[1] + b[1]);
        }

        -sum / T::from_usize(2)
    }
}


#[test]
fn test_area() {
    let points = [[1.0, -1.0], [1.0, 1.0], [-1.0, 1.0], [-1.0, -1.0]];
    assert_eq!(area(&points), 4.0);

    let points = [[0.5, -0.5], [0.5, 0.5], [-0.5, 0.5], [-0.5, -0.5]];
    assert_eq!(area(&points), 1.0);
}
