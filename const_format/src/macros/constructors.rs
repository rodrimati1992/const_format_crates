/// Constructs an `AsciiStr` constant from an ascii string,
///
/// # Compile-time errors
///
/// This macro produces a compile-time error by indexing an empty array with
/// the index of the first non-ascii byte.
///
/// # Example
///
/// ```rust
/// use const_format::ascii_str;
///
/// let fooo = ascii_str!("hello");
///
/// assert_eq!(fooo.as_str(), "hello")
///
/// ```
///
/// ```compile_fail
/// use const_format::ascii_str;
///
/// let fooo = ascii_str!("Γειά σου Κόσμε!");
///
/// ```
#[cfg(feature = "with_fmt")]
#[macro_export]
macro_rules! ascii_str {
    ($str:expr) => {{
        const __CF_ASCII_STR_CONSTANT: $crate::wrapper_types::AsciiStr = {
            match $crate::wrapper_types::AsciiStr::new($str.as_bytes()) {
                Ok(x) => x,
                $crate::pmr::Err(e) => [][e.invalid_from],
            }
        };
        __CF_ASCII_STR_CONSTANT
    }};
}
