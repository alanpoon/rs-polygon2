use number_traits::Signed;

#[inline]
pub fn intersect<T>(s1: &[T; 2], s2: &[T; 2], c1: &[T; 2], c2: &[T; 2]) -> Option<([T; 2], T, T)>
where
    T: Copy + Signed,
{
    let den = (c2[1] - c1[1]) * (s2[0] - s1[0]) - (c2[0] - c1[0]) * (s2[1] - s1[1]);

    if den == T::zero() {
        None
    } else {
        let us = ((c2[0] - c1[0]) * (s1[1] - c1[1]) - (c2[1] - c1[1]) * (s1[0] - c1[0])) / den;
        let uc = ((s2[0] - s1[0]) * (s1[1] - c1[1]) - (s2[1] - s1[1]) * (s1[0] - c1[0])) / den;

        if (us == T::zero() || us == T::one()) && (T::zero() <= uc && uc <= T::one())
            || (uc == T::zero() || uc == T::one()) && (T::zero() <= us && us <= T::one())
        {
            None
        } else if (T::zero() < us && us < T::one()) && (T::zero() < uc && uc < T::one()) {
            let x = s1[0] + us * (s2[0] - s1[0]);
            let y = s1[1] + us * (s2[1] - s1[1]);
            Some(([x, y], us, uc))
        } else {
            None
        }
    }
}
