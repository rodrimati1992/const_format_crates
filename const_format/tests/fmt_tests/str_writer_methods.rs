use crate::RngExt;

use const_format::{
    fmt::{Error, FormattingFlags, StrWriter, StrWriterMut},
    formatcp,
    test_utils::{ALL_ASCII, ALL_ASCII_ESCAPED},
    utils::saturate_range,
    wrapper_types::{AsciiStr, PWrapper},
};

use arrayvec::ArrayString;

use fastrand::Rng;

use core::{fmt::Write, ops::Range};

#[derive(Debug, Copy, Clone)]
enum Formatting {
    Debug,
    Display,
}

macro_rules! write_integer_tests {
    (
        $(($display_fn:ident, $debug_fn:ident, $type:ident))*
    ) => ({
        $({
            let writer: &mut StrWriter = &mut StrWriter::new([0; 1024]);
            let mut writer = writer.as_mut();

            assert!( writer.is_empty() );

            let snapshot = writer.len();
            let mut string = ArrayString::<[u8; 1024]>::new();

            let flags = FormattingFlags::DEFAULT;

            for number in &[$type::MIN, $type::MAX][..] {
                let mut number = *number;

                loop {
                    writer.truncate(snapshot).unwrap();
                    string.clear();

                    // Adding some padding at the end so that numbers can't accidentally overlap
                    writer.$display_fn(number).unwrap();
                    writer.write_str("_").unwrap();
                    writer.$debug_fn(number, flags.set_decimal()).unwrap();
                    writer.write_str("_").unwrap();
                    writer.$debug_fn(number, flags.set_binary()).unwrap();
                    writer.write_str("_").unwrap();
                    writer.$debug_fn(number, flags.set_hexadecimal()).unwrap();
                    writer.write_str("__").unwrap();

                    assert!( !writer.is_empty() );


                    let fmt = &mut writer.make_formatter(flags);
                    PWrapper(number).const_display_fmt(fmt).unwrap();
                    writer.write_str("_").unwrap();

                    let fmt = &mut writer.make_formatter(flags.set_decimal());
                    PWrapper(number).const_debug_fmt(fmt).unwrap();
                    writer.write_str("_").unwrap();

                    let fmt = &mut writer.make_formatter(flags.set_binary());
                    PWrapper(number).const_debug_fmt(fmt).unwrap();
                    writer.write_str("_").unwrap();

                    let fmt = &mut writer.make_formatter(flags.set_hexadecimal());
                    PWrapper(number).const_debug_fmt(fmt).unwrap();


                    write!(
                        string,
                        "{0}_{0:?}_{0:b}_{0:X}__{0}_{0:?}_{0:b}_{0:X}",
                        number
                    ).unwrap();

                    assert_eq!(
                        writer.as_bytes(),
                        string.as_bytes(),
                        "\n\ntype:{}\n\n",
                        stringify!($type)
                    );

                    // This assertion must go after the as_bytes one
                    assert_eq!(
                        writer.as_str(),
                        string.as_str(),
                        "\n\ntype:{}\n\n",
                        stringify!($type)
                    );

                    if number == 0 {
                        break;
                    }

                    let prev = number;
                    number = number / 3 * 2;
                    if number > 2 {
                        number += prev % 2;
                    }
                }
            }

        })*
    })
}

#[test]
fn test_write_ints() {
    write_integer_tests! {
        (write_u8_display, write_u8_debug, u8)
        (write_u16_display, write_u16_debug, u16)
        (write_u32_display, write_u32_debug, u32)
        (write_u64_display, write_u64_debug, u64)
        (write_u128_display, write_u128_debug, u128)
        (write_usize_display, write_usize_debug, usize)
        (write_i8_display, write_i8_debug, i8)
        (write_i16_display, write_i16_debug, i16)
        (write_i32_display, write_i32_debug, i32)
        (write_i64_display, write_i64_debug, i64)
        (write_i128_display, write_i128_debug, i128)
        (write_isize_display, write_isize_debug, isize)
    }
}

