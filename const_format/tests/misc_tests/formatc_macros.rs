use cfmt_b::formatcp;
use cfmt_b::test_utils::{ALL_ASCII, ALL_ASCII_ESCAPED};

#[cfg(feature = "fmt")]
use cfmt_b::formatc;

use arrayvec::{ArrayString, ArrayVec};

use core::fmt::Write;

macro_rules! fmt_assert {
    ( ($($arg:tt)*), $expected:expr $(,)? ) => ({
        let expected = $expected;

        assert_eq!(formatcp!($($arg)*), expected);

        #[cfg(feature = "fmt")]
        assert_eq!(formatc!($($arg)*), expected);
    })
}

#[test]
fn concat_fmt_strings() {
    fmt_assert!((concat!()), "");

    fmt_assert!((concat!("foo{}baz"), "bar"), "foobarbaz");

    fmt_assert!((concat!(r#"foo{}"#, "{:?}"), "bar", "baz"), "foobar\"baz\"");

    fmt_assert!((concat!(concat!())), "");
    fmt_assert!((concat!("foo", concat!())), "foo");
    fmt_assert!((concat!(concat!(), "foo")), "foo");
    fmt_assert!((concat!(concat!("foo"), "bar")), "foobar");
    fmt_assert!((concat!(concat!("foo"), concat!("bar"))), "foobar");
    fmt_assert!(
        (
            concat!(concat!("foo{}", "bar{}"), concat!(r#"baz{}"#, "qux{}")),
            3u8,
            5u8,
            13u8,
            21u8
        ),
        "foo3bar5baz13qux21",
    );

    fmt_assert!((concat!("foo{}baz"), "bar"), "foobarbaz");

    fmt_assert!((concat!(r#"foo{}"#, "{:?}"), "bar", "baz"), "foobar\"baz\"");
}

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

                #[cfg(feature = "fmt")]
                assert_eq!(formatc!("{:?}", $expr), string.as_str());
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

    ////////////////////////////
    //  Let's debug format all ascii characters!

    let mut escapedes = ArrayVec::<[&str; 16]>::new();

    escapedes.extend(
        [
            formatcp!("{:?}", ALL_ASCII),
            formatcp!("{:x}", ALL_ASCII),
            formatcp!("{:b}", ALL_ASCII),
            formatcp!("{:x?}", ALL_ASCII),
            formatcp!("{:X?}", ALL_ASCII),
            formatcp!("{:b?}", ALL_ASCII),
        ]
        .iter()
        .copied(),
    );

    #[cfg(feature = "fmt")]
    escapedes.extend(
        [
            formatc!("{:?}", ALL_ASCII),
            formatc!("{:x}", ALL_ASCII),
            formatc!("{:b}", ALL_ASCII),
            formatc!("{:x?}", ALL_ASCII),
            formatc!("{:X?}", ALL_ASCII),
            formatc!("{:b?}", ALL_ASCII),
        ]
        .iter()
        .copied(),
    );

    for escaped in escapedes.iter().copied() {
        assert_eq!(&escaped[..1], "\"");
        assert_eq!(&escaped[1..escaped.len() - 1], ALL_ASCII_ESCAPED);
        assert_eq!(&escaped[escaped.len() - 1..], "\"");
    }
}

macro_rules! binary_hex_test_case {
    ($ty:ident, $buffer:ident) => {{
        binary_hex_test_case! {@inner formatcp, $ty, $buffer};

        #[cfg(feature = "fmt")]
        binary_hex_test_case! {@inner formatc, $ty, $buffer};
    }};
    (@inner $themacro:ident, $ty:ident, $buffer:ident) => {{
        const P: &[$ty] = &[
            $ty::MIN,
            $ty::MIN + 1,
            $ty::MIN / 7,
            $ty::MIN / 13,
            0,
            1,
            4,
            $ty::MAX - 1,
            $ty::MAX,
        ];

        {
            let cp_string = $themacro!(
                "{0:?}_{0:X?}_{0:b?}_{0:#X?}_{0:#b?}_\
                 {1:?}_{1:X?}_{1:b?}_{1:#X?}_{1:#b?}_\
                 {2:?}_{2:X?}_{2:b?}_{2:#X?}_{2:#b?}_\
                 {3:?}_{3:X?}_{3:b?}_{3:#X?}_{3:#b?}_\
                 {4:?}_{4:X}_{4:b}_{4:#X}_{4:#b}_\
                 {5:?}_{5:X?}_{5:b?}_{5:#X?}_{5:#b?}_\
                 {6:?}_{6:X?}_{6:b?}_{6:#X?}_{6:#b?}_\
                 {7:?}_{7:X?}_{7:b?}_{7:#X?}_{7:#b?}_\
                 {8:?}_{8:X?}_{8:b?}_{8:#X?}_{8:#b?}_\
                 ",
                P[0],
                P[1],
                P[2],
                P[3],
                P[4],
                P[5],
                P[6],
                P[7],
                P[8]
            );

            $buffer.clear();
            write!(
                $buffer,
                "{0:?}_{0:X}_{0:b}_{0:#X}_{0:#b}_\
            {1:?}_{1:X}_{1:b}_{1:#X}_{1:#b}_\
            {2:?}_{2:X}_{2:b}_{2:#X}_{2:#b}_\
            {3:?}_{3:X}_{3:b}_{3:#X}_{3:#b}_\
            {4:?}_{4:X}_{4:b}_{4:#X}_{4:#b}_\
            {5:?}_{5:X}_{5:b}_{5:#X}_{5:#b}_\
            {6:?}_{6:X}_{6:b}_{6:#X}_{6:#b}_\
            {7:?}_{7:X}_{7:b}_{7:#X}_{7:#b}_\
            {8:?}_{8:X}_{8:b}_{8:#X}_{8:#b}_\
            ",
                P[0], P[1], P[2], P[3], P[4], P[5], P[6], P[7], P[8]
            )
            .unwrap();

            assert_eq!(cp_string, $buffer.as_str());
        }
        {
            let cp_string = $themacro!(
                "{0:x?}_{0:#x?}_\
                 {1:x?}_{1:#x?}_\
                 {2:x?}_{2:#x?}_\
                 {3:x?}_{3:#x?}_\
                 {4:x}_{4:#x}_\
                 {5:x?}_{5:#x?}_\
                 {6:x?}_{6:#x?}_\
                 {7:x?}_{7:#x?}_\
                 {8:x?}_{8:#x?}_\
                 ",
                P[0],
                P[1],
                P[2],
                P[3],
                P[4],
                P[5],
                P[6],
                P[7],
                P[8]
            );

            $buffer.clear();
            write!(
                $buffer,
                "{0:x?}_{0:#x?}_\
                 {1:x?}_{1:#x?}_\
                 {2:x?}_{2:#x?}_\
                 {3:x?}_{3:#x?}_\
                 {4:x}_{4:#x}_\
                 {5:x?}_{5:#x?}_\
                 {6:x?}_{6:#x?}_\
                 {7:x?}_{7:#x?}_\
                 {8:x?}_{8:#x?}_\
                 ",
                P[0], P[1], P[2], P[3], P[4], P[5], P[6], P[7], P[8]
            )
            .unwrap();

            assert_eq!(cp_string, $buffer.as_str());
        }
    }};
}

#[test]
fn binary_and_hex_formatting() {
    let mut s = ArrayString::<[u8; 4096]>::new();
    binary_hex_test_case!(u8, s);
    binary_hex_test_case!(u16, s);
    binary_hex_test_case!(u32, s);
    binary_hex_test_case!(u64, s);
    binary_hex_test_case!(u128, s);
    binary_hex_test_case!(i8, s);
    binary_hex_test_case!(i16, s);
    binary_hex_test_case!(i32, s);
    binary_hex_test_case!(i64, s);
    binary_hex_test_case!(i128, s);
}

#[test]
fn other_tests() {
    assert_eq!(formatcp!("{0:?}-{0:x?}-{0:b?}", ""), r#"""-""-"""#);

    const EMPTY: &str = formatcp!("");
    assert_eq!(EMPTY, "");
}

#[test]
fn escaped_format_string() {
    {
        let found = formatcp!(
            "\
             \x00\x01\x02\x03\x04\x05\x06\x07\x08\t\n\x0b\x0c\r\x0e\x0f\x10\
             \x11\x12\x13\x14\x15\x16\x17\x18\x19\x1a\x1b\x1c\x1d\x1e\x1f \
             !\"#$%&\'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]\
             ^_`abcdefghijklmnopqrstuvwxyz{{|}}~\x7f\u{80}\u{81}\u{90}\u{91}\
            "
        );
        assert_eq!(found, ALL_ASCII);
    }
    assert_eq!(formatcp!("\\"), r#"\"#);

    assert_eq!(formatcp!(r#"\u\u{}"#, "{}"), r#"\u\u{}"#);
    assert_eq!(formatcp!(r#"\u\u{{}}"#), r#"\u\u{}"#);
}

#[test]
fn raw_literals() {
    assert_eq!(formatcp!(r"-{}_", "hello"), r"-hello_");
    assert_eq!(formatcp!(r"-{{}}_"), r"-{}_");

    assert_eq!(formatcp!(r#"r"-{}_""#, "hello"), r#"r"-hello_""#);
    assert_eq!(formatcp!(r#"r"-{{}}_""#), r#"r"-{}_""#);

    assert_eq!(formatcp!(r##"r#"-{}_"#"##, "hello"), r##"r#"-hello_"#"##);
    assert_eq!(formatcp!(r##"r#"-{{}}_"#"##), r##"r#"-{}_"#"##);
}

#[test]
#[cfg(feature = "fmt")]
#[expect(non_local_definitions)]
fn access_formatter() {
    use cfmt_b::call_debug_fmt;

    struct FmtConst;

    assert_eq!(
        formatc!("{0}", |fmt| {
            impl FmtConst {
                const A: u32 = 3;
            }
            call_debug_fmt!(array, [(), ()], fmt)
        }),
        "[(), ()]"
    );

    assert_eq!(FmtConst::A, 3);

    assert_eq!(
        formatc!("{0} ; {0}", |fmt| { call_debug_fmt!(array, [(), ()], fmt) }),
        "[(), ()] ; [(), ()]"
    );

    assert_eq!(
        formatc!("{0}", |fmt| call_debug_fmt!(array, [(), ()], fmt)),
        "[(), ()]"
    );

    assert_eq!(
        formatc!("{0} ; {0}", |fmt| call_debug_fmt!(array, [(), ()], fmt)),
        "[(), ()] ; [(), ()]"
    );
}
