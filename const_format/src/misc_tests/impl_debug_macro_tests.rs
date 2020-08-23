use crate::fmt::{Error, FormattingFlags, FormattingLength, StrWriter};
use crate::wrapper_types::PWrapper;

use core::marker::PhantomData;

struct BracedStruct<T: 'static> {
    a: T,
    b: &'static [T],
    c: TupleStruct<T, UnDebug>,
    d: UnitStruct,
}

struct TupleStruct<T, U>(T, u32, PhantomData<U>);

struct UnitStruct;

impl_debug! {
    impl[] BracedStruct<u32>;

    #[allow(dead_code)]
    impl[] BracedStruct<u64>;

    struct BracedStruct {
        a => PWrapper(*a),
        b: hello => PWrapper(*hello),
        c: world,
        d,
    }
}

struct UnDebug;

impl_debug! {
    #[allow(dead_code)]
    impl[] TupleStruct<u64, UnDebug>;

    impl[] TupleStruct<u32, UnDebug>;

    struct TupleStruct (
        a => PWrapper(*a),
        b => PWrapper(*b),
    )
}

impl_debug! {
    impl[] UnitStruct;

    struct UnitStruct{}
}

#[test]
fn struct_debug_impl() {
    const fn inner(
        this: &BracedStruct<u32>,
        writer: &mut StrWriter,
        flags: FormattingFlags,
    ) -> Result<usize, Error> {
        try_!(this.const_debug_fmt(&mut writer.make_formatter(flags)));

        let mut fmt_len = FormattingLength::new(flags);
        this.const_debug_len(&mut fmt_len);

        Ok(fmt_len.len())
    }

    fn test_case(
        this: &BracedStruct<u32>,
        writer: &mut StrWriter,
        flags: FormattingFlags,
        expected: &str,
    ) {
        writer.clear();
        let len = inner(this, writer, flags).unwrap();

        assert_eq!(writer.as_str(), expected);
        assert_eq!(writer.len(), len, "{}", writer.as_str());
    }

    let foo = BracedStruct {
        a: 10,
        b: &[20, 30],
        c: TupleStruct(40, 50, PhantomData),
        d: UnitStruct,
    };

    let writer: &mut StrWriter = &mut StrWriter::new([0; 512]);

    test_case(
        &foo,
        writer,
        FormattingFlags::NEW.set_alternate(false),
        "BracedStruct { a: 10, b: [20, 30], c: TupleStruct(40, 50), d: UnitStruct }",
    );

    const ALT: &str = "\
BracedStruct {
    a: 10,
    b: [
        20,
        30,
    ],
    c: TupleStruct(
        40,
        50,
    ),
    d: UnitStruct,
}\
    ";

    test_case(&foo, writer, FormattingFlags::NEW.set_alternate(true), ALT);

    const ALT_HEX: &str = "\
BracedStruct {
    a: 0xA,
    b: [
        0x14,
        0x1E,
    ],
    c: TupleStruct(
        0x28,
        0x32,
    ),
    d: UnitStruct,
}\
    ";

    test_case(
        &foo,
        writer,
        FormattingFlags::NEW
            .set_alternate(true)
            .set_hexadecimal_mode(),
        ALT_HEX,
    );
}
