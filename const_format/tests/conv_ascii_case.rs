#![cfg(feature = "const_generics")]

use const_format::__ascii_case_conv::{convert_str, size_after_conversion};
use const_format::{map_ascii_case, Case};

macro_rules! assert_case {
    ($case:expr, $input:expr, $output:expr $(,)?) => {{
        const IN: &str = $input;
        const OUT: &str = $output;
        const CASE: Case = $case;

        assert_eq!(size_after_conversion(CASE, IN), OUT.len());

        assert_eq!(convert_str::<{ OUT.len() }>(CASE, IN), OUT.as_bytes());

        assert_eq!(map_ascii_case!(CASE, IN), OUT);
    }};
}

#[test]
fn test_lowercase() {
    assert_case!(
        Case::Lower,
        "helloazWORLDAZ \u{303}n\u{303}Nñ",
        "helloazworldaz \u{303}n\u{303}nñ",
    );
}

#[test]
fn test_uppercasecase() {
    assert_case!(
        Case::Upper,
        "helloazWORLDAZ \u{303}n\u{303}Nñ",
        "HELLOAZWORLDAZ \u{303}N\u{303}Nñ",
    );
}
