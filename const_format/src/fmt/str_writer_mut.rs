use crate::{
    formatting::{hex_as_ascii, ForEscaping, FormattingFlags, NumberFormatting, FOR_ESCAPING},
    utils::min_usize,
    wrapper_types::{AsciiStr, PWrapper},
};

use super::{Error, Formatter, StrWriter};

use core::ops::Range;

/// A handle to write a formatted utf8 string into a `[u8]`.
///
pub struct StrWriterMut<'w> {
    pub(super) len: &'w mut usize,
    pub(super) buffer: &'w mut [u8],
}

macro_rules! borrow_fields {
    ($self:ident, $len:ident, $buffer:ident) => {
        let $len = &mut *$self.len;
        let $buffer = &mut *$self.buffer;
    };
}

impl<'w> StrWriterMut<'w> {
    pub const fn new(writer: &'w mut StrWriter) -> Self {
        Self {
            len: &mut writer.len,
            buffer: &mut writer.buffer,
        }
    }

    /// Construct a `StrWriterMut` from length and byte slice mutable references.
    ///
    /// # Safety
    ///
    /// The bytes up to (and excluding) `length` in `buffer` must be valid utf8.
    pub const unsafe fn from_custom(length: &'w mut usize, buffer: &'w mut [u8]) -> Self {
        *length = min_usize(*length, buffer.len());

        Self {
            len: length,
            buffer,
        }
    }

    /// Construct a `StrWriterMut` from length and byte slice mutable references.
    ///
    pub const fn from_custom_cleared(length: &'w mut usize, buffer: &'w mut [u8]) -> Self {
        *length = 0;

        Self {
            len: length,
            buffer,
        }
    }
}

impl<'w> StrWriterMut<'w> {
    #[inline(always)]
    pub const fn len(&self) -> usize {
        *self.len
    }

    #[inline(always)]
    pub const fn buffer(&self) -> &[u8] {
        self.buffer
    }

    #[inline(always)]
    pub const fn capacity(&self) -> usize {
        self.buffer.len()
    }

    conditionally_const! {
        feature = "const_as_str";
        /// Gets the written part of this StrWriterMut as a `&str`
        ///
        /// This can be called in const contexts by enabling the "const_as_str" feature,
        /// which requires nightly Rust versions after 2020-08-15.
        ///
        #[inline(always)]
        pub fn as_str(&self) -> &str {
            // All the methods that modify the buffer must ensure utf8 validity,
            // only methods from this module need to ensure this.
            unsafe { core::str::from_utf8_unchecked(self.as_bytes()) }
        }

        /// Gets the written part of this StrWriterMut as a `&[u8]`
        ///
        /// The slice is guaranteed to be valid utf8, so this is mostly for convenience.
        ///
        /// This can be called in const contexts by enabling the "const_as_str" feature,
        /// which requires nightly Rust versions after 2020-08-15.
        ///
        #[inline(always)]
        pub fn as_bytes(&self) -> &[u8] {
            crate::utils::slice_up_to_len(self.buffer, *self.len)
        }
    }

    #[inline(always)]
    pub const fn make_formatter(&mut self, flags: FormattingFlags) -> Formatter<'_> {
        Formatter::from_sw_mut(self.reborrow(), flags)
    }

    // For borrowing this mutably in macros, without getting nested mutable references.
    #[inline(always)]
    pub const fn borrow_mutably(&mut self) -> &mut StrWriterMut<'w> {
        self
    }

    // For borrowing this mutably in macros, without getting nested mutable references.
    #[inline(always)]
    pub const fn reborrow(&mut self) -> StrWriterMut<'_> {
        StrWriterMut {
            len: self.len,
            buffer: self.buffer,
        }
    }

    /// Truncates this `StrWriterMut` to `length`.
    ///
    /// If `length` is greater than the current length, this does nothing.
    ///
    /// # Errors
    ///
    /// Returns an `Error::NotOnCharBoundary` if `length` is not on a char boundary.
    #[inline]
    pub const fn truncate(&mut self, length: usize) -> Result<(), Error> {
        if length <= *self.len {
            if !is_valid_str_index(self.buffer, length) {
                return Err(Error::NotOnCharBoundary);
            }

            *self.len = length;
        }
        Ok(())
    }

    /// Truncates this `StrWriterMut` to length 0.
    #[inline]
    pub const fn clear(&mut self) {
        *self.len = 0;
    }
}

/////////////////////////////////////////////////////////////////////////////////

