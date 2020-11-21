#[macro_use]
#[cfg(feature = "assert")]
mod assertions;

#[macro_use]
#[cfg(feature = "fmt")]
mod call_debug_fmt;

#[macro_use]
mod constructors;

#[macro_use]
mod helper_macros;

#[macro_use]
mod fmt_macros;

#[macro_use]
#[cfg(feature = "fmt")]
mod impl_fmt;

/// For returning early on an error, otherwise evaluating to `()`.
///
/// # Example
///
/// ```rust
/// #![feature(const_mut_refs)]
///
/// use const_format::{Error, StrWriter};
/// use const_format::{try_, writec};
///
/// const fn write_stuff(buffer: &mut StrWriter) -> Result<&[u8], Error> {
///     try_!(writec!(buffer, "Foo{},Bar{},Baz{},", 8u32, 13u32, 21u32));
///     Ok(buffer.as_bytes_alt())
/// }
///
/// let mut buffer = StrWriter::new([0; 32]);
/// assert_eq!(write_stuff(&mut buffer)?, "Foo8,Bar13,Baz21,".as_bytes());
///
/// # Ok::<(), Error>(())
/// ```
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "fmt")))]
#[cfg(feature = "fmt")]
#[macro_export]
macro_rules! try_ {
    ($e:expr) => {
        if let $crate::pmr::Err(e) = $e {
            return $crate::pmr::Err(e);
        }
    };
}

/// Equivalent to `Result::unwrap`, for use with [`const_format::Error`] errors.
///
/// You can use this when you know for certain that no error will happen.
///
/// [`const_format::Error`]: ./fmt/enum.Error.html
///
/// # Example
///
/// ```rust
/// #![feature(const_mut_refs)]
///
/// use const_format::{StrWriter, strwriter_as_str, unwrap, writec};
///
/// const CAP: usize = 11;
/// // Defined this function as a workaround for mutable references not
/// // being allowed in `const`ants.
/// const fn foo() -> StrWriter<[u8; CAP]> {
///     let mut writer = StrWriter::new([0; CAP]);
///     unwrap!(writec!(writer, "foo bar baz"));
///     writer
/// }
///
/// const TEXT: &str = strwriter_as_str!(&foo());
/// assert_eq!(TEXT, "foo bar baz")
///
/// ```
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "fmt")))]
#[cfg(feature = "fmt")]
#[macro_export]
macro_rules! unwrap {
    ($e:expr $(,)*) => {
        match $e {
            $crate::pmr::Ok(x) => x,
            $crate::pmr::Err(error) => $crate::Error::unwrap(&error),
        }
    };
}

/// Equivalent to `Result::unwrap_or_else` but allows returning from the enclosing function.
///
/// # Examples
///
/// ### Early return
///
/// ```rust
/// #![feature(const_mut_refs)]
///
/// use const_format::unwrap_or_else;
///
/// const fn unwrap_square(number: Result<u32, u32>) -> u64 {
///     let n = unwrap_or_else!(number, |n| return n as u64 ) as u64;
///     n * n
/// }
///
/// assert_eq!(unwrap_square(Ok(10)), 100);
/// assert_eq!(unwrap_square(Ok(30)), 900);
/// assert_eq!(unwrap_square(Err(100)), 100);
///
/// ```
///
/// ### As unwrap_or
///
/// ```rust
/// #![feature(const_mut_refs)]
///
/// use const_format::{AsciiStr, strwriter_as_str, unwrap_or_else};
///
/// const FOO: AsciiStr = unwrap_or_else!(AsciiStr::new(b"AB\x80"), |_| AsciiStr::empty() );
///
/// const BAR: AsciiStr = unwrap_or_else!(AsciiStr::new(b"bar"), |_| loop{} );
///
/// assert_eq!(FOO.as_str(), "");
/// assert_eq!(BAR.as_str(), "bar");
///
/// ```
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "fmt")))]
#[cfg(feature = "fmt")]
#[macro_export]
macro_rules! unwrap_or_else {
    ($e:expr, |$($error:ident)? $(_)*| $orelse:expr ) => {
        match $e {
            $crate::pmr::Ok(x) => x,
            $crate::pmr::Err($($error,)?..) => $orelse,
        }
    };
}

