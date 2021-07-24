/// Converts the casing style of a `&'static str` constant,
/// ignoring non-ascii unicode characters.
///
/// This nacro is equivalent to a function with this signature:
///
/// ```rust
/// const fn map_ascii_case(case: const_format::Case, input: &'static str) -> &'static str
/// # {""}
/// ```
///
/// # Ascii
///
/// This only operates on ascii characters because broader unicode case conversion,
/// while possible, is much harder to implement purely with `const fn`s.
///
/// # Example
///
/// ```rust
/// use const_format::{Case, map_ascii_case};
///
/// const LOW: &str = map_ascii_case!(Case::Lower, "hello WORLD");
/// assert_eq!(LOW, "hello world");
///
/// const IN: &str = "hello WORLD каждому";
/// const UPPER: &str = map_ascii_case!(Case::Upper, IN);
/// // non-ascii characters are ignored by map_ascii_case.
/// assert_eq!(UPPER, "HELLO WORLD каждому");
///
/// ```
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "const_generics")))]
#[macro_export]
macro_rules! map_ascii_case {
    ($case:expr, $str:expr) => {{
        const S_OSRCTFL4A: &$crate::pmr::str = $str;
        const CASE_OSRCTFL4A: $crate::Case = $case;
        {
            const L: $crate::pmr::usize =
                $crate::__ascii_case_conv::size_after_conversion(CASE_OSRCTFL4A, S_OSRCTFL4A);

            const OB: &[$crate::pmr::u8; L] =
                &$crate::__ascii_case_conv::convert_str::<L>(CASE_OSRCTFL4A, S_OSRCTFL4A);

            const OS: &$crate::pmr::str =
                unsafe { $crate::pmr::transmute::<&[$crate::pmr::u8], &$crate::pmr::str>(OB) };

            OS
        }
    }};
}