macro_rules! write_integer_fn {
    (
        $(($display_fn:ident, $debug_fn:ident, $sign:ident, $ty:ident, $Unsigned:ident))*
    )=>{
        impl StrWriterMut<'_>{
            $(
                write_integer_fn!{
                    @methods
                    $display_fn, $debug_fn, $sign, ($ty, $Unsigned), stringify!($ty)
                }
            )*
        }

        $(
            write_integer_fn!{
                @pwrapper
                $display_fn, $debug_fn, $sign, ($ty, $Unsigned), stringify!($ty)
            }
        )*
    };
    (@pwrapper
        $display_fn:ident,
        $debug_fn:ident,
        $sign:ident,
        ($ty:ident, $Unsigned:ident),
        $ty_name:expr
    )=>{
        impl PWrapper<$ty> {
            /// Writes a
            #[doc = $ty_name]
            /// with Display formatting.
            pub const fn const_display_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
                f.$display_fn(self.0)
            }

            /// Writes a
            #[doc = $ty_name]
            /// with Debug formatting.
            pub const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
                f.$debug_fn(self.0)
            }
        }
    };
    (@methods
        $display_fn:ident,
        $debug_fn:ident,
        $sign:ident,
        ($ty:ident, $Unsigned:ident),
        $ty_name:expr
    )=>{
        /// Writes a
        #[doc = $ty_name]
        /// with Display formatting.
        pub const fn $display_fn(&mut self, number: $ty) -> Result<(), Error> {
            borrow_fields!(self, this_len, this_buffer);

            let n = PWrapper(number);
            let len = n.compute_display_len(FormattingFlags::DEFAULT);

            let mut cursor = *this_len + len;

            if cursor > this_buffer.len() {
                return Err(Error::NotEnoughSpace);
            }

            write_integer_fn!(@unsigned_abs $sign, n);

            loop {
                cursor-=1;
                let digit = (n % 10) as u8;
                this_buffer[cursor] = b'0' + digit;
                n/=10;
                if n == 0 { break }
            }

            write_integer_fn!(@write_sign $sign, this_len, this_buffer, number);

            *this_len+=len;
            Ok(())
        }

        /// Writes a
        #[doc = $ty_name]
        /// with Debug formatting.
        pub const fn $debug_fn(&mut self, n: $ty, flags: FormattingFlags) -> Result<(), Error> {
            const fn hex(
                this: &mut StrWriterMut<'_>,
                n: $ty,
                f: FormattingFlags,
            ) -> Result<(), Error> {
                borrow_fields!(this, this_len, this_buffer);

                let is_alternate = f.is_alternate();
                let len = PWrapper(n).hexadecimal_len(f);

                let mut cursor = *this_len + len;

                if cursor > this_buffer.len() {
                    return Err(Error::NotEnoughSpace);
                }

                if is_alternate {
                    this_buffer[*this_len] = b'0';
                    this_buffer[*this_len + 1] = b'x';
                }

                write_integer_fn!(@as_unsigned $sign, n, $Unsigned);

                loop {
                    cursor-=1;
                    let digit = (n & 0b1111) as u8;
                    this_buffer[cursor] = hex_as_ascii(digit);
                    n = n >> 4;
                    if n == 0 { break }
                }

                *this_len+=len;
                Ok(())
            }

            const fn binary(
                this: &mut StrWriterMut<'_>,
                n: $ty,
                f: FormattingFlags,
            ) -> Result<(), Error> {
                borrow_fields!(this, this_len, this_buffer);

                let is_alternate = f.is_alternate();
                let len = PWrapper(n).binary_len(f);

                let mut cursor = *this_len + len;

                if cursor > this_buffer.len() {
                    return Err(Error::NotEnoughSpace);
                }

                if is_alternate {
                    this_buffer[*this_len] = b'0';
                    this_buffer[*this_len + 1] = b'b';
                }

                write_integer_fn!(@as_unsigned $sign, n, $Unsigned);

                loop {
                    cursor-=1;
                    let digit = (n & 1) as u8;
                    this_buffer[cursor] = hex_as_ascii(digit);
                    n = n >> 1;
                    if n == 0 { break }
                }

                *this_len+=len;
                Ok(())
            }

            match flags.num_fmt() {
                NumberFormatting::Decimal=>self.$display_fn(n),
                NumberFormatting::Hexadecimal=>hex(self, n, flags),
                NumberFormatting::Binary=>binary(self, n, flags),
            }
        }
    };
    (@unsigned_abs signed, $n:ident) => (
        let mut $n = $n.unsigned_abs();
    );
    (@unsigned_abs unsigned, $n:ident) => (
        let mut $n = $n.0;
    );
    (@as_unsigned signed, $n:ident, $Unsigned:ident) => (
        let mut $n = $n as $Unsigned;
    );
    (@as_unsigned unsigned, $n:ident, $Unsigned:ident) => (
        let mut $n = $n;
    );
    (@write_sign signed, $self_len:ident, $self_buffer:ident, $n:ident) => ({
        if $n < 0 {
            $self_buffer[*$self_len] = b'-';
        }
    });
    (@write_sign unsigned, $self_len:ident, $self_buffer:ident, $n:ident) => ({});
}

