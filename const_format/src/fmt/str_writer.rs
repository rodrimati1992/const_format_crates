use super::{Error, Formatter, FormattingFlags, StrWriterMut};

////////////////////////////////////////////////////////////////////////////////

/// A wrapper over an array usable to build up a `&str` at compile-time.
///
/// # Construction
///
/// This type is constructed with an array,
/// and then a reference to it must be coerced to point to `StrWriter<[u8]>` to call
/// [certain methods](#certain-methods)
///
/// Example of coercing it:
///
/// ```rust
/// # use const_format::StrWriter;
/// let writer: &mut StrWriter<[u8; 8]> = &mut StrWriter::new([0; 8]);
///
/// // Coerces the `&mut StrWriter<[u8; 8]>` to `&mut StrWriter<[u8]>`
/// let writer: &mut StrWriter = writer;
/// # drop(writer);
/// ```
///
/// `StrWriter`'s type parameter defaults to `[u8]`,
/// so every instance of a `StrWriter` as a concrete type is a `StrWriter<[u8]>`.
///
/// # StrWriterMut
///
/// `StrWriter` can be borrowed into a [`StrWriterMut`],
/// which provides methods for writing a formatted string..
///
/// Example:
///
/// ```rust
/// use const_format::StrWriter;
///
/// let mut buffer: &mut StrWriter = &mut StrWriter::new([0; 100]);
///
/// let mut writer = buffer.as_mut();
/// writer.write_str("Your password is: ");
/// writer.write_str_debug("PASSWORD");
///
/// assert_eq!(writer.as_str(), r#"Your password is: "PASSWORD""#);
///
/// ```
///
/// # Examples
///
/// ### Formatting into associated constant
///
/// This example shows how you can construct a formatted `&'static str` from associated constants.
///
/// ```rust
/// #![feature(const_mut_refs)]
/// #![feature(const_panic)]
///
/// use const_format::{StrWriter, strwriter_as_str, writec, unwrap};
///
/// trait Num {
///     const V: u32;
/// }
///
/// struct Two;
///
/// impl Num for Two {
///     const V: u32 = 2;
/// }
///
/// struct Three;
///
/// impl Num for Three {
///     const V: u32 = 3;
/// }
///
/// struct Mul<L, R>(L, R);
///
/// const fn compute_str(l: u32, r: u32) -> StrWriter<[u8; 128]> {
///     let mut writer = StrWriter::new([0; 128]);
///     unwrap!(writec!(writer, "{} * {} == {}", l, r, l * r ));
///     writer
/// }
///
/// impl<L: Num, R: Num> Mul<L, R> {
///     const STR: &'static str = strwriter_as_str!(&compute_str(L::V, R::V));
/// }
///
/// assert_eq!(Mul::<Two,Three>::STR, "2 * 3 == 6");
/// assert_eq!(Mul::<Three,Three>::STR, "3 * 3 == 9");
///
/// ```
///
/// [`StrWriterMut`]: ./struct.StrWriterMut.html
///
#[derive(Debug, Copy, Clone)]
pub struct StrWriter<A: ?Sized = [u8]> {
    pub(super) len: usize,
    pub(super) buffer: A,
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

impl<A: ?Sized> StrWriter<A> {
    /// How long the string in this is.
    #[inline(always)]
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Accesses the underlying buffer immutably.
    #[inline(always)]
    pub const fn buffer(&self) -> &A {
        &self.buffer
    }

    // For borrowing this mutably in macros, without getting nested mutable references.
    #[inline(always)]
    pub const fn borrow_mutably(&mut self) -> &mut Self {
        self
    }
}

/// <span id="certain-methods"></span>
impl StrWriter {
    /// Truncates this `StrWriter` to `length`.
    ///
    /// If `length` is greater than the current length, this does nothing.
    ///
    /// # Errors
    ///
    /// Returns an `Error::NotOnCharBoundary` if `length` is not on a char boundary.
    #[inline]
    pub const fn truncate(&mut self, length: usize) -> Result<(), Error> {
        self.as_mut().truncate(length)
    }

    /// Truncates this `StrWriter` to length 0.
    #[inline]
    pub const fn clear(&mut self) {
        self.len = 0;
    }

    /// Gets how the maximum length for a string written into this.
    ///
    /// Trying to write more that the capacity causes is an error,
    /// returning back an `Err(Error::NotEnoughSpace)`
    #[inline(always)]
    pub const fn capacity(&self) -> usize {
        self.buffer.len()
    }

