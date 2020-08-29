use super::{Error, Formatter, FormattingFlags, StrWriterMut};

////////////////////////////////////////////////////////////////////////////////

/// A wrapper over an array used to build up a `&str` at compile-time.
///
/// # Examples
///
/// ### At Runtime
///
/// This example shows how you can construct the str at compile-time,
/// and do the conversion from `&'static StrWriter` to `&'static str` at runtime.
///
/// ```rust
/// #![feature(const_mut_refs)]
///
/// use const_format::{StrWriter, writec, unwrap};
///
/// trait Str {
///     const V: &'static str;
/// }
///
/// struct Hello;
///
/// impl Str for Hello {
///     const V: &'static str = "hello";
/// }
///
/// struct World;
///
/// impl Str for World {
///     const V: &'static str = "world";
/// }
///
/// struct Add<L, R>(L, R);
///
/// const fn compute_str(l: &str, r: &str) -> StrWriter<[u8; 128]> {
///     let mut writer = StrWriter::new([0; 128]);
///     unwrap!(writec!(writer, "{}{}", l, r));
///     writer
/// }
///
/// impl<L: Str, R: Str> Add<L, R> {
///     const STR: &'static StrWriter = &compute_str(L::V, R::V);
/// }
///
/// assert_eq!(Add::<Hello, Hello>::STR.as_str(), "hellohello");
/// assert_eq!(Add::<Hello, World>::STR.as_str(), "helloworld");
/// assert_eq!(Add::<World, Hello>::STR.as_str(), "worldhello");
/// assert_eq!(Add::<World, World>::STR.as_str(), "worldworld");
///
/// ```
///
/// ### At compile time
///
/// This example shows how you can construct the string at compile-time,
/// *and* also do the conversion from `&'static StrWriter` to `&'static str`
/// at compile-time.
///
/// This requires the "const_as_str" feature,which uses additional nightly Rust features.
///
/// ```rust
/// #![feature(const_mut_refs)]
/// #![feature(const_panic)]
///
/// use const_format::{StrWriter, writec, unwrap};
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
///     const STR: &'static str = {
///         // We have to coerce `&StrWriter<[u8; 128]>` to `&StrWriter` to call the
///         // `as_str` method.
///         let w: &StrWriter = &compute_str(L::V, R::V);
///         w.as_str()
///     };
/// }
///
/// assert_eq!(Mul::<Two,Three>::STR, "2 * 3 == 6");
/// assert_eq!(Mul::<Three,Three>::STR, "3 * 3 == 9");
///
/// ```
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
    #[inline(always)]
    pub const fn len(&self) -> usize {
        self.len
    }

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

impl StrWriter {
    #[inline]
    pub const fn truncate(&mut self, length: usize) -> Result<(), Error> {
        self.as_mut().truncate(length)
    }

    /// Truncates this `StrWriterMut` to length 0.
    #[inline]
    pub const fn clear(&mut self) {
        self.len = 0;
    }

    #[inline(always)]
    pub const fn capacity(&self) -> usize {
        self.buffer.len()
    }

    conditionally_const! {
        feature = "const_as_str";
        /// Gets the written part of this StrWriter as a `&str`
        ///
        /// This can be called in const contexts by enabling the "const_as_str" feature,
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

        /// Gets the written part of this StrWriterMut as a `&[u8]`
        ///
        /// The slice is guaranteed to be valid utf8, so this is mostly for convenience.
        ///
        /// This can be called in const contexts by enabling the "const_as_str" feature,
        /// which requires nightly Rust versions after 2020-08-15.
        ///
        #[inline(always)]
        pub fn as_bytes(&self) -> &[u8] {
            crate::utils::slice_up_to_len(&self.buffer, self.len)
        }
    }

    #[inline(always)]
    pub const fn as_mut(&mut self) -> StrWriterMut<'_> {
        StrWriterMut {
            len: &mut self.len,
            buffer: &mut self.buffer,
        }
    }

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
