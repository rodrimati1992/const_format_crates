use arrayvec::ArrayString;

use core::fmt::Write;

#[test]
fn positional_and_named_arguments() {
    // Positional,implicit
    assert_eq!(formatcp!("{}{}{}", 3u8, 5u8, 8u8), "358");

    // Positional explicit
    assert_eq!(formatcp!("{0},{1},{2},{1},{0}", 3u8, 5u8, 8u8), "3,5,8,5,3");

    const A: u32 = 3;
    const B: u32 = 5;

    // Uses the named argument if it's provided,otherwise looks for constant from scope.
    assert_eq!(formatcp!("{A},{B}", B = 8u8), "3,8");
    assert_eq!(formatcp!("{A},{B}"), "3,5");

    // implicit positional args aren't affected by explicit ones.
    assert_eq!(
        formatcp!("{2},{},{2},{},{P},{}", 3u8, 5u8, 8u8, P = 89u8),
        "8,3,8,5,89,8"
    );
}

// Display formatting is already tested in the `shared_cp_macro_tests` module
#[test]
fn debug_formatting() {
    let mut string = ArrayString::<[u8; 64]>::new();

    macro_rules! same_as_display {
        ($($expr:expr),* $(,)?) => (
            $(
                string.clear();
                write!(string, "{:?}", $expr).unwrap();

                assert_eq!(formatcp!("{:?}", $expr), string.as_str());
            )*
        )
    }

    same_as_display! {
        i8::MIN, i8::MAX,
        i16::MIN, i16::MAX,
        i32::MIN, i32::MAX,
        i64::MIN, i64::MAX,
        i128::MIN, i128::MAX,
        isize::MIN, isize::MAX,
        u8::MIN, u8::MAX,
        u16::MIN, u16::MAX,
        u32::MIN, u32::MAX,
        u64::MIN, u64::MAX,
        u128::MIN, u128::MAX,
        usize::MIN, usize::MAX,
        false, true,
    }

    assert_eq!(formatcp!("{:?}", ""), r#""""#);

    assert_eq!(
        formatcp!("{:?}", r#" !Aq¡\"🧡🧠₀₁ "#),
        r#"" !Aq¡\\\"🧡🧠₀₁ ""#
    );
}

#[test]
fn other_tests() {
    const EMPTY: &str = formatcp!("");
    assert_eq!(EMPTY, "");
}
