use crate::{
    formatting::{hex_as_ascii, ForEscaping, FormattingFlags, FormattingMode, FOR_ESCAPING},
    utils::min_usize,
    wrapper_types::{AsciiStr, PWrapper},
};

use super::{Error, Formatter};

use core::ops::Range;

////////////////////////////////////////////////////////////////////////////////

/// A wrapper over an array used to build up a `&str` at compile-time.
///
#[derive(Debug, Copy, Clone)]
pub struct StrWriter<A: ?Sized = [u8]> {
    len: usize,
    buffer: A,
}

impl<A> StrWriter<A> {
    /// Constructs a `StrWriter` from a `u8` array
    pub const fn new(array: A) -> Self {
        Self {
            len: 0,
            buffer: array,
        }
    }
}

#[cfg(feature = "const_generics")]
impl<const N: usize> StrWriter<[u8; N]> {
    /// Constructs a `StrWriter` from a `u8` array.
    pub const fn from_array(array: [u8; N]) -> Self {
        Self {
            len: 0,
            buffer: array,
        }
    }
}

impl<A: ?Sized> StrWriter<A> {
    #[inline(always)]
    pub const fn len(&self) -> usize {
        self.len
    }

    #[inline(always)]
    pub const fn buffer(&self) -> &A {
        &self.buffer
    }

    #[inline(always)]
    pub const fn strwriter(&mut self) -> &mut Self {
        self
    }
}