/// Checks that a range is valid for indexing a string,
/// assuming that the range is in-bounds, and start <= end.
#[inline]
const fn is_valid_str_range(s: &[u8], Range { start, end }: Range<usize>) -> bool {
    let len = s.len();

    (end == len || ((s[end] as i8) >= -0x40)) && (start == len || ((s[start] as i8) >= -0x40))
}

/// Checks that an index is valid for indexing a string,
/// assuming that the index is in-bounds.
#[inline]
const fn is_valid_str_index(s: &[u8], index: usize) -> bool {
    let len = s.len();

    index == len || ((s[index] as i8) >= -0x40)
}

#[inline]
pub(super) const fn saturate_range(s: &[u8], range: &Range<usize>) -> Range<usize> {
    let len = s.len();
    let end = min_usize(range.end, len);
    min_usize(range.start, end)..end
}

impl StrWriterMut<'_> {
    /// Writes a subslice of `s` with Display formatting.
    ///
    /// # Additional Errors
    ///
    /// This method returns `Error::NotOnCharBoundary` if the range is not
    /// on a character boundary.
    ///
    /// Out of bounds range bounds are treated as being at `s.len()`,
    /// this only returns an error on an in-bounds index that is not on a character boundary.
    pub const fn write_str_range(&mut self, s: &str, range: Range<usize>) -> Result<(), Error> {
        let bytes = s.as_bytes();
        let Range { start, end } = saturate_range(bytes, &range);

        if !is_valid_str_range(bytes, start..end) {
            return Err(Error::NotOnCharBoundary);
        }

        self.write_str_inner(bytes, start, end)
    }

    /// Writes `s` with Display formatting.
    pub const fn write_str(&mut self, s: &str) -> Result<(), Error> {
        let bytes = s.as_bytes();

        self.write_str_inner(bytes, 0, s.len())
    }

    /// Writes a subslice of `ascii` with Display formatting.
    ///
    /// Out of bounds range bounds are treated as being at `s.len()`.
    pub const fn write_ascii_range(
        &mut self,
        ascii: AsciiStr<'_>,
        range: Range<usize>,
    ) -> Result<(), Error> {
        let bytes = ascii.as_bytes();
        let Range { start, end } = saturate_range(bytes, &range);

        self.write_str_inner(bytes, start, end)
    }

    /// Writes `ascii` with Display formatting.
    pub const fn write_ascii(&mut self, ascii: AsciiStr<'_>) -> Result<(), Error> {
        let bytes = ascii.as_bytes();

        self.write_str_inner(bytes, 0, bytes.len())
    }

    /// Writes an ascii `character`, `repeated` times.
    pub const fn write_ascii_repeated(
        &mut self,
        mut character: u8,
        repeated: usize,
    ) -> Result<(), Error> {
        borrow_fields!(self, self_len, self_buffer);

        // Truncating non-ascii u8s
        character = character & 0b111_1111;

        let end = *self_len + repeated;

        if end > self_buffer.len() {
            return Err(Error::NotEnoughSpace);
        }

        while *self_len < end {
            self_buffer[*self_len] = character;
            *self_len += 1;
        }

        Ok(())
    }

    #[inline(always)]
    const fn write_str_inner(
        &mut self,
        bytes: &[u8],
        mut start: usize,
        end: usize,
    ) -> Result<(), Error> {
        borrow_fields!(self, self_len, self_buffer);

        let len = end - start;

        if *self_len + len > self_buffer.len() {
            return Err(Error::NotEnoughSpace);
        }

        while start < end {
            self_buffer[*self_len] = bytes[start];
            *self_len += 1;
            start += 1;
        }

        Ok(())
    }
}

