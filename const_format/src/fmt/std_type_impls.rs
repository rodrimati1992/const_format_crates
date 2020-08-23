use crate::wrapper_types::PWrapper;

use super::{Error, Formatter, FormattingLength};

impl PWrapper<&str> {
    #[inline(always)]
    pub const fn const_display_len(&self, f: &mut FormattingLength) {
        f.add_len(self.0.len());
    }

    #[inline(always)]
    pub const fn const_debug_len(&self, f: &mut FormattingLength) {
        let len = self.compute_debug_len(f.flags());
        f.add_len(len);
    }

    /// Writes a `&str` with Display formatting.
    pub const fn const_display_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.w().write_whole_str(self.0)
    }

    /// Writes a `&str` with Debug formatting.
    pub const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.w().write_whole_str_debug(self.0)
    }
}

impl PWrapper<bool> {
    pub const fn const_display_len(&self, f: &mut FormattingLength) {
        const TRUE: usize = "true".len();
        const FALSE: usize = "false".len();
        f.add_len(if self.0 { TRUE } else { FALSE });
    }

    #[inline(always)]
    pub const fn const_debug_len(&self, f: &mut FormattingLength) {
        self.const_display_len(f)
    }

    /// Writes a `&str` with Display formatting.
    pub const fn const_display_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.w().write_whole_str(if self.0 { "true" } else { "false" })
    }

    /// Writes a `&str` with Debug formatting.
    #[inline(always)]
    pub const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        self.const_display_fmt(f)
    }
}

macro_rules! slice_of_std_impl {($($elem:ty),* $(,)?) => (
    $(

        impl PWrapper<&[$elem]> {
            pub const fn const_debug_len(&self, f: &mut FormattingLength) {
                let mut f = f.debug_list();
                __for_range!{i in 0..self.0.len() =>
                    PWrapper(self.0[i]).const_debug_len(f.entry());
                }
                f.finish()
            }

            pub const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
                let mut f = try_!(f.debug_list());
                __for_range!{i in 0..self.0.len() =>
                    try_!(PWrapper(self.0[i]).const_debug_fmt(try_!(f.entry())));
                }
                f.finish()
            }
        }
    )*
)}

slice_of_std_impl! {
    &str,
    u8, i8,
    u16, i16,
    u32, i32,
    u64, i64,
    u128, i128,
    usize, isize,
}
