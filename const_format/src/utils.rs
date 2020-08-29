/// Whether two `&str` are equal.
pub const fn str_eq(left: &str, right: &str) -> bool {
    u8_slice_eq(left.as_bytes(), right.as_bytes())
}

/// Whether two `&[u8]` are equal.
pub const fn u8_slice_eq(left: &[u8], right: &[u8]) -> bool {
    if left.len() != right.len() {
        return false;
    }

    let mut i = 0;
    while i != left.len() {
        if left[i] != right[i] {
            return false;
        }
        i += 1;
    }

    true
}

#[doc(hidden)]
#[cfg(feature = "constant_time_as_str")]
pub union Dereference<'a, T: ?Sized> {
    pub ptr: *const T,
    pub reff: &'a T,
}

////////////////////////////////////////////////////////////////////////////////

macro_rules! slice_up_to_len_alt_docs {
    ($item:item) => {
        /// A const equivalent of `&slice[..len]`.
        ///
        /// # Runtime
        ///
        /// If the "constant_time_as_str" feature is disabled,
        /// thich takes linear time to remove the trailing elements,
        /// proportional to `slice.len() - len`.
        ///
        /// If the "constant_time_as_str" feature is enabled, it takes constant time to run,
        /// but uses a few additional nightly features.
        ///
        $item
    };
}

slice_up_to_len_alt_docs! {
    #[cfg(feature = "constant_time_as_str")]
    #[inline(always)]
    pub const fn slice_up_to_len_alt<T>(slice: &[T], len: usize) -> &[T] {
        slice_up_to_len(slice, len)
    }
}
slice_up_to_len_alt_docs! {
    #[cfg(not(feature = "constant_time_as_str"))]
    pub const fn slice_up_to_len_alt<T>(slice: &[T], len: usize) -> &[T] {
        let mut rem = slice.len().saturating_add(1).saturating_sub(len);
        let mut ret = slice;

        if rem == 0 {
            return slice;
        }

        macro_rules! slice_up_to_len_alt_impl{
            (
                $( ($len:expr, [$($ignored:tt)*]) )*
            )=>{
                $(
                    while $rem > $len {
                        if let [next @ .., $($ignored)* ] = $ret {
                            $ret = next;
                            $rem -= $len;
                        }
                    }
                )*
            }
        }
        slice_up_to_len_alt_impl!{
            (36, [_, _, _, _, _, _,_, _, _, _, _, _,_, _, _, _, _, _,_, _, _, _, _, _,_, _, _, _, _, _,_, _, _, _, _, _,])
            (6, [_, _, _, _, _, _])
            (1, [_])
        }
        ret
    }
}

////////////////////////////////////////////////////////////////////////////////

macro_rules! slice_up_to_len_docs {
    ($item:item) => {
        /// A conditionally-const equivalent of `&slice[..len]`.
        ///
        /// # Constness
        ///
        /// This function takes constant time,
        /// and in order to be `const fn` it needs the "constant_time_as_str" feature,
        /// which uses some nightly Rust features.
        $item
    };
}

slice_up_to_len_docs! {
    #[cfg(feature = "constant_time_as_str")]
    #[inline]
    pub const fn slice_up_to_len(slice: &[u8], len: usize) -> &[u8] {
        ["Bug: out of bounds length!!!!"][(len > slice.len()) as usize];

        // Doing this to get a slice up to length at compile-time
        unsafe {
            let raw_slice = core::ptr::slice_from_raw_parts(slice.as_ptr(), len);
            Dereference { ptr: raw_slice }.reff
        }
    }
}

slice_up_to_len_docs! {
    #[cfg(not(feature = "constant_time_as_str"))]
    #[inline]
    pub fn slice_up_to_len(slice: &[u8], len: usize) -> &[u8] {
        &slice[..len]
    }
}

////////////////////////////////////////////////////////////////////////////////

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
    fn slice_eq_test() {
        assert!(u8_slice_eq(&[], &[]));
        assert!(!u8_slice_eq(&[], &[0]));
        assert!(!u8_slice_eq(&[0], &[]));
        assert!(u8_slice_eq(&[0], &[0]));
        assert!(!u8_slice_eq(&[0], &[1]));
        assert!(!u8_slice_eq(&[1], &[0]));
        assert!(!u8_slice_eq(&[0], &[0, 1]));
        assert!(!u8_slice_eq(&[0, 1], &[0]));
        assert!(u8_slice_eq(&[0, 1], &[0, 1]));
        assert!(!u8_slice_eq(&[0, 1], &[0, 2]));
    }

    #[test]
    fn str_eq_test() {
        assert!(str_eq("", ""));
        assert!(!str_eq("", "0"));
        assert!(!str_eq("0", ""));
        assert!(str_eq("0", "0"));
        assert!(!str_eq("0", "1"));
        assert!(!str_eq("1", "0"));
        assert!(!str_eq("0", "0, 1"));
        assert!(!str_eq("0, 1", "0"));
        assert!(str_eq("0, 1", "0, 1"));
        assert!(!str_eq("0, 1", "0, 2"));
    }

    #[test]
    fn test_slice_up_to_len_alt() {
        let mut list = [0u16; 256];

        (100..).zip(list.iter_mut()).for_each(|(i, m)| *m = i);

        fn sub_test(sub: &[u16]) {
            for j in 0..sub.len() {
                assert_eq!(slice_up_to_len_alt(sub, j), &sub[..j]);
            }
        }

        for i in 0..list.len() {
            sub_test(&list[..i]);
            sub_test(&list[i..]);
        }
    }

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