/// Debug-formatted string writing
impl StrWriterMut<'_> {
    /// Writes a subslice of `s` with  Debug-like formatting.
    ///
    ///
    /// # Additional Errors
    ///
    /// This method returns `Error::NotOnCharBoundary` if the range is not
    /// on a character boundary.
    ///
    /// Out of bounds range bounds are treated as being at `s.len()`,
    /// this only returns an error on an in-bounds index that is not on a character boundary.
    pub const fn write_str_range_debug(
        &mut self,
        s: &str,
        range: Range<usize>,
    ) -> Result<(), Error> {
        let bytes = s.as_bytes();
        let Range { start, end } = saturate_range(bytes, &range);

        if !is_valid_str_range(bytes, start..end) {
            return Err(Error::NotOnCharBoundary);
        }

        self.write_str_debug_inner(bytes, start, end)
    }

    /// Writes `s` with Debug-like formatting.
    pub const fn write_str_debug(&mut self, str: &str) -> Result<(), Error> {
        let bytes = str.as_bytes();
        self.write_str_debug_inner(bytes, 0, str.len())
    }

    /// Writes a subslice of `ascii` with Debug-like formatting.
    ///
    /// Out of bounds range bounds are treated as being at `s.len()`.
    pub const fn write_ascii_range_debug(
        &mut self,
        ascii: AsciiStr<'_>,
        range: Range<usize>,
    ) -> Result<(), Error> {
        let bytes = ascii.as_bytes();
        let Range { start, end } = saturate_range(bytes, &range);

        self.write_str_debug_inner(bytes, start, end)
    }

    /// Writes `ascii` with Debug-like formatting.
    pub const fn write_ascii_debug(&mut self, ascii: AsciiStr<'_>) -> Result<(), Error> {
        let bytes = ascii.as_bytes();
        self.write_str_debug_inner(bytes, 0, bytes.len())
    }

    #[inline(always)]
    const fn write_str_debug_inner(
        &mut self,
        bytes: &[u8],
        mut start: usize,
        end: usize,
    ) -> Result<(), Error> {
        borrow_fields!(self, self_len, self_buffer);

        let len = end - start;

        // + 2 for the quote characters around the string.
        if *self_len + len + 2 > self_buffer.len() {
            return Err(Error::NotEnoughSpace);
        }

        // The amount of bytes available for escapes,
        // not counting the `writte_c`.
        let mut remaining_for_escapes = (self_buffer.len() - 2 - len - *self_len) as isize;
        let mut written = *self_len;

        self_buffer[written] = b'"';
        written += 1;

        while start != end {
            let c = bytes[start];
            let mut written_c = c;

            let mut shifted = 0;

            if c < 128
                && ({
                    shifted = 1 << c;
                    (FOR_ESCAPING.is_escaped & shifted) != 0
                })
            {
                self_buffer[written] = b'\\';
                written += 1;

                if (FOR_ESCAPING.is_backslash_escaped & shifted) != 0 {
                    remaining_for_escapes -= 1;
                    if remaining_for_escapes < 0 {
                        return Err(Error::NotEnoughSpace);
                    }
                    written_c = ForEscaping::get_backslash_escape(c);
                } else {
                    remaining_for_escapes -= 3;
                    if remaining_for_escapes < 0 {
                        return Err(Error::NotEnoughSpace);
                    }
                    self_buffer[written] = b'x';
                    written += 1;
                    self_buffer[written] = hex_as_ascii(c >> 4);
                    written += 1;
                    written_c = hex_as_ascii(c & 0xF);
                };
            }

            self_buffer[written] = written_c;
            written += 1;
            start += 1;
        }

        self_buffer[written] = b'"';
        written += 1;

        *self_len = written;

        Ok(())
    }
}

write_integer_fn! {
    (write_u8_display, write_u8_debug, unsigned, u8, u8)
    (write_u16_display, write_u16_debug, unsigned, u16, u16)
    (write_u32_display, write_u32_debug, unsigned, u32, u32)
    (write_u64_display, write_u64_debug, unsigned, u64, u64)
    (write_u128_display, write_u128_debug, unsigned, u128, u128)
    (write_usize_display, write_usize_debug, unsigned, usize, usize)

    (write_i8_display, write_i8_debug, signed, i8, u8)
    (write_i16_display, write_i16_debug, signed, i16, u16)
    (write_i32_display, write_i32_debug, signed, i32, u32)
    (write_i64_display, write_i64_debug, signed, i64, u64)
    (write_i128_display, write_i128_debug, signed, i128, u128)
    (write_isize_display, write_isize_debug, signed, isize, usize)
}
