#![cfg(feature = "const_generics")]

use const_format::__ascii_case_conv::{convert_str, size_after_conversion};
use const_format::{map_ascii_case, Case};

macro_rules! assert_case {
    ($case:expr, $input:expr, $output:expr $(,)?) => {{
        const IN: &str = $input;
        const OUT: &str = $output;
        const CASE: Case = $case;

        assert_eq!(size_after_conversion(CASE, IN), OUT.len());

        assert_eq!(
            std::str::from_utf8(&convert_str::<{ OUT.len() }>(CASE, IN)).unwrap(),
            OUT,
        );

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

#[test]
fn test_snake_kebab_case() {
    assert_case!(Case::Snake, " __ 100 hello_nnWorld ", "100_hello_nn_world");
    assert_case!(
        Case::UpperSnake,
        " __ 100 hello_nnWorld ",
        "100_HELLO_NN_WORLD"
    );

    assert_case!(Case::Kebab, " __ 100 hello_nnWorld ", "100-hello-nn-world");
    assert_case!(
        Case::UpperKebab,
        " __ 100 hello_nnWorld ",
        "100-HELLO-NN-WORLD"
    );
}

#[test]
fn test_pascal_camel_case() {
    assert_case!(
        Case::Pascal,
        " _foo_ 100 hello_nnñWorld ",
        "Foo100HelloNnñWorld"
    );

    assert_case!(
        Case::Camel,
        " _bar_ 100 hello_nnñWorld ",
        "bar100HelloNnñWorld"
    );
}
