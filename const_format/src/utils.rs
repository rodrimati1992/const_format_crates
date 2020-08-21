pub union Transmute<F: Copy, T: Copy> {
    pub from: F,
    pub to: T,
}

pub(crate) const fn min_usize(l: usize, r: usize) -> usize {
    if l < r {
        l
    } else {
        r
    }
}
