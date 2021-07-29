pub(crate) const fn max_usize(l: usize, r: usize) -> usize {
    if l > r {
        l
    } else {
        r
    }
}
pub(crate) const fn saturating_add(l: usize, r: usize) -> usize {
    let (sum, overflowed) = l.overflowing_add(r);
    if overflowed {
        usize::MAX
    } else {
        sum
    }
}

pub union PtrToRef<'a, T: ?Sized> {
    pub ptr: *const T,
    pub reff: &'a T,
}
