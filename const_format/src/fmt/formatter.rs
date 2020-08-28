use crate::{
    fmt::{str_writer_mut::saturate_range, Error, FormattingFlags, StrWriter, StrWriterMut},
    wrapper_types::{AsciiStr, PWrapper},
};

use core::ops::Range;

////////////////////////////////////////////////////////////////////////////////

/// For computing how long a formatted string would be.
///
/// This is what the [`formatc`] macro uses to precalculate the length of its returned `&str`.
///
/// # Example
///
/// ```rust
/// #![feature(const_mut_refs)]
///
/// use const_format::fmt::{ComputeStrLength, Error, Formatter, FormattingFlags, StrWriter};
/// use const_format::{try_, writec, unwrap};
///
/// const fn write_sum(mut f: Formatter<'_>) -> Result<(), Error> {
///     let l = 7u8;
///     let r = 8u8;
///     writec!(f, "{} + {} = {}", l, r, l + r)
/// }
///
/// // This is a const fn because mutable references can't be used in const initializers.
/// const fn len() -> usize {
///     let mut computer = ComputeStrLength::new();
///     unwrap!(write_sum(computer.make_formatter(FormattingFlags::NEW)));
///     computer.len()
/// }
/// const LEN: usize = len();
///
/// // The `&mut StrWriter` type here wraps a `[u8]` slice,
/// // coercing the assigned value from a `&mut StrWriter<[u8; LEN]>`.
/// let writer: &mut StrWriter = &mut StrWriter::new([0; LEN]);
///
/// write_sum(writer.make_formatter(FormattingFlags::NEW)).unwrap();
///
/// assert_eq!(writer.as_str(), "7 + 8 = 15");
/// assert_eq!(writer.len(), LEN);
/// assert_eq!(writer.capacity(), LEN);
///
/// ```
///
/// [`formatc`]: ../macro.formatc.html
///
///
pub struct ComputeStrLength {
    len: usize,
}

impl ComputeStrLength {
    /// Constructs a ComputeStrLength of length 0.
    pub const fn new() -> Self {
        Self { len: 0 }
    }

    /// Constructs a `Formatter`,
    /// which instead of writing to a buffer it adds the computed length into this.
    pub const fn make_formatter(&mut self, flags: FormattingFlags) -> Formatter<'_> {
        Formatter {
            flags,
            writer: WriterBackend::Length(self),
        }
    }

    /// Adds `len` to the calculated length.
    pub const fn add_len(&mut self, len: usize) {
        self.len += len;
    }

    /// The length of the string when formatted.
    pub const fn len(&self) -> usize {
        self.len
    }

    // For borrowing this mutably in macros, without getting nested mutable references.
    #[inline(always)]
    pub const fn borrow_mutably(&mut self) -> &mut Self {
        self
    }
}

////////////////////////////////////////////////////////////////////////////////

