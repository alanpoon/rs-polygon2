use alloc::vec::Vec;

use number_traits::Signed;

use super::greiner_hormann::Polygon;

#[inline]
pub fn intersection<T>(subject: &[[T; 2]], clip: &[[T; 2]]) -> Vec<Vec<[T; 2]>>
where
    T: Copy + Signed,
{
    let mut subject_polygon = Polygon::from(subject);
    let mut clip_polygon = Polygon::from(clip);
    subject_polygon.intersection(&mut clip_polygon)
}

#[test]
fn intersection_test() {
    let subject = [[1.0, 0.0], [1.0, 1.0], [0.0, 1.0], [0.0, 0.0]];
    let clip = [[0.5, -0.5], [0.5, 0.5], [-0.5, 0.5], [-0.5, -0.5]];
    assert_eq!(
        intersection(&subject, &clip),
        [[[0.0, 0.0], [0.5, 0.0], [0.5, 0.5], [0.0, 0.5]]]
    );
}
