use arrayvec::ArrayString;

use core::fmt::Write;

macro_rules! tests {
    (
        $($expr:expr,)*
    ) => (
        const ALL_TYS: &'static str = concatcp!( $($expr,)* );

        #[test]
        fn all_types() {
            let mut string = ArrayString::<[u8; 1024]>::new();
            $(
                write!(string, "{}", $expr).unwrap();
            )*
            assert_eq!(string.as_str(), ALL_TYS);
        }

        #[test]
        fn each_type(){
            $({
                const VALUE: &'static str = concatcp!($expr);
                let mut string = ArrayString::<[u8; 64]>::new();
                write!(string, "{}", $expr).unwrap();
                assert_eq!(string.as_str(), VALUE);
            })*
        }
    )
}

tests! {
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
}

#[test]
fn other_tests() {
    const EMPTY: &str = concatcp!();
    assert_eq!(EMPTY, "");
}
