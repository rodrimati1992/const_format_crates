use const_format::{
    ascii_str,
    fmt::{ComputeStrLength, Formatter, FormattingFlags},
    AsciiStr, PWrapper,
};

macro_rules! append_str {
    ($formatter:ident, $str:expr; $($statement:stmt;)*) => (
        $(
            $statement
            $formatter.write_str($str).unwrap();
        )*
    )
}

fn write_with_flag(flags: FormattingFlags, expected: &str, takes_fmt: &dyn Fn(Formatter<'_>)) {
    let mut len = 0;
    let mut buffer = [0; 4096];

    fn reset_start(buffer: &mut [u8], expected: &str) {
        buffer[..expected.len()].iter_mut().for_each(|x| *x = 0)
    }

    takes_fmt(Formatter::from_custom(&mut buffer, &mut len, flags));
    assert_eq!(std::str::from_utf8(&buffer[..len]).unwrap(), expected);

    len = usize::MAX / 2;
    reset_start(&mut buffer, expected);
    takes_fmt(Formatter::from_custom_cleared(&mut buffer, &mut len, flags));
    assert_eq!(std::str::from_utf8(&buffer[..len]).unwrap(), expected);

    let mut compute = ComputeStrLength::new();
    takes_fmt(compute.make_formatter(flags));
    assert_eq!(compute.len(), expected.len());
}

// This ensures that all integer methods have the correct type.
#[test]
fn write_integers() {
    fn inner(mut fmt: Formatter<'_>) {
        append_str!(fmt,",";
            fmt.write_u8_display(17_u8).unwrap();
            fmt.write_u16_display(18_u16).unwrap();
            fmt.write_u32_display(19_u32).unwrap();
            fmt.write_u64_display(20_u64).unwrap();
            fmt.write_u128_display(21_u128).unwrap();
            fmt.write_usize_display(22_usize).unwrap();
            fmt.write_i8_display(23_i8).unwrap();
            fmt.write_i16_display(24_i16).unwrap();
            fmt.write_i32_display(25_i32).unwrap();
            fmt.write_i64_display(26_i64).unwrap();
            fmt.write_i128_display(27_i128).unwrap();
            fmt.write_isize_display(28_isize).unwrap();
            fmt.write_u8_debug(29_u8).unwrap();
            fmt.write_u16_debug(30_u16).unwrap();
            fmt.write_u32_debug(31_u32).unwrap();
            fmt.write_u64_debug(32_u64).unwrap();
            fmt.write_u128_debug(33_u128).unwrap();
            fmt.write_usize_debug(34_usize).unwrap();
            fmt.write_i8_debug(35_i8).unwrap();
            fmt.write_i16_debug(36_i16).unwrap();
            fmt.write_i32_debug(37_i32).unwrap();
            fmt.write_i64_debug(38_i64).unwrap();
            fmt.write_i128_debug(39_i128).unwrap();
            fmt.write_isize_debug(40_isize).unwrap();
            fmt.write_i8_debug(-41_i8).unwrap();
            fmt.write_i16_debug(-42_i16).unwrap();
            fmt.write_i32_debug(-43_i32).unwrap();
            fmt.write_i64_debug(-44_i64).unwrap();
            fmt.write_i128_debug(-45_i128).unwrap();
            fmt.write_isize_debug(-46_isize).unwrap();
        );
    }

    {
        let expected = "\
            17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,\
            -41,-42,-43,-44,-45,-46,\
        ";

        write_with_flag(FormattingFlags::NEW, expected, &inner);
    }
    {
        let expected = format!(
            "\
                17,18,19,20,21,22,23,24,25,26,27,28,1D,1E,1F,20,21,22,23,24,25,26,27,28,D7,\
                FFD6,FFFFFFD5,FFFFFFFFFFFFFFD4,FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFD3,{:X},\
            ",
            -46_isize
        );

        write_with_flag(FormattingFlags::NEW.set_hexadecimal(), &expected, &inner);
    }
    {
        let expected = format!(
            "\
                17,18,19,20,21,22,23,24,25,26,27,28,11101,\
                11110,11111,100000,100001,100010,100011,100100,100101,100110,100111,101000,\
                11010111,1111111111010110,11111111111111111111111111010101,\
                1111111111111111111111111111111111111111111111111111111111010100,\
                1111111111111111111111111111111111111111111111111111111111111111\
                1111111111111111111111111111111111111111111111111111111111010011,\
                {:b},\
            ",
            -46_isize
        );

        write_with_flag(FormattingFlags::NEW.set_binary(), &expected, &inner);
    }
}

#[test]
fn write_str_methods() {
    const A_FOO: AsciiStr = ascii_str!("hello\n\x1F bar\t\rbaz");
    const A_BAR: AsciiStr = ascii_str!("what\0the");
    const FOO: &str = "hello\n\x1F bar\t\rbaz";

    fn inner(mut fmt: Formatter<'_>) {
        append_str!(fmt,";;";
            fmt.write_str_range(FOO, 1..6).unwrap();
            fmt.write_str("\nABCD\n").unwrap();
            fmt.write_ascii_range(A_FOO, 6..11).unwrap();
            fmt.write_ascii(A_BAR).unwrap();
            fmt.write_ascii_repeated(b'-', 4).unwrap();
            fmt.write_str_range_debug(FOO, 1..6).unwrap();
            fmt.write_str_debug("\nABCD\n").unwrap();
            fmt.write_ascii_range_debug(A_FOO, 6..11).unwrap();
            fmt.write_ascii_debug(A_BAR).unwrap();
        );
    }

    let expected = "\
        ello\n;;\nABCD\n;;\x1F bar;;what\0the;;----;;\
        \"ello\\n\";;\"\\nABCD\\n\";;\"\\x1F bar\";;\"what\\x00the\";;\
    ";

    write_with_flag(FormattingFlags::NEW, expected, &inner);
}
