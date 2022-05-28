use cfmt_b::{__identity, concatcp, formatcp};

#[cfg(feature = "fmt")]
use cfmt_b::{concatc, formatc};

use arrayvec::ArrayString;

use core::fmt::Write;

macro_rules! tests {
    (
        format_str = $format_str:literal,
        $($expr:expr,)*
    ) => (
        const ALL_TYS_F: &'static str = formatcp!( $format_str, $($expr,)* );
        const ALL_TYS: &'static str = concatcp!( $($expr,)* );

        #[cfg(feature = "fmt")]
        const ALL_TYS_C: &'static str = concatc!( $($expr,)* );

        #[test]
        fn all_types() {
            let mut string = ArrayString::<[u8; 1024]>::new();
            $(
                write!(string, "{}", $expr).unwrap();
            )*
            assert_eq!(string.as_str(), ALL_TYS);

            assert_eq!(string.as_str(), ALL_TYS_F);

            #[cfg(feature = "fmt")]
            assert_eq!(string.as_str(), ALL_TYS_C);
        }

        #[test]
        fn each_type(){
            $({
                const VALUE_F: &'static str = formatcp!("{}", $expr);
                const VALUE: &'static str = concatcp!($expr);

                let mut string = ArrayString::<[u8; 64]>::new();
                write!(string, "{}", $expr).unwrap();

                assert_eq!(string.as_str(), VALUE);
                assert_eq!(string.as_str(), VALUE_F);
            })*
        }
    )
}

tests! {
    format_str = "\
        {}{}{}{}{}{}{}{}\
        {}{}{}{}{}{}{}{}\
        {}{}{}{}{}{}{}{}\
        {}{}{}{}{}{}{}{}\
        {}{}{}{}{}{}{}{}\
        {}{}{}{}{}{}{}{}\
        {}{}{}{}{}{}{}{}\
    ",

    i8::MIN, " ", i8::MAX, " ",
    i16::MIN, " ", i16::MAX, " ",
    i32::MIN, " ", i32::MAX, " ",
    i64::MIN, " ", i64::MAX, " ",
    i128::MIN, " ", i128::MAX, " ",
    isize::MIN, " ", isize::MAX, " ",
    "!Aq¡🧡🧠₀₁",
    "",
    u8::MIN, " ", u8::MAX, " ",
    u16::MIN, " ", u16::MAX, " ",
    u32::MIN, " ", u32::MAX, " ",
    u64::MIN, " ", u64::MAX, " ",
    u128::MIN, " ", u128::MAX, " ",
    usize::MIN, " ", usize::MAX, " ",
    false, true,
    'o', 'ñ', '个', '\u{100000}',
}

#[test]
fn other_tests() {
    {
        const EMPTY: &str = concatcp!();
        assert_eq!(EMPTY, "");
    }

    #[cfg(feature = "fmt")]
    {
        const EMPTY: &str = concatc!();
        assert_eq!(EMPTY, "");
    }
}

// test that this error doesn't happen:
// https://github.com/rodrimati1992/const_format_crates/issues/36
#[test]
fn call_in_braced_macro() {
    {
        assert_eq!(
            {
                __identity! {concatcp!("a", "b")}
            },
            "ab"
        );
        {
            __identity! {concatcp!("a", "b")}
        };
        __identity! {concatcp!("a", "b")};
    }

    #[cfg(feature = "fmt")]
    {
        assert_eq!(
            {
                __identity! {concatc!("a", "b")}
            },
            "ab"
        );
        {
            __identity! {concatc!("a", "b")}
        };
        __identity! {concatc!("a", "b")};
    }

    {
        assert_eq!(
            {
                __identity! {formatcp!("ab")}
            },
            "ab"
        );
        {
            __identity! {formatcp!("ab")}
        };
        __identity! {formatcp!("ab")};
    }

    #[cfg(feature = "fmt")]
    {
        assert_eq!(
            {
                __identity! {formatc!("ab")}
            },
            "ab"
        );
        {
            __identity! {formatc!("ab")}
        };
        __identity! {formatc!("ab")};
    }
}
