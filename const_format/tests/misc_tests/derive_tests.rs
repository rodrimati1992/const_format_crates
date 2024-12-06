use cfmt_b::{
    fmt::{Error, Formatter, FormattingFlags, StrWriter},
    try_, writec, ConstDebug,
};

use core::marker::PhantomData;

mod is_a_attributes;

///////////////////////////////////////////////////////////////////////////////

struct Dummy;

#[derive(ConstDebug)]
#[cdeb(crate = "::cfmt_b")]
struct Braced {
    x: u32,
    y: Option<&'static str>,
    #[allow(dead_code)]
    #[cdeb(ignore)]
    z: (u32, u32),
    a: bool,
}

#[derive(ConstDebug)]
#[cdeb(crate = "::cfmt_b")]
#[cdeb(impls(
    "<U> Tupled<u32, U>",
    "<U> Tupled<u64, U>",
    "<U> Tupled<bool, U> where U: 'static,",
))]
struct Tupled<T, U>(
    u32,
    #[cdeb(ignore)]
    #[allow(dead_code)]
    Option<&'static str>,
    T,
    PhantomData<U>,
);

#[derive(ConstDebug)]
#[cdeb(crate = "::cfmt_b")]
struct Unit;

#[test]
fn struct_formatting() {
    const fn inner(f: &mut Formatter<'_>) -> Result<(), Error> {
        let braced = Braced {
            x: 100,
            y: Some("hello"),
            z: (0, 1),
            a: true,
        };
        try_!(writec!(f, "{0:?}\n{0:x?}\n{0:X?}\n", braced));

        let tupled_a: Tupled<u32, Dummy> = Tupled(13, Some("hello"), 21, PhantomData);
        let tupled_b: Tupled<u64, Dummy> = Tupled(32, Some("hello"), 33, PhantomData);
        let tupled_c: Tupled<bool, Dummy> = Tupled(48, Some("hello"), false, PhantomData);
        try_!(writec!(
            f,
            "{0:?}\n{0:x?}\n{0:X?}\n{1:?}\n{1:x?}\n{1:X?}\n{2:?}\n{2:x?}\n{2:X?}\n",
            tupled_a,
            tupled_b,
            tupled_c,
        ));

        try_!(writec!(f, "{:?}", Unit));

        Ok(())
    }

    let writer: &mut StrWriter = &mut StrWriter::new([0; 1024]);

    inner(&mut writer.make_formatter(FormattingFlags::NEW)).unwrap();

    assert_eq!(
        writer.as_str(),
        "\
            Braced { x: 100, y: Some(\"hello\"), a: true }\n\
            Braced { x: 64, y: Some(\"hello\"), a: true }\n\
            Braced { x: 64, y: Some(\"hello\"), a: true }\n\
            Tupled(13, 21, PhantomData)\n\
            Tupled(d, 15, PhantomData)\n\
            Tupled(D, 15, PhantomData)\n\
            Tupled(32, 33, PhantomData)\n\
            Tupled(20, 21, PhantomData)\n\
            Tupled(20, 21, PhantomData)\n\
            Tupled(48, false, PhantomData)\n\
            Tupled(30, false, PhantomData)\n\
            Tupled(30, false, PhantomData)\n\
            Unit\
        ",
    );
}

///////////////////////////////////////////////////////////////////////////////

#[derive(ConstDebug)]
#[cdeb(crate = "::cfmt_b")]
enum Enum {
    Braced {
        x: u32,
        y: Option<&'static str>,
        #[allow(dead_code)]
        #[cdeb(ignore)]
        z: (u32, u32),
        a: bool,
    },
    Tupled(
        u32,
        #[cdeb(ignore)]
        #[allow(dead_code)]
        Option<&'static str>,
        u32,
        PhantomData<()>,
    ),
    Unit,
}

#[test]
fn enum_formatting() {
    const fn inner(f: &mut Formatter<'_>) -> Result<(), Error> {
        let braced = Enum::Braced {
            x: 100,
            y: Some("hello"),
            z: (0, 1),
            a: true,
        };
        try_!(writec!(f, "{0:?}\n{0:x?}\n{0:X?}\n", braced));

        let tupled_a = Enum::Tupled(13, Some("hello"), 21, PhantomData);
        let tupled_b = Enum::Tupled(32, Some("hello"), 33, PhantomData);
        try_!(writec!(
            f,
            "{0:?}\n{0:x?}\n{0:X?}\n{1:?}\n{1:x?}\n{1:X?}\n",
            tupled_a,
            tupled_b
        ));

        try_!(writec!(f, "{:?}", Enum::Unit));

        Ok(())
    }

    let writer: &mut StrWriter = &mut StrWriter::new([0; 1024]);

    inner(&mut writer.make_formatter(FormattingFlags::NEW)).unwrap();

    assert_eq!(
        writer.as_str(),
        "\
            Braced { x: 100, y: Some(\"hello\"), a: true }\n\
            Braced { x: 64, y: Some(\"hello\"), a: true }\n\
            Braced { x: 64, y: Some(\"hello\"), a: true }\n\
            Tupled(13, 21, PhantomData)\n\
            Tupled(d, 15, PhantomData)\n\
            Tupled(D, 15, PhantomData)\n\
            Tupled(32, 33, PhantomData)\n\
            Tupled(20, 21, PhantomData)\n\
            Tupled(20, 21, PhantomData)\n\
            Unit\
        ",
    );
}
