use number_traits::Num;


#[derive(Debug, Clone, PartialEq)]
pub struct Intersection<T>
    where T: Copy + Num,
{
    pub edge: usize,
    pub distance: T,
    pub point: [T; 2],
    pub normal: [T; 2],
}

impl<T> From<(usize, T, [T; 2], [T; 2])> for Intersection<T>
    where T: Copy + Num,
{
    #[inline(always)]
    fn from((edge, distance, point, normal): (usize, T, [T; 2], [T; 2])) -> Self {
        Intersection {
            edge: edge,
            distance: distance,
            point: point,
            normal: normal,
        }
    }
}

impl<T> Intersection<T>
    where T: Copy + Num,
{
    #[inline(always)]
    pub fn new() -> Self {
        Intersection {
            edge: 0,
            distance: T::max_value(),
            point: [T::zero(); 2],
            normal: [T::zero(); 2],
        }
    }
}