enum WriterBackend<'w> {
    Str(StrWriterMut<'w>),
    Length(&'w mut ComputeStrLength),
}

////////////////////////////////////////////////////////////////////////////////

/// A type through which you can write formatted text.
///
/// # FormattingFlags
///
/// Types can change how they're formatted based on the value of the
/// [`FormattingFlags`]`returned by `.flags()`,
/// for more details on that you can read the documentation for [`FormattingFlags`].
///
/// # Examples
///
/// ### Debug and Display formatting
///
/// This example demonstrates how you can do display and/or debug-like formatting
/// with a Formatter:
///
/// ```rust
/// #![feature(const_mut_refs)]
///
/// use const_format::{Error, Formatter, FormattingFlags, StrWriter};
/// use const_format::{impl_fmt, try_};
///
/// struct Foo;
///
/// impl_fmt!{
///     impl[] Foo;
///     
///     const fn const_display_fmt(&self, mut f: Formatter<'_>) -> Result<(), Error> {
///         let string = "foo bar baz";
///         try_!(f.write_u8_display(100));
///         try_!(f.write_str(" "));
///         try_!(f.write_str_range(string, 4..7));
///         try_!(f.write_str("\n\n\n...figters"));
///         Ok(())
///     }
///     
///     const fn const_debug_fmt(&self, mut f: Formatter<'_>) -> Result<(), Error> {
///         let string = "foo bar baz";
///         try_!(f.write_u8_debug(100));
///         try_!(f.write_str_range_debug(string, 8..usize::MAX));
///         try_!(f.write_str_debug("\n\n\n...figters"));
///         Ok(())
///     }
/// }
///
///
///
/// // We have to coerce `&mut StrWriter<[u8; 128]>` to `&mut StrWriter` to call the
/// // `make_formatter` method.
/// let writer: &mut StrWriter = &mut StrWriter::new([0; 256]);
///
/// let flags = FormattingFlags::NEW.set_binary();
///
/// // The Display formatters from this crate don't care which NumberFormatting you pass,
/// // they'll just write integers as decimal.
/// Foo.const_display_fmt(writer.make_formatter(flags));
///
/// assert_eq!(writer.as_str(), "100 bar\n\n\n...figters");
///
/// writer.clear();
///
/// Foo.const_debug_fmt(writer.make_formatter(flags)).unwrap();
/// // Another way to write the above
/// // writec!(writer, "{:b?}", Foo).unwrap();
///
/// assert_eq!(writer.as_str(), "1100100\"baz\"\"\\n\\n\\n...figters\"");
///
/// ```
///
/// ### Writing to an array
///
/// This example demonstrates how you can use a Formatter to write to a byte slice.
///
/// You can use the unsafe [`from_custom`] constructor if you need to start writing from
/// anywhere other than 0.
///
/// ```rust
/// #![feature(const_mut_refs)]
///
/// use const_format::{Error, Formatter, FormattingFlags, StrWriter};
/// use const_format::{impl_fmt, try_, writec};
///
/// const fn write_int(int: u32, buffer: &mut [u8]) -> Result<usize, Error> {
///     let mut len = 0;
///     let mut f = Formatter::from_custom_cleared(FormattingFlags::NEW, &mut len, buffer);
///     try_!(writec!(f, "{0},{0:x},{0:b}", int));
///     Ok(len)
/// }
///
/// let mut buffer = [0;64];
///
/// let written = write_int(17, &mut buffer).unwrap();
///
/// let string = std::str::from_utf8(&buffer[..written])
///     .expect("Formatter only writes valid UTF8");
///
/// assert_eq!(string, "17,11,10001");
///
/// ```
///
///
/// [`from_custom`]: #method.from_constructor
/// [`NumberFormatting`]: ./enum.NumberFormatting.html
/// [`FormattingFlags`]: ./struct.FormattingFlags.html
///
pub struct Formatter<'w> {
    flags: FormattingFlags,
    writer: WriterBackend<'w>,
}

impl<'w> Formatter<'w> {
    /// Constructs a `Formatter`.
    #[inline]
    pub const fn from_sw(flags: FormattingFlags, writer: &'w mut StrWriter) -> Self {
        Self {
            flags,
            writer: WriterBackend::Str(writer.as_mut()),
        }
    }

    /// Constructs a `Formatter`.
    #[inline]
    pub const fn from_sw_mut(flags: FormattingFlags, writer: StrWriterMut<'w>) -> Self {
        Self {
            flags,
            writer: WriterBackend::Str(writer),
        }
    }

    /// Construct a `Formatter` from a byte slice.
    ///
    /// # Safety
    ///
    /// The bytes up to (and excluding) `length` in `buffer` must be valid utf8.
    ///
    /// # Example
    ///
    /// This example demonstrates how you can use a Formatter to write to a byte slice
    /// that had some text already written to it already.
    ///
    /// ```rust
    /// #![feature(const_mut_refs)]
    ///
    /// use const_format::{Error, Formatter, FormattingFlags, StrWriter};
    /// use const_format::{impl_fmt, try_, writec};
    ///
    /// ///
    /// /// # Safety
    /// ///
    /// /// `&buffer[..start]` must be valid utf8.
    /// const unsafe fn write_int(
    ///     int: u32,
    ///     buffer: &mut [u8],
    ///     start: usize,
    /// ) -> Result<usize, Error> {
    ///     let mut len = start;
    ///     let mut f = Formatter::from_custom(FormattingFlags::NEW, &mut len, buffer);
    ///     try_!(writec!(f, "{0},{0:x},{0:b}", int));
    ///     Ok(len)
    /// }
    ///
    /// let start_str = "The number is ";
    /// let mut buffer = [0;64];
    /// buffer[..start_str.len()].copy_from_slice(start_str.as_bytes());
    ///
    /// // Safety: The buffer is entirely ascii, so any index is safe.
    /// let new_len = unsafe{ write_int(20, &mut buffer, start_str.len()).unwrap() };
    ///
    /// let string = std::str::from_utf8(&buffer[..new_len])
    ///     .expect("Formatter only writes valid UTF8");
    ///
    /// assert_eq!(string, "The number is 20,14,10100");
    ///
    /// ```
    #[inline]
    pub const unsafe fn from_custom(
        flags: FormattingFlags,
        length: &'w mut usize,
        buffer: &'w mut [u8],
    ) -> Self {
        Self {
            flags,
            writer: WriterBackend::Str(StrWriterMut::from_custom(length, buffer)),
        }
    }

