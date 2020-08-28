use core::ops::Range;

#[cfg(feature = "const_as_str")]
pub union Dereference<'a, T: ?Sized> {
    pub ptr: *const T,
    pub reff: &'a T,
}

#[cfg(feature = "const_as_str")]
#[inline]
pub(crate) const fn slice_up_to_len(slice: &[u8], len: usize) -> &[u8] {
    ["Bug: out of bounds length!!!!"][(len > slice.len()) as usize];

    // Doing this to get a slice up to length at compile-time
    unsafe {
        let raw_slice = core::ptr::slice_from_raw_parts(slice.as_ptr(), len);
        Dereference { ptr: raw_slice }.reff
    }
}

#[cfg(not(feature = "const_as_str"))]
#[inline]
pub(crate) fn slice_up_to_len(slice: &[u8], len: usize) -> &[u8] {
    &slice[..len]
}

pub(crate) const fn min_usize(l: usize, r: usize) -> usize {
    if l < r {
        l
    } else {
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn slice_out_of_bounds() {
        slice_up_to_len(&[3, 5], 3);
    }

    #[test]
    fn slice_in_bounds() {
        assert_eq!(slice_up_to_len(&[3, 5], 0), []);
        assert_eq!(slice_up_to_len(&[3, 5], 1), [3]);
        assert_eq!(slice_up_to_len(&[3, 5], 2), [3, 5]);
    }
}
