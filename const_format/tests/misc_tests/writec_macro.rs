// Don't need the tests for this macro to be thorough,
// since this uses a lot of the same machinery as `formatcp` and `formatc`

use cfmt_b::fmt::{Error, Formatter, FormattingFlags, StrWriter};
use cfmt_b::{try_, writec};

struct Foo {
    x: u32,
    y: &'static str,
}

#[test]
fn basic() {
    const fn inner_0(writer: &mut StrWriter) -> Result<(), Error> {
        writer.clear();
        try_!(writec!(writer, "10"));
        try_!(writec!(writer, "-"));
        try_!(writec!(writer, "20"));
        Ok(())
    }
    const fn inner_1(writer: &mut StrWriter) -> Result<(), Error> {
        writer.clear();
        try_!(writec!(writer, ""));
        Ok(())
    }

    let writer: &mut StrWriter = &mut StrWriter::new([0; 40]);
    inner_0(writer).unwrap();
    assert_eq!(writer.as_str(), "10-20");

    inner_1(writer).unwrap();
    assert_eq!(writer.as_str(), "");
}

#[test]
fn repeated_positional_args() {
    const fn inner(foo: &Foo, writer: &mut StrWriter) -> Result<(), Error> {
        writer.clear();
        try_!(writec!(
            writer,
            "{0:},{0:?},{0:#x},{0:#X},{0:#b},{1},{1:?}",
            foo.x,
            foo.y
        ));
        Ok(())
    }

    let foo = Foo {
        x: 13,
        y: "foo\nbar\tbaz\x00",
    };

    let writer: &mut StrWriter = &mut StrWriter::new([0; 256]);
    inner(&foo, writer).unwrap();
    assert_eq!(
        writer.as_str(),
        "13,13,0xd,0xD,0b1101,foo\nbar\tbaz\x00,\"foo\\nbar\\tbaz\\x00\""
    );
}

#[test]
fn write_from_consts() {
    const FOO: Foo = Foo {
        x: 13,
        y: "foo\nbar\tbaz\x00",
    };

    const fn inner(f: &mut Formatter<'_>) -> Result<(), Error> {
        const X: u32 = FOO.x;
        const Y: &str = FOO.y;
        try_!(writec!(f, "{X:},{X:?},{X:#x},{X:#X},{X:#b},{Y},{Y:?}"));
        Ok(())
    }

    let writer: &mut StrWriter = &mut StrWriter::new([0; 256]);
    inner(&mut writer.make_formatter(FormattingFlags::NEW)).unwrap();
    assert_eq!(
        writer.as_str(),
        "13,13,0xd,0xD,0b1101,foo\nbar\tbaz\x00,\"foo\\nbar\\tbaz\\x00\""
    );
}

#[test]
fn named_parameters() {
    const fn inner(f: &mut Formatter<'_>) -> Result<(), Error> {
        try_!(writec!(
            f,
            "{x},{y},{},{},{x:b},{y:x},{y:X},{:?}",
            21u8,
            34u8,
            55..89,
            x = 8u8,
            y = 13u8
        ));
        Ok(())
    }

    let writer: &mut StrWriter = &mut StrWriter::new([0; 256]);
    inner(&mut writer.make_formatter(FormattingFlags::NEW)).unwrap();
    assert_eq!(writer.as_str(), "8,13,21,34,1000,d,D,55..89");
}

#[test]
fn write_from_locals() {
    const fn inner(f: &mut Formatter<'_>) -> Result<(), Error> {
        let foo = 13u8;
        let bar = "58";

        innerb(f, foo, bar)
    }
    const fn innerb(f: &mut Formatter<'_>, foo: u8, bar: &str) -> Result<(), Error> {
        writec!(
            f,
            "{foo},{bar},{foo:?},{bar:?},{foo:x},{bar:x},{foo:X},{bar:X},{foo:b},{bar:b}"
        )
    }

    let writer: &mut StrWriter = &mut StrWriter::new([0; 96]);
    inner(&mut writer.make_formatter(FormattingFlags::NEW)).unwrap();
    assert_eq!(writer.as_str(), r#"13,58,13,"58",d,"58",D,"58",1101,"58""#);

    writer.clear();
    inner(&mut writer.make_formatter(FormattingFlags::NEW)).unwrap();
    assert_eq!(writer.as_str(), r#"13,58,13,"58",d,"58",D,"58",1101,"58""#);
}

#[test]
#[cfg(feature = "fmt")]
fn access_formatter() {
    use cfmt_b::call_debug_fmt;

    const fn inner(f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut n = 0u64;

        try_!(writec!(f, "{0};;;", |fmt| {
            n += 1;
            call_debug_fmt!(array, [(), ()], fmt)
        }));

        try_!(writec!(f, "{0}; {0}; {0};;;", |fmt| {
            n += 100;
            call_debug_fmt!(array, [n, n], fmt)
        }));

        try_!(writec!(f, "{0};;;", |fmt| call_debug_fmt!(
            array,
            [(), ()],
            fmt
        )));

        try_!(writec!(f, "{0}; {0};;;", |fmt| call_debug_fmt!(
            array,
            [(), ()],
            fmt
        )));

        Ok(())
    }

    let writer: &mut StrWriter = &mut StrWriter::new([0; 256]);
    inner(&mut writer.make_formatter(FormattingFlags::NEW)).unwrap();

    assert_eq!(
        writer.as_str(),
        "\
            [(), ()];;;\
            [101, 101]; [201, 201]; [301, 301];;;\
            [(), ()];;;\
            [(), ()]; [(), ()];;;\
        "
    );
}