    /// Construct a `Formatter`from a byte slice.
    #[inline]
    pub const fn from_custom_cleared(
        flags: FormattingFlags,
        length: &'w mut usize,
        buffer: &'w mut [u8],
    ) -> Self {
        Self {
            flags,
            writer: WriterBackend::Str(StrWriterMut::from_custom_cleared(length, buffer)),
        }
    }

    /// Gets the formatting flags associated with this `Formatter`.
    #[inline(always)]
    pub const fn flags(&self) -> FormattingFlags {
        self.flags
    }

    #[inline(always)]
    const fn increment_margin(&mut self) -> &mut Self {
        self.flags = self.flags.increment_margin();
        self
    }

    #[inline(always)]
    const fn decrement_margin(&mut self) {
        self.flags = self.flags.decrement_margin();
    }
}

impl<'w> Formatter<'w> {
    // For borrowing this mutably in macros, without getting nested mutable references.
    #[inline(always)]
    pub const fn borrow_mutably(&mut self) -> &mut Self {
        self
    }

    /// Constructs a reborrow of this formatter, using `flags` as the formatting flags.
    ///
    /// The return value inherits the margin from this Formatter.
    ///
    /// This method exists because the [`writec`] macro gets a formatter from any writer
    /// by calling a `make_formatter` method.
    ///
    /// # Example
    ///
    /// This example demonstrates how you can change the flags when printing a field..
    ///
    /// ```rust
    /// #![feature(const_mut_refs)]
    ///
    /// use const_format::{Error, Formatter, PWrapper};
    /// use const_format::{coerce_to_fmt, formatc, impl_fmt, try_};
    ///
    /// use std::ops::RangeInclusive;
    ///
    /// struct Foo{
    ///     x: u32,
    ///     y: RangeInclusive<usize>,
    ///     z: u32,
    /// }
    ///
    /// impl_fmt!{
    ///     impl Foo;
    ///
    ///     pub const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    ///         let mut f = f.debug_struct("Foo");
    ///         try_!(PWrapper(self.x).const_debug_fmt(f.field("x")));
    ///         
    ///         let mut fmt_y = f.field("y");
    ///         let flags = fmt_y.flags().set_binary();
    ///         try_!(coerce_to_fmt!(&self.y).const_debug_fmt(&mut fmt_y.make_formatter(flags)));
    ///
    ///         try_!(PWrapper(self.z).const_debug_fmt(f.field("z")));
    ///         f.finish()
    ///     }
    /// }
    ///
    /// const FOO: Foo = Foo {
    ///     x: 15,
    ///     y: 16..=31,
    ///     z: 32,
    /// };
    /// const S: &str = formatc!("{FOO:#?}");
    ///
    /// const EXPECTED: &str = "\
    /// Foo {
    ///     x: 15,
    ///     y: 0b10000..=0b11111,
    ///     z: 32,
    /// }\
    /// ";
    ///
    /// assert_eq!(S, EXPECTED);
    /// ```
    ///
    /// [`writec`]: ../macro.writec.html
    ///
    pub const fn make_formatter(&mut self, flags: FormattingFlags) -> Formatter<'_> {
        let flags = flags.copy_margin_of(self.flags);
        match &mut self.writer {
            WriterBackend::Str(x) => x.make_formatter(flags),
            WriterBackend::Length(x) => x.make_formatter(flags),
        }
    }

    #[inline]
    pub const fn debug_struct(&mut self, name: &str) -> DebugStruct<'_, 'w> {
        let err = self.write_str(name);
        DebugStruct {
            fmt: self.increment_margin(),
            wrote_field: false,
            err,
        }
    }

    #[inline]
    pub const fn debug_tuple(&mut self, name: &str) -> DebugTuple<'_, 'w> {
        let err = self.write_str(name);
        DebugTuple {
            fmt: self.increment_margin(),
            wrote_field: false,
            err,
        }
    }

    #[inline]
    pub const fn debug_list(&mut self) -> DebugList<'_, 'w> {
        DebugList {
            fmt: self.increment_margin(),
            wrote_field: false,
            err: Ok(()),
        }
    }

    #[inline]
    pub const fn debug_set(&mut self) -> DebugSet<'_, 'w> {
        DebugSet {
            fmt: self.increment_margin(),
            wrote_field: false,
            err: Ok(()),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

macro_rules! trys {
    ($e:expr,$self:ident) => {
        if let result @ Err(_) = $e {
            $self.err = result;
        }
    };
}

const COLON_SPACE_LEN: usize = ": ".len();
const COMMA_SPACE_LEN: usize = ", ".len();
const COMMA_NL_LEN: usize = ",\n".len();

macro_rules! field_method_impl {
    ($
        self: ident, $open_space:expr, $open_newline:expr;
        len(|$fmt_len:ident| $($write_name_len:tt)*)
        fmt(|$writer:ident| $($write_name_fmt:tt)*)
    ) => ({
        match &mut $self.fmt.writer {
            WriterBackend::Length($fmt_len)=>{
                let $fmt_len = &mut **$fmt_len;

                const OPEN_SPACE: usize = $open_space.len();
                const OPEN_NEWLINE: usize = $open_newline.len();

                let is_alternate = $self.fmt.flags.is_alternate();
                $fmt_len.add_len(match ($self.wrote_field, is_alternate) {
                    (false, false) => OPEN_SPACE,
                    (false, true) => OPEN_NEWLINE + $self.fmt.flags.margin(),
                    (true , false) => COMMA_SPACE_LEN,
                    (true , true) => COMMA_NL_LEN + $self.fmt.flags.margin(),
                });
                $($write_name_len)*
            }
            WriterBackend::Str($writer)=>{
                let $writer = &mut *$writer;

                let is_alternate = $self.fmt.flags.is_alternate();
                let sep = match ($self.wrote_field, is_alternate) {
                    (false, false)=>$open_space,
                    (false, true)=>$open_newline,
                    (true, false)=>", ",
                    (true, true)=>",\n",
                };
                trys!($writer.write_str(sep), $self);
                if is_alternate {
                    trys!($writer.write_ascii_repeated(b' ', $self.fmt.flags.margin()), $self);
                }
                $($write_name_fmt)*
            }
        }
        $self.wrote_field = true;

        $self.fmt
    })
}

macro_rules! finish_method_impl {
    ($self: ident, $close_token:expr, $space_close:expr) => {{
        if let result @ Err(_) = $self.err {
            return result;
        }

        $self.fmt.decrement_margin();
        if $self.wrote_field {
            match &mut $self.fmt.writer {
                WriterBackend::Length(fmt_len) => {
                    let fmt_len = &mut **fmt_len;

                    const CLOSE_TOKEN: usize = $close_token.len();
                    const SPACE_CLOSE: usize = $space_close.len();

                    if $self.fmt.flags.is_alternate() {
                        fmt_len.add_len(COMMA_NL_LEN + $self.fmt.flags.margin() + CLOSE_TOKEN);
                    } else {
                        fmt_len.add_len(SPACE_CLOSE);
                    }
                    Ok(())
                }
                WriterBackend::Str(writer) => {
                    let writer = &mut *writer;

                    if $self.fmt.flags.is_alternate() {
                        try_!(writer.write_str(",\n"));
                        try_!(writer.write_ascii_repeated(b' ', $self.fmt.flags.margin()));
                        writer.write_str($close_token)
                    } else {
                        writer.write_str($space_close)
                    }
                }
            }
        } else {
            Ok(())
        }
    }};
}

////////////////////////////////////////////////////////////////////////////////

pub struct DebugStruct<'f, 'w> {
    fmt: &'f mut Formatter<'w>,
    wrote_field: bool,
    err: Result<(), Error>,
}

impl<'f, 'w> DebugStruct<'f, 'w> {
    pub const fn field(&mut self, name: &str) -> &mut Formatter<'w> {
        field_method_impl!(
            self, " { ", " {\n";
            len(|fmt_len|
                fmt_len.add_len(name.len() + COLON_SPACE_LEN);
            )
            fmt(|writer|
                trys!(writer.write_str(name), self);
                trys!(writer.write_str(": "), self);
            )
        )
    }

    pub const fn finish(self) -> Result<(), Error> {
        finish_method_impl!(self, "}", " }")
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct DebugTuple<'f, 'w> {
    fmt: &'f mut Formatter<'w>,
    wrote_field: bool,
    err: Result<(), Error>,
}

impl<'f, 'w> DebugTuple<'f, 'w> {
    pub const fn field(&mut self) -> &mut Formatter<'w> {
        field_method_impl!(self, "(", "(\n"; len(|fmt_len|) fmt(|writer|) )
    }

    pub const fn finish(self) -> Result<(), Error> {
        finish_method_impl!(self, ")", ")")
    }
}

////////////////////////////////////////////////////////////////////////////////

macro_rules! finish_listset_method_impl {
    ($self: ident, $close_token:expr, $open_close:expr) => {{
        if let result @ Err(_) = $self.err {
            return result;
        }

        match &mut $self.fmt.writer {
            WriterBackend::Length(fmt_len) => {
                let fmt_len = &mut **fmt_len;
                const CLOSE_TOKEN: usize = $close_token.len();
                const OPEN_CLOSE: usize = $open_close.len();

                $self.fmt.flags = $self.fmt.flags.decrement_margin();
                if $self.wrote_field {
                    if $self.fmt.flags.is_alternate() {
                        fmt_len.add_len(COMMA_NL_LEN + $self.fmt.flags.margin());
                    }
                    fmt_len.add_len(CLOSE_TOKEN);
                } else {
                    fmt_len.add_len(OPEN_CLOSE);
                }
                Ok(())
            }
            WriterBackend::Str(writer) => {
                let writer = &mut *writer;

                $self.fmt.flags = $self.fmt.flags.decrement_margin();
                let margin = $self.fmt.flags.margin();
                if $self.wrote_field {
                    if $self.fmt.flags.is_alternate() {
                        try_!(writer.write_str(",\n"));
                        try_!(writer.write_ascii_repeated(b' ', margin));
                    }
                    writer.write_str($close_token)
                } else {
                    writer.write_str($open_close)
                }
            }
        }
    }};
}

////////////////////////////////////////////////////////////////////////////////

pub struct DebugList<'f, 'w> {
    fmt: &'f mut Formatter<'w>,
    wrote_field: bool,
    err: Result<(), Error>,
}

impl<'f, 'w> DebugList<'f, 'w> {
    pub const fn entry(&mut self) -> &mut Formatter<'w> {
        field_method_impl!(self, "[", "[\n"; len(|fmt_len|) fmt(|writer|) )
    }

    pub const fn finish(self) -> Result<(), Error> {
        finish_listset_method_impl!(self, "]", "[]")
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct DebugSet<'f, 'w> {
    fmt: &'f mut Formatter<'w>,
    wrote_field: bool,
    err: Result<(), Error>,
}

impl<'f, 'w> DebugSet<'f, 'w> {
    pub const fn entry(&mut self) -> &mut Formatter<'w> {
        field_method_impl!(self, "{", "{\n"; len(|fmt_len|) fmt(|writer|) )
    }

    pub const fn finish(self) -> Result<(), Error> {
        finish_listset_method_impl!(self, "}", "{}")
    }
}

////////////////////////////////////////////////////////////////////////////////

macro_rules! delegate_write_methods {
    (
        $(
            $(#[$attrs:meta])*
            fn $method:ident($($arg:ident: $arg_ty:ty ),* $(,)* )
            length = $len:expr;
        )*
    ) => (
        $(
            impl Formatter<'_>{
                $(#[$attrs:meta])*
                #[inline(always)]
                pub const fn $method(&mut self, $($arg: $arg_ty ),*  ) -> Result<(), Error> {
                    match &mut self.writer {
                        WriterBackend::Length(fmt_len)=>{
                            fmt_len.add_len($len);
                            Ok(())
                        }
                        WriterBackend::Str(writer)=>{
                            writer.$method($($arg,)*)
                        }
                    }
                }
            }

        )*
    )
}

delegate_write_methods! {

    fn write_str_range(s: &str, range: Range<usize>)
    length = calculate_display_len(s.as_bytes(), &range);

    fn write_str(s: &str)
    length = s.len();

    fn write_ascii_range(ascii: AsciiStr<'_>, range: Range<usize>)
    length = calculate_display_len(ascii.as_bytes(), &range);

    fn write_ascii( ascii: AsciiStr<'_>)
    length = ascii.len();

    fn write_ascii_repeated(character: u8,repeated: usize)
    length = repeated;

    fn write_str_range_debug(s: &str, range: Range<usize>)
    length = calculate_display_len_debug_range(s.as_bytes(), &range);

    fn write_str_debug(str: &str)
    length = PWrapper(str.as_bytes()).compute_utf8_debug_len();

    fn write_ascii_range_debug(ascii: AsciiStr<'_>,range: Range<usize>)
    length = calculate_display_len_debug_range(ascii.as_bytes(), &range);

    fn write_ascii_debug(ascii: AsciiStr<'_>)
    length = PWrapper(ascii.as_bytes()).compute_utf8_debug_len();


    fn write_u8_display(n: u8)
    length = PWrapper(n).compute_display_len(FormattingFlags::NEW);

    fn write_u16_display(n: u16)
    length = PWrapper(n).compute_display_len(FormattingFlags::NEW);

    fn write_u32_display(n: u32)
    length = PWrapper(n).compute_display_len(FormattingFlags::NEW);

    fn write_u64_display(n: u64)
    length = PWrapper(n).compute_display_len(FormattingFlags::NEW);

    fn write_u128_display(n: u128)
    length = PWrapper(n).compute_display_len(FormattingFlags::NEW);

    fn write_usize_display(n: usize)
    length = PWrapper(n).compute_display_len(FormattingFlags::NEW);

    fn write_i8_display(n: i8)
    length = PWrapper(n).compute_display_len(FormattingFlags::NEW);

    fn write_i16_display(n: i16)
    length = PWrapper(n).compute_display_len(FormattingFlags::NEW);

    fn write_i32_display(n: i32)
    length = PWrapper(n).compute_display_len(FormattingFlags::NEW);

    fn write_i64_display(n: i64)
    length = PWrapper(n).compute_display_len(FormattingFlags::NEW);

    fn write_i128_display(n: i128)
    length = PWrapper(n).compute_display_len(FormattingFlags::NEW);

    fn write_isize_display(n: isize)
    length = PWrapper(n).compute_display_len(FormattingFlags::NEW);
}

macro_rules! delegate_integer_debug_methods {
    (
        $(
            $(#[$attrs:meta])*
            fn $method:ident($($arg:ident: $arg_ty:ty ),* $(,)* )
            length = |$flags:ident| $len:expr;
        )*
    ) => (
        $(
            impl Formatter<'_>{
                $(#[$attrs:meta])*
                #[inline(always)]
                pub const fn $method(&mut self, $($arg: $arg_ty ),*  ) -> Result<(), Error> {
                    let $flags = self.flags;

                    match &mut self.writer {
                        WriterBackend::Length(fmt_len)=>{
                            fmt_len.add_len($len);
                            Ok(())
                        }
                        WriterBackend::Str(writer)=>{
                            writer.$method($($arg,)* $flags)
                        }
                    }
                }
            }

        )*
    )
}

delegate_integer_debug_methods! {
    fn write_u8_debug(n: u8)
    length = |flags| PWrapper(n).compute_debug_len(flags);

    fn write_u16_debug(n: u16)
    length = |flags| PWrapper(n).compute_debug_len(flags);

    fn write_u32_debug(n: u32)
    length = |flags| PWrapper(n).compute_debug_len(flags);

    fn write_u64_debug(n: u64)
    length = |flags| PWrapper(n).compute_debug_len(flags);

    fn write_u128_debug(n: u128)
    length = |flags| PWrapper(n).compute_debug_len(flags);

    fn write_usize_debug(n: usize)
    length = |flags| PWrapper(n).compute_debug_len(flags);

    fn write_i8_debug(n: i8)
    length = |flags| PWrapper(n).compute_debug_len(flags);

    fn write_i16_debug(n: i16)
    length = |flags| PWrapper(n).compute_debug_len(flags);

    fn write_i32_debug(n: i32)
    length = |flags| PWrapper(n).compute_debug_len(flags);

    fn write_i64_debug(n: i64)
    length = |flags| PWrapper(n).compute_debug_len(flags);

    fn write_i128_debug(n: i128)
    length = |flags| PWrapper(n).compute_debug_len(flags);

    fn write_isize_debug(n: isize)
    length = |flags| PWrapper(n).compute_debug_len(flags);
}

#[inline(always)]
const fn calculate_display_len(b: &[u8], range: &Range<usize>) -> usize {
    let Range { start, end } = saturate_range(b, range);
    end - start
}

#[inline(always)]
const fn calculate_display_len_debug_range(b: &[u8], range: &Range<usize>) -> usize {
    let Range { start, end } = saturate_range(b, range);
    PWrapper(b).compute_utf8_debug_len_in_range(start..end)
}