macro_rules! write_integer_fn {
    (
        $(($display_fn:ident, $debug_fn:ident, $sign:ident, $ty:ident, $Unsigned:ident))*
    )=>{
        impl StrWriter{
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
                let flags = f.flags();
                f.$debug_fn(self.0, flags)
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
            let n = PWrapper(number);
            let len = n.compute_display_len(FormattingFlags::DEFAULT);

            let mut cursor = self.len + len;

            if cursor > self.buffer.len() {
                return Err(Error::NotEnoughSpace);
            }

            write_integer_fn!(@unsigned_abs $sign, n);

            loop {
                cursor-=1;
                let digit = (n % 10) as u8;
                self.buffer[cursor] = b'0' + digit;
                n/=10;
                if n == 0 { break }
            }

            write_integer_fn!(@write_sign $sign, self, number);

            self.len+=len;
            Ok(())
        }

        /// Writes a
        #[doc = $ty_name]
        /// with Debug formatting.
        pub const fn $debug_fn(&mut self, n: $ty, flags: FormattingFlags) -> Result<(), Error> {
            const fn hex(this: &mut StrWriter, n: $ty,  f: FormattingFlags) -> Result<(), Error> {
                let is_alternate = f.is_alternate();
                let len = PWrapper(n).hexadecimal_len(f);

                let mut cursor = this.len + len;

                if cursor > this.buffer.len() {
                    return Err(Error::NotEnoughSpace);
                }

                if is_alternate {
                    this.buffer[this.len] = b'0';
                    this.buffer[this.len + 1] = b'x';
                }

                write_integer_fn!(@as_unsigned $sign, n, $Unsigned);

                loop {
                    cursor-=1;
                    let digit = (n & 0b1111) as u8;
                    this.buffer[cursor] = hex_as_ascii(digit);
                    n = n >> 4;
                    if n == 0 { break }
                }

                this.len+=len;
                Ok(())
            }

            const fn binary(this: &mut StrWriter, n: $ty, f: FormattingFlags) -> Result<(), Error> {
                let is_alternate = f.is_alternate();
                let len = PWrapper(n).binary_len(f);

                let mut cursor = this.len + len;

                if cursor > this.buffer.len() {
                    return Err(Error::NotEnoughSpace);
                }

                if is_alternate {
                    this.buffer[this.len] = b'0';
                    this.buffer[this.len + 1] = b'b';
                }

                write_integer_fn!(@as_unsigned $sign, n, $Unsigned);

                loop {
                    cursor-=1;
                    let digit = (n & 1) as u8;
                    this.buffer[cursor] = hex_as_ascii(digit);
                    n = n >> 1;
                    if n == 0 { break }
                }

                this.len+=len;
                Ok(())
            }

            match flags.mode() {
                FormattingMode::Regular=>self.$display_fn(n),
                FormattingMode::Hexadecimal=>hex(self, n, flags),
                FormattingMode::Binary=>binary(self, n, flags),
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
    (@write_sign signed, $self:ident, $n:ident) => ({
        if $n < 0 {
            $self.buffer[$self.len] = b'-';
        }
    });
    (@write_sign unsigned, $self:ident, $n:ident) => ({});
}

impl StrWriter {
    pub fn as_erased(&mut self) -> &mut Self {
        self
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

impl StrWriter {
    /// Writes a subslice of `s` with Display formatting.
    ///
    /// # Additional Errors
    ///
    /// This method returns `Error::NotOnCharBoundary` if the range is not
    /// on a character boundary.
    ///
    /// Out of bounds range bounds are treated as being at `s.len()`,
    /// this only returns an error on an in-bounds index that is not on a character boundary.
    pub const fn write_str(&mut self, s: &str, range: &Range<usize>) -> Result<(), Error> {
        let bytes = s.as_bytes();
        let Range { start, end } = saturate_range(bytes, range);

        if !is_valid_str_range(bytes, start..end) {
            return Err(Error::NotOnCharBoundary);
        }

        self.write_str_inner(bytes, start, end)
    }

    /// Writes `s` with Display formatting.
    pub const fn write_whole_str(&mut self, s: &str) -> Result<(), Error> {
        let bytes = s.as_bytes();

        self.write_str_inner(bytes, 0, s.len())
    }

    /// Writes a subslice of `ascii` with Display formatting.
    ///
    /// Out of bounds range bounds are treated as being at `s.len()`.
    pub const fn write_ascii(
        &mut self,
        ascii: AsciiStr<'_>,
        range: &Range<usize>,
    ) -> Result<(), Error> {
        let bytes = ascii.as_bytes();
        let Range { start, end } = saturate_range(bytes, range);

        self.write_str_inner(bytes, start, end)
    }

    /// Writes `ascii` with Display formatting.
    pub const fn write_whole_ascii(&mut self, ascii: AsciiStr<'_>) -> Result<(), Error> {
        let bytes = ascii.as_bytes();

        self.write_str_inner(bytes, 0, bytes.len())
    }

    /// Writes an ascii `character`, `repeated` times.
    pub const fn write_ascii_repeated(
        &mut self,
        mut character: u8,
        repeated: usize,
    ) -> Result<(), Error> {
        // Truncating non-ascii u8s
        character = character & 0b111_1111;

        let end = self.len + repeated;

        if end > self.buffer.len() {
            return Err(Error::NotEnoughSpace);
        }

        while self.len < end {
            self.buffer[self.len] = character;
            self.len += 1;
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
        let len = end - start;

        if self.len + len > self.buffer.len() {
            return Err(Error::NotEnoughSpace);
        }

        while start < end {
            self.buffer[self.len] = bytes[start];
            self.len += 1;
            start += 1;
        }

        Ok(())
    }
}

/// Debug-formatted string writing
impl StrWriter {
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
    pub const fn write_str_debug(&mut self, s: &str, range: &Range<usize>) -> Result<(), Error> {
        let bytes = s.as_bytes();
        let Range { start, end } = saturate_range(bytes, range);

        if !is_valid_str_range(bytes, start..end) {
            return Err(Error::NotOnCharBoundary);
        }

        self.write_str_debug_inner(bytes, start, end)
    }

    /// Writes `s` with Debug-like formatting.
    pub const fn write_whole_str_debug(&mut self, str: &str) -> Result<(), Error> {
        let bytes = str.as_bytes();
        self.write_str_debug_inner(bytes, 0, str.len())
    }

    /// Writes a subslice of `ascii` with Debug-like formatting.
    ///
    /// Out of bounds range bounds are treated as being at `s.len()`.
    pub const fn write_ascii_debug(
        &mut self,
        ascii: AsciiStr<'_>,
        range: &Range<usize>,
    ) -> Result<(), Error> {
        let bytes = ascii.as_bytes();
        let Range { start, end } = saturate_range(bytes, range);

        self.write_str_debug_inner(bytes, start, end)
    }

    /// Writes `ascii` with Debug-like formatting.
    pub const fn write_whole_ascii_debug(&mut self, ascii: AsciiStr<'_>) -> Result<(), Error> {
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
        let len = end - start;

        // + 2 for the quote characters around the string.
        if self.len + len + 2 > self.buffer.len() {
            return Err(Error::NotEnoughSpace);
        }

        // The amount of bytes available for escapes,
        // not counting the `writte_c`.
        let mut remaining_for_escapes = (self.buffer.len() - 2 - len - self.len()) as isize;
        let mut written = self.len;

        self.buffer[written] = b'"';
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
                self.buffer[written] = b'\\';
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
                    self.buffer[written] = b'x';
                    written += 1;
                    self.buffer[written] = hex_as_ascii(c >> 4);
                    written += 1;
                    written_c = hex_as_ascii(c & 0xF);
                };
            }

            self.buffer[written] = written_c;
            written += 1;
            start += 1;
        }

        self.buffer[written] = b'"';
        written += 1;

        self.len = written;

        Ok(())
    }
}

impl StrWriter {
    #[inline(always)]
    pub const fn capacity(&self) -> usize {
        self.buffer.len()
    }

    #[inline(always)]
    pub fn as_str(&self) -> &str {
        // All the methods that modify the buffer must ensure utf8 validity,
        // only methods from this module need to ensure this.
        unsafe { core::str::from_utf8_unchecked(self.as_bytes()) }
    }

    #[inline(always)]
    pub fn as_bytes(&self) -> &[u8] {
        &self.buffer[..self.len]
    }

    // For borrowing this mutably in macros, without getting nested mutable references.
    #[inline(always)]
    pub const fn borrow_mutably(&mut self) -> &mut Self {
        self
    }

    #[inline(always)]
    pub const fn make_formatter(&mut self, flags: FormattingFlags) -> Formatter<'_> {
        Formatter::with_strwriter(flags, self)
    }

    /// Truncates this `StrWriter` to `length`.
    ///
    /// If `length` is greater than the current length, this does nothing.
    ///
    /// # Errors
    ///
    /// Returns an `Error::NotOnCharBoundary` if `length` is not on a char boundary.
    #[inline]
    pub const fn truncate(&mut self, length: usize) -> Result<(), Error> {
        if length <= self.len {
            if !is_valid_str_index(&self.buffer, length) {
                return Err(Error::NotOnCharBoundary);
            }

            self.len = length;
        }
        Ok(())
    }

    /// Truncates this `StrWriter` to length 0.
    #[inline]
    pub const fn clear(&mut self) {
        self.len = 0;
    }
}

////////////////////////////////////////////////////////////////////////////////