/// Coerces a reference to a type that has a `const_*_fmt` method.
///
/// # Behavior
///
/// For arrays it coerces them into a slice, and wraps them in a [`PWrapper`].
///
/// For std types, it wraps them in a [`PWrapper`], which implements the
/// `const_*_fmt` methods.
///
/// For std types, it just returns back the same reference.
///
/// # Example
///
/// ```rust
/// #![feature(const_mut_refs)]
///
/// use const_format::{
///     for_examples::Unit,
///     Formatter, FormattingFlags, PWrapper, StrWriter,
///     coerce_to_fmt, strwriter_as_str,
/// };
///
/// const CAP: usize = 128;
/// const fn make_strwriter() -> StrWriter<[u8; CAP]> {
///     let mut writer = StrWriter::new([0; CAP]);
///     let mut fmt = Formatter::from_sw(&mut writer, FormattingFlags::NEW);
///
///     // This is equivalent to the `PWrapper::slice(&[0u8, 1])` below
///     let _ = coerce_to_fmt!([0u8, 1]).const_debug_fmt(&mut fmt);
///     let _ = fmt.write_str(",");
///
///     let _ = PWrapper::slice(&[0u8, 1]).const_debug_fmt(&mut fmt);
///     let _ = fmt.write_str(",");
///
///
///     // This is equivalent to the `PWrapper(100u32)` line
///     let _ = coerce_to_fmt!(100u32).const_debug_fmt(&mut fmt);
///     let _ = fmt.write_str(",");
///
///     let _ = PWrapper(100u32).const_debug_fmt(&mut fmt);
///     let _ = fmt.write_str(",");
///
///
///     // This is equivalent to the `Unit.const_debug_fmt(&mut fmt)` line
///     let _ = coerce_to_fmt!(Unit).const_debug_fmt(&mut fmt);
///
///
///     let _ = fmt.write_str(",");
///     let _ = Unit.const_debug_fmt(&mut fmt);
///
///     writer
/// }
///
/// const TEXT: &str = strwriter_as_str!(&make_strwriter());
///
/// assert_eq!(TEXT, "[0, 1],[0, 1],100,100,Unit,Unit");
///
/// ```
///
/// [`PWrapper`]: ./struct.PWrapper.html
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "fmt")))]
#[cfg(feature = "fmt")]
#[macro_export]
macro_rules! coerce_to_fmt {
    ($reference:expr) => {{
        match $reference {
            ref reference => {
                let marker = $crate::pmr::IsAFormatMarker::NEW;
                if false {
                    marker.infer_type(reference);
                }
                marker.coerce(marker.unreference(reference))
            }
        }
    }};
}

/// Converts a `&'static StrWriter` to a `&'static str`, in a `const`/`static` initializer.
///
/// This is usable in `const` or `static` initializers,
/// but not inside of `const fn`s.
///
/// # Runtime
///
/// If the "constant_time_as_str" feature is disabled,
/// thich takes time proportional to `$expr.capacity() - $expr.len()`.
///
/// If the "constant_time_as_str" feature is enabled, it takes constant time to run,
/// but uses a few additional nightly features.
///
/// # Example
///
/// ```rust
/// #![feature(const_mut_refs)]
///
/// use const_format::StrWriter;
/// use const_format::{strwriter_as_str, unwrap, writec};
///
///
/// const CAP: usize = 128;
///
/// // Can't use mutable references in `const`s yet, the `const fn` is a workaround
/// const fn formatted() -> StrWriter<[u8; CAP]> {
///     let mut writer =  StrWriter::new([0; CAP]);
///
///     // Writing the array with debug formatting, and the integers with hexadecimal formatting.
///     unwrap!(writec!(writer, "{:x}", [3u32, 5, 8, 13, 21, 34]));
///
///     writer
/// }
///
/// const STR: &str = strwriter_as_str!(&formatted());
///
/// fn main() {
///     assert_eq!(STR, "[3, 5, 8, D, 15, 22]");
/// }
/// ```
///
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "fmt")))]
#[cfg(feature = "fmt")]
#[macro_export]
macro_rules! strwriter_as_str {
    ($expr:expr) => {
        unsafe {
            let writer: &'static $crate::StrWriter = $expr;
            #[allow(clippy::transmute_bytes_to_str)]
            let ret = $crate::pmr::transmute::<&'static [u8], &'static str>(writer.as_bytes_alt());
            ret
        }
    };
}

#[cfg_attr(feature = "docsrs", doc(cfg(feature = "fmt")))]
#[cfg(feature = "fmt")]
macro_rules! conditionally_const {
    (
        feature = $feature:literal;
        $(
            $( #[$meta:meta] )*
            $vis:vis fn $fn_name:ident ($($params:tt)*) -> $ret:ty $block:block
        )*
    ) => (
        $(
            $(#[$meta])*
            #[cfg(feature = $feature)]
            $vis const fn $fn_name ($($params)*) -> $ret $block

            $(#[$meta])*
            #[cfg(not(feature = $feature))]
            $vis fn $fn_name ($($params)*) -> $ret $block
        )*
    )
}

#[cfg_attr(feature = "docsrs", doc(cfg(feature = "fmt")))]
#[cfg(feature = "fmt")]
macro_rules! std_kind_impl {
    (
        impl[$($impl:tt)*] $self:ty
        $(where[ $($where_:tt)* ])?
    )=>{
        impl<$($impl)*> $crate::pmr::FormatMarker for $self
        where
            $($($where_)*)?
        {
            type Kind = $crate::pmr::IsStdKind;
            type This = Self;
        }

        impl<$($impl)* __T> $crate::pmr::IsAFormatMarker<$crate::pmr::IsStdKind, $self, __T>
        where
            $($($where_)*)?
        {
            #[inline(always)]
            pub const fn coerce(self, reference: &$self) -> $crate::pmr::PWrapper<$self> {
                $crate::pmr::PWrapper(*reference)
            }
        }
    }
}