#[test]
fn basic() {
    assert_eq!(
        formatcp!("{:?}", r#" !Aq¡\"🧡🧠₀₁ "#),
        r#"" !Aq¡\\\"🧡🧠₀₁ ""#
    );
}

#[test]
fn saturate_range_tests() {
    let all_ascii = ALL_ASCII.as_bytes();
    let len = ALL_ASCII.len();

    let same_ranges = [0..len, 1..len - 1, 2..len - 2, 3..len - 3];

    for range in same_ranges.iter() {
        assert_eq!(saturate_range(all_ascii, range), *range);
    }

    let saturated_ranges = [(10..!0, 10..len), (!0..10, 10..10), (!0..!0, len..len)];

    for (range, expected) in saturated_ranges.iter().cloned() {
        assert_eq!(saturate_range(all_ascii, &range), expected);
    }
}

struct WriteArgs<'sw, 's> {
    writer: StrWriterMut<'sw>,
    input: &'s str,
    range: Range<usize>,
    sat_range: Range<usize>,
}

fn test_unescaped_str_fn(
    formatting: Formatting,
    rng: &mut dyn FnMut() -> char,
    write: &mut dyn FnMut(WriteArgs<'_, '_>) -> Result<(), const_format::fmt::Error>,
) {
    for _ in 0..64 {
        let mut input = ArrayString::<[u8; 32]>::new();
        while input.try_push(rng()).is_ok() {}

        let input = input.as_str();
        let input_bytes = input.as_bytes();

        for end in 0..input.len() {
            for start in 0..end + 2 {
                let writer: &mut StrWriter = &mut StrWriter::new([0; 192]);
                let mut writer = writer.as_mut();

                let toosmall: &mut StrWriter = &mut StrWriter::new([0; 8]);
                let toosmall = toosmall.as_mut();

                let range = start..end;
                let sat_range = saturate_range(input_bytes, &range);

                writer.write_u8_display(0).unwrap();
                writer.write_u8_display(0).unwrap();
                let __just_dont_panic = write(WriteArgs {
                    writer: toosmall,
                    input,
                    range: range.clone(),
                    sat_range: sat_range.clone(),
                });
                let res = write(WriteArgs {
                    writer: writer.reborrow(),
                    input,
                    range: range.clone(),
                    sat_range: sat_range.clone(),
                });
                writer.write_u8_display(0).unwrap();
                writer.write_u8_display(0).unwrap();

                let bytes = writer.as_bytes();

                // 00.....00 with Display
                // 00"....."00 with Debug
                let around = match formatting {
                    Formatting::Display { .. } => 2,
                    Formatting::Debug { .. } => 3,
                };

                assert_eq!(&bytes[..2], b"00");
                assert_eq!(&bytes[bytes.len() - 2..], b"00");

                assert_eq!(res.is_ok(), input.get(sat_range.clone()).is_some());

                if res.is_ok() {
                    assert_eq!(
                        &bytes[around..bytes.len() - around],
                        &input_bytes[sat_range.clone()],
                        "\n\nbytes: {:?}\n\n",
                        bytes,
                    );
                }
            }
        }
    }
}

#[test]
fn write_str() {
    let rng = Rng::new();
    test_unescaped_str_fn(
        Formatting::Display,
        &mut || rng.unicode_char(),
        &mut |mut p| p.writer.write_str_range(p.input, p.range),
    );
    test_unescaped_str_fn(
        Formatting::Display,
        &mut || rng.unicode_char(),
        &mut |mut p| {
            let input = p.input.get(p.sat_range).ok_or(Error::NotOnCharBoundary)?;
            p.writer.write_str(input)
        },
    );
}

#[test]
fn write_ascii() {
    let rng = Rng::new();
    let mut rng_fn = || rng.char_('\0'..='\u{7F}');
    test_unescaped_str_fn(Formatting::Display, &mut rng_fn, &mut |mut p| {
        let ascii = AsciiStr::new(p.input.as_bytes()).unwrap();
        p.writer.write_ascii_range(ascii, p.range)
    });
    test_unescaped_str_fn(Formatting::Display, &mut rng_fn, &mut |mut p| {
        let ascii = AsciiStr::new(&p.input.as_bytes()[p.sat_range]).unwrap();
        p.writer.write_ascii(ascii)
    });
}

fn is_it_escaped(c: char) -> bool {
    matches!(c, '\0'..='\u{1F}' | '\\' | '"' | '\'')
}

#[test]
fn write_str_debug() {
    {
        let rng = Rng::new();
        let mut rng_fn = || loop {
            let c = rng.unicode_char();
            if !is_it_escaped(c) {
                break c;
            }
        };
        test_unescaped_str_fn(Formatting::Debug, &mut rng_fn, &mut |mut p| {
            p.writer.write_str_range_debug(p.input, p.range)
        });
        test_unescaped_str_fn(Formatting::Debug, &mut rng_fn, &mut |mut p| {
            let input = p.input.get(p.sat_range).ok_or(Error::NotOnCharBoundary)?;
            p.writer.write_str_debug(input)
        });
    }

    // Testing that all ascii characters are escaped as expected
    let writer: &mut StrWriter = &mut StrWriter::new([0; 512]);
    let mut writer = writer.as_mut();

    let snapshot = writer.len();
    {
        writer.truncate(snapshot).unwrap();
        writer.write_str_debug(ALL_ASCII).unwrap();

        let bytes = writer.as_bytes();
        assert_eq!(bytes[0], b'"');
        assert_eq!(&bytes[1..bytes.len() - 1], ALL_ASCII_ESCAPED.as_bytes());
        assert_eq!(bytes[bytes.len() - 1], b'"');
    }
    {
        let all_ascii = AsciiStr::new(&ALL_ASCII.as_bytes()[..128]).unwrap();

        writer.truncate(snapshot).unwrap();
        writer.write_ascii_debug(all_ascii).unwrap();
        let end = ALL_ASCII_ESCAPED.find('\u{80}').unwrap();

        let bytes = writer.as_bytes();
        assert_eq!(bytes[0], b'"');
        assert_eq!(
            &bytes[1..bytes.len() - 1],
            &ALL_ASCII_ESCAPED.as_bytes()[..end]
        );
        assert_eq!(bytes[bytes.len() - 1], b'"');
    }

    // Testing that escaping random ranges in ALL_ASCII produces escaped strings
    // that can be found in ALL_ASCII_ESCAPED
    {
        let rng = Rng::new();
        fn write_range(rng: &Rng, mut w: StrWriterMut<'_>) -> Range<usize> {
            let gen_range = || rng.usize(..ALL_ASCII.len())..rng.usize(..ALL_ASCII.len());
            let start = w.len();
            while let Err(_) = w.write_str_range_debug(ALL_ASCII, gen_range()) {}
            let end = w.len();
            start + 1..end - 1
        };

        for _ in 1..1000 {
            writer.truncate(snapshot).unwrap();

            let range_a = write_range(&rng, writer.reborrow());
            let range_b = write_range(&rng, writer.reborrow());
            let range_c = write_range(&rng, writer.reborrow());

            let written = writer.as_str();

            let haystack = ALL_ASCII_ESCAPED;

            assert!(
                haystack.contains(&written[range_a.clone()]),
                "{:?} {:?}",
                range_a,
                written
            );
            assert!(
                haystack.contains(&written[range_b.clone()]),
                "{:?} {:?}",
                range_b,
                written
            );
            assert!(
                haystack.contains(&written[range_c.clone()]),
                "{:?} {:?}",
                range_c,
                written
            );
        }
    }
}

// Makes sure that a StrWriter that's too small for a string returns an Error
// instead of panicking
#[test]
fn returns_error() {
    const ZEROES_AROUND_STR: usize = 2;

    fn test_case_(writer: &mut StrWriter, s: &str, extra: usize) {
        let str_writer_cap = writer.capacity();
        let mut writer = writer.as_mut();
        writer.write_u8_display(0).unwrap();
        writer.write_u8_display(0).unwrap();
        let res = writer.write_str_debug(s);
        assert_eq!(&writer.as_bytes()[..2], &b"00"[..]);

        if writer.capacity() == extra && str_writer_cap == extra {
            res.unwrap();
        } else {
            res.unwrap_err();
            assert_eq!(writer.len(), ZEROES_AROUND_STR);
        }
    }

    macro_rules! test_case {
        ($str:expr, $extra:expr) => {{
            const S: &str = $str;
            const EXTRA: usize = ZEROES_AROUND_STR + S.len() + $extra + 2;
            test_case_(&mut StrWriter::new([0; EXTRA - 2]), S, EXTRA);
            test_case_(&mut StrWriter::new([0; EXTRA - 1]), S, EXTRA);
            test_case_(&mut StrWriter::new([0; EXTRA]), S, EXTRA);
        }};
    }

    test_case!("foo\nb", 1);
    test_case!("foo\"ba", 1);
    test_case!("foo\'bar", 1);
    test_case!("foo\rbarb", 1);
    test_case!("foo\\barba", 1);
    test_case!("foo\u{5}bar", 3);
    test_case!("foo\u{11}bar", 3);
}

#[test]
fn remaining_capacity_test() {
    const CAP: usize = 16;

    let underscored = "___________________________________";

    let str_writer: &mut StrWriter = &mut StrWriter::new([0; CAP]);
    assert_eq!(str_writer.remaining_capacity(), CAP);

    for i in (0..CAP).rev() {
        str_writer.as_mut().write_str("_").unwrap();
        assert_eq!(str_writer.remaining_capacity(), i);
    }
    assert_eq!(str_writer.remaining_capacity(), 0);
    assert_eq!(str_writer.as_str(), &underscored[..str_writer.capacity()]);

    str_writer.truncate(5).unwrap();
    assert_eq!(str_writer.remaining_capacity(), CAP - 5);
}

#[test]
fn truncation() {
    let str_writer: &mut StrWriter = &mut StrWriter::new([0; 4096]);
    let mut writer = str_writer.as_mut();

    let snapshot = writer.len();

    writer.write_str("hello").unwrap();
    assert_eq!(writer.as_bytes(), "hello".as_bytes());

    writer.truncate(snapshot).unwrap();
    assert_eq!(writer.as_bytes(), "".as_bytes());

    writer.write_str("world").unwrap();
    assert_eq!(writer.as_bytes(), "world".as_bytes());

    {
        let nested = writer.len();

        writer
            .write_str("\u{0000}\u{0080}\u{0800}\u{10000}")
            .unwrap();
        assert_eq!(
            writer.as_bytes(),
            "world\u{0000}\u{0080}\u{0800}\u{10000}".as_bytes()
        );
        let with_foo_len = writer.len();

        writer.truncate(15).unwrap();
        assert_eq!(
            writer.as_bytes(),
            "world\u{0000}\u{0080}\u{0800}\u{10000}".as_bytes()
        );

        assert_eq!(writer.truncate(14).unwrap_err(), Error::NotOnCharBoundary);
        assert_eq!(writer.truncate(13).unwrap_err(), Error::NotOnCharBoundary);
        assert_eq!(writer.truncate(12).unwrap_err(), Error::NotOnCharBoundary);

        writer.truncate(11).unwrap();
        assert_eq!(
            writer.as_bytes(),
            "world\u{0000}\u{0080}\u{0800}".as_bytes()
        );

        let writer = &mut *str_writer;

        assert_eq!(writer.truncate(10).unwrap_err(), Error::NotOnCharBoundary);
        assert_eq!(writer.truncate(9).unwrap_err(), Error::NotOnCharBoundary);

        writer.truncate(8).unwrap();
        assert_eq!(writer.as_bytes(), "world\u{0000}\u{0080}".as_bytes());

        assert_eq!(writer.truncate(7).unwrap_err(), Error::NotOnCharBoundary);

        writer.truncate(6).unwrap();
        assert_eq!(writer.as_bytes(), "world\u{0000}".as_bytes());

        writer.truncate(nested).unwrap();
        assert_eq!(writer.as_bytes(), "world".as_bytes());

        writer.truncate(with_foo_len).unwrap();
        assert_eq!(writer.len(), 5);
    }
    writer = str_writer.as_mut();

    assert_eq!(writer.as_bytes(), "world".as_bytes());
    writer.truncate(snapshot).unwrap();
    assert_eq!(writer.as_bytes(), "".as_bytes());

    writer.truncate(5).unwrap();
    assert_eq!(writer.len(), 0);
}

#[test]
fn as_bytes() {
    let writer: &mut StrWriter = &mut StrWriter::new([0; 512]);
    let mut string = String::new();

    for i in 0..512 {
        let ascii_char = ((i % 61) + 32) as u8;
        writer.as_mut().write_ascii_repeated(ascii_char, 1).unwrap();
        string.push(ascii_char as char);

        assert_eq!(writer.as_bytes(), string.as_bytes());
        assert_eq!(writer.as_bytes_alt(), string.as_bytes());
        assert_eq!(writer.as_str(), string.as_str());
    }
}

#[test]
fn clear() {
    let str_writer: &mut StrWriter = &mut StrWriter::new([0; 16]);

    {
        let mut writer = str_writer.as_mut();

        writer.write_str("hello").unwrap();
    }

    assert_eq!(str_writer.as_bytes(), "hello".as_bytes());
    str_writer.clear();
    assert_eq!(str_writer.as_bytes(), "".as_bytes());

    {
        let mut writer = str_writer.as_mut();

        writer.write_str("hello").unwrap();
        assert_eq!(writer.as_bytes(), "hello".as_bytes());
        writer.clear();
        assert_eq!(writer.as_bytes(), "".as_bytes());
    }
}

#[test]
fn write_ascii_debug() {
    let rng = Rng::new();
    let mut rng_fn = || loop {
        let c = rng.char_('\0'..='\u{7F}');
        if !is_it_escaped(c) {
            break c;
        }
    };
    test_unescaped_str_fn(Formatting::Debug, &mut rng_fn, &mut |mut p| {
        let ascii = AsciiStr::new(p.input.as_bytes()).unwrap();
        p.writer.write_ascii_range_debug(ascii, p.range)
    });
    test_unescaped_str_fn(Formatting::Debug, &mut rng_fn, &mut |mut p| {
        let ascii = AsciiStr::new(&p.input.as_bytes()[p.sat_range]).unwrap();
        p.writer.write_ascii_debug(ascii)
    });
}