    /// Gets the written part of this `StrWriter` as a `&[u8]`
    ///
    /// The slice is guaranteed to be valid utf8, so this is mostly for convenience.
    ///
    /// ### Constness
    ///
    /// This can be always be called in const contexts,
    ///
    /// If the "constant_time_as_str" feature is disabled,
    /// thich takes time proportional to `self.capacity() - self.len()`.
    ///
    /// If the "constant_time_as_str" feature is enabled, it takes constant time to run,
    /// but uses a few additional nightly features.
    ///
    /// # Example
    ///
    /// ```rust
    /// #![feature(const_mut_refs)]
    ///
    /// use const_format::{StrWriter, StrWriterMut};
    ///
    /// const fn slice() -> StrWriter<[u8; 64]> {
    ///     let mut buffer = StrWriter::new([0; 64]);
    ///     let mut writer = StrWriterMut::new(&mut buffer);
    ///     writer.write_str("Hello, World!");
    ///     buffer
    /// }
    ///
    /// const SLICE: &[u8] = {
    ///     let promoted: &'static StrWriter = &slice();
    ///     promoted.as_bytes_alt()
    /// };
    ///
    ///
    /// assert_eq!(SLICE, "Hello, World!".as_bytes());
    ///
    /// ```
    #[inline(always)]
    pub const fn as_bytes_alt(&self) -> &[u8] {
        crate::utils::slice_up_to_len_alt(&self.buffer, self.len)
    }

    conditionally_const! {
        feature = "constant_time_as_str";
        /// Gets the written part of this `StrWriter` as a `&str`
        ///
        /// ### Constness
        ///
        /// This can be called in const contexts by enabling the "constant_time_as_str" feature,
        /// which requires nightly Rust versions after 2020-08-15.
        ///
        /// ### Alternative
        ///
        /// For converting `&'static StrWriter` constants to `&'static str` constants,
        /// you can also use the [`strwriter_as_str`] macro.
        ///
        /// ### Examples
        ///
        /// You can look at the [type-level docs](#examples)
        /// for examples of using this method.
        ///
        /// [`strwriter_as_str`]: ../macro.strwriter_as_str.html
        #[inline(always)]
        pub fn as_str(&self) -> &str {
            // All the methods that modify the buffer must ensure utf8 validity,
            // only methods from this module need to ensure this.
            unsafe { core::str::from_utf8_unchecked(self.as_bytes()) }
        }

        /// Gets the written part of this `StrWriter` as a `&[u8]`
        ///
        /// The slice is guaranteed to be valid utf8, so this is mostly for convenience.
        ///
        /// ### Constness
        ///
        /// This can be called in const contexts by enabling the "constant_time_as_str" feature,
        /// which requires nightly Rust versions after 2020-08-15.
        ///
        /// # Example
        ///
        /// ```rust
        /// use const_format::StrWriter;
        ///
        /// let buffer: &mut StrWriter = &mut StrWriter::new([0; 64]);
        ///
        /// buffer.as_mut().write_str("Hello, World!");
        ///
        /// assert_eq!(buffer.as_bytes(), "Hello, World!".as_bytes());
        ///
        /// ```
        #[inline(always)]
        pub fn as_bytes(&self) -> &[u8] {
            crate::utils::slice_up_to_len(&self.buffer, self.len)
        }
    }

    /// Borrows this `StrWriter<[u8]>` into a `StrWriterMut`,
    /// most useful for calling the `write_*` methods.
    ///
    /// ```rust
    /// use const_format::StrWriter;
    ///
    /// let buffer: &mut StrWriter = &mut StrWriter::new([0; 64]);
    ///
    /// buffer.as_mut().write_str_range("trust", 1..usize::MAX);
    ///
    /// assert_eq!(buffer.as_str(), "rust");
    ///
    /// ```
    #[inline(always)]
    pub const fn as_mut(&mut self) -> StrWriterMut<'_> {
        StrWriterMut {
            len: &mut self.len,
            buffer: &mut self.buffer,
        }
    }

    /// Constructs a [`Formatter`] that writes into this `StrWriter`,
    /// which can be passed to debug and display formatting methods.
    ///
    /// # Example
    ///
    /// ```rust
    /// #![feature(const_mut_refs)]
    ///
    /// use const_format::{Error, Formatter, FormattingFlags, StrWriter, call_debug_fmt};
    ///
    /// use std::ops::Range;
    ///
    /// const fn range_debug_fmt(
    ///     slice: &[Range<usize>],
    ///     f: &mut Formatter<'_>
    /// ) -> Result<(), Error> {
    ///     // We need this macro to debug format arrays of non-primitive types
    ///     // Also, it implicitly returns a `const_format::Error` on error.
    ///     call_debug_fmt!(array, slice, f);
    ///     Ok(())
    /// }
    ///
    /// fn main() -> Result<(), Error> {
    ///     let buffer: &mut StrWriter = &mut StrWriter::new([0; 64]);
    ///
    ///     range_debug_fmt(
    ///         &[0..14, 14..31, 31..48],
    ///         &mut buffer.make_formatter(FormattingFlags::new().set_binary())
    ///     )?;
    ///    
    ///     assert_eq!(buffer.as_str(), "[0..1110, 1110..11111, 11111..110000]");
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// [`Formatter`]: ./struct.Formatter.html
    #[inline(always)]
    pub const fn make_formatter(&mut self, flags: FormattingFlags) -> Formatter<'_> {
        Formatter::from_sw_mut(
            StrWriterMut {
                len: &mut self.len,
                buffer: &mut self.buffer,
            },
            flags,
        )
    }
}
