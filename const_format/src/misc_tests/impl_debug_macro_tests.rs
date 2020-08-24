use crate::{
    fmt::{Error, FormattingFlags, FormattingLength, StrWriter},
    wrapper_types::PWrapper,
};

use arrayvec::ArrayString;

use core::{fmt::Write, marker::PhantomData};

////////////////////////////////////////////////////////////////////////////////

struct Delegating<T>(T);

////////////////////////////////////////////////////////////////////////////////

struct BracedStruct<T: 'static> {
    a: T,
    b: &'static [T],
    c: TupleStruct<T>,
    d: UnitStruct,
}
struct TupleStruct<T>(T, u32);

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

impl_debug! {
    #[allow(dead_code)]
    impl[] TupleStruct<u64>;

    impl[] TupleStruct<u32>;

    struct TupleStruct (
        a => PWrapper(*a),
        b => PWrapper(*b),
    )
}

impl_debug! {
    impl[] UnitStruct;

    struct UnitStruct
}

macro_rules! declare_test_case_fns {
    ( $Ty:ty ) => {
        impl_debug! {
            impl[] Delegating<&$Ty>;

            delegating = |x| x.0
        }

        const fn inner_delegating(
            this: Delegating<&$Ty>,
            writer: &mut StrWriter,
            flags: FormattingFlags,
        ) -> Result<usize, Error> {
            try_!(this.const_debug_fmt(&mut writer.make_formatter(flags)));

            let mut fmt_len = FormattingLength::new(flags);
            this.const_debug_len(&mut fmt_len);

            Ok(fmt_len.len())
        }

        const fn inner(
            this: &$Ty,
            writer: &mut StrWriter,
            flags: FormattingFlags,
        ) -> Result<usize, Error> {
            try_!(this.const_debug_fmt(&mut writer.make_formatter(flags)));

            let mut fmt_len = FormattingLength::new(flags);
            this.const_debug_len(&mut fmt_len);

            Ok(fmt_len.len())
        }

        fn test_case(this: &$Ty, writer: &mut StrWriter, flags: FormattingFlags, expected: &str) {
            {
                writer.clear();
                let len = inner(this, writer, flags).unwrap();

                assert_eq!(writer.as_str(), expected);
                assert_eq!(writer.len(), len, "{}", writer.as_str());
            }
            {
                writer.clear();
                let len = inner_delegating(Delegating(this), writer, flags).unwrap();

                assert_eq!(writer.as_str(), expected);
                assert_eq!(writer.len(), len, "{}", writer.as_str());
            }
        }
    };
}

#[test]
fn struct_debug_impl() {
    declare_test_case_fns!(BracedStruct<u32>);

    let foo = BracedStruct {
        a: 10,
        b: &[20, 30],
        c: TupleStruct(40, 50),
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

struct BracedStructNE<T: 'static> {
    a: T,
    b: &'static [T],
    c: TupleStructNE<T, UnDebug>,
    d: UnitStruct,
    e: u8,
    f: (),
}

struct UnDebug;

struct TupleStructNE<T, U>(T, u32, PhantomData<U>);

impl_debug! {
    impl[] BracedStructNE<u32>;

    #[allow(dead_code)]
    impl[] BracedStructNE<u64>;

    struct BracedStructNE {
        a => PWrapper(*a),
        b: hello => PWrapper(*hello),
        c: world,
        d,
        ..
    }
}

impl_debug! {
    #[allow(dead_code)]
    impl[] TupleStructNE<u64, UnDebug>;

    impl[] TupleStructNE<u32, UnDebug>;

    struct TupleStructNE (
        a => PWrapper(*a),
        b => PWrapper(*b),
        ..
    )
}

#[test]
fn struct_nonexhaustive_debug_impl() {
    declare_test_case_fns!(BracedStructNE<u32>);

    let foo = BracedStructNE {
        a: 10,
        b: &[20, 30],
        c: TupleStructNE(40, 50, PhantomData),
        d: UnitStruct,
        e: 0,
        f: (),
    };

    let writer: &mut StrWriter = &mut StrWriter::new([0; 512]);

    test_case(
        &foo,
        writer,
        FormattingFlags::NEW.set_alternate(false),
        "BracedStructNE { a: 10, b: [20, 30], c: TupleStructNE(40, 50), d: UnitStruct }",
    );
}

////////////////////////////////////////////////////////////////////////////////

enum EnumA<T> {
    Tupled(T, u8),
    Braced {
        a: u16,
        b: u16,
        c: UnitStruct,
        d: UnitStruct,
    },
    Unit,
}

impl_debug! {
    impl[] EnumA<u32>;

    #[allow(dead_code)]
    impl[] EnumA<u64>;

    enum EnumA {
        Tupled(
            a => PWrapper(*a),
            b => PWrapper(*b),
        ),
        Braced{
            a => PWrapper(*a),
            b: bb => PWrapper(*bb),
            c,
            d: cc,
        },
        Unit,
    }
}

#[test]
fn enum_debug_impl() {
    declare_test_case_fns!(EnumA<u32>);

    let writer: &mut StrWriter = &mut StrWriter::new([0; 512]);

    {
        let tupled = EnumA::Tupled(3, 5);

        test_case(
            &tupled,
            writer,
            FormattingFlags::NEW.set_alternate(false),
            "Tupled(3, 5)",
        );

        test_case(
            &tupled,
            writer,
            FormattingFlags::NEW.set_alternate(true),
            "Tupled(\n    3,\n    5,\n)",
        );
    }
    {
        let braced = EnumA::Braced {
            a: 8,
            b: 13,
            c: UnitStruct,
            d: UnitStruct,
        };

        test_case(
            &braced,
            writer,
            FormattingFlags::NEW.set_alternate(false),
            "Braced { a: 8, b: 13, c: UnitStruct, d: UnitStruct }",
        );

        test_case(
            &braced,
            writer,
            FormattingFlags::NEW.set_alternate(true),
            "Braced {\n    a: 8,\n    b: 13,\n    c: UnitStruct,\n    d: UnitStruct,\n}",
        );
    }
    {
        let unit = EnumA::Unit;

        test_case(
            &unit,
            writer,
            FormattingFlags::NEW.set_alternate(false),
            "Unit",
        );

        test_case(
            &unit,
            writer,
            FormattingFlags::NEW.set_alternate(true),
            "Unit",
        );
    }
}

enum EnumA_NE<T> {
    Tupled(T, u8, ()),
    Braced {
        a: u16,
        b: u16,
        c: UnitStruct,
        d: UnitStruct,
        e: (),
        f: (),
    },
    Unit,
}

impl_debug! {
    impl[] EnumA_NE<u32>;

    #[allow(dead_code)]
    impl[] EnumA_NE<u64>;

    enum EnumA_NE {
        Tupled(
            a => PWrapper(*a),
            b => PWrapper(*b),
            ..
        ),
        Braced{
            a => PWrapper(*a),
            b: bb => PWrapper(*bb),
            c,
            d: cc,
            ..
        },
        ..
    }
}

#[test]
fn enum_nonexhaustive_debug_impl() {
    declare_test_case_fns!(EnumA_NE<u32>);

    let writer: &mut StrWriter = &mut StrWriter::new([0; 512]);

    {
        let tupled = EnumA_NE::Tupled(3, 5, ());

        test_case(&tupled, writer, FormattingFlags::NEW, "Tupled(3, 5)");
    }
    {
        let braced = EnumA_NE::Braced {
            a: 8,
            b: 13,
            c: UnitStruct,
            d: UnitStruct,
            e: (),
            f: (),
        };

        test_case(
            &braced,
            writer,
            FormattingFlags::NEW,
            "Braced { a: 8, b: 13, c: UnitStruct, d: UnitStruct }",
        );
    }
    {
        let braced = EnumA_NE::Unit;

        test_case(&braced, writer, FormattingFlags::NEW, "<unknown_variant>");
    }
}

////////////////////////////////////////////////////////////////////////////////

struct StructWE<T>(EnumA<T>);

impl_debug! {
    impl[] StructWE<u32>;

    #[allow(dead_code)]
    impl[] StructWE<u64>;

    struct StructWE( e )
}

#[test]
fn enum_inside_struct() {
    declare_test_case_fns!(StructWE<u32>);

    let writer: &mut StrWriter = &mut StrWriter::new([0; 512]);
    let mut string = ArrayString::<[u8; 512]>::new();

    {
        let tupled = StructWE(EnumA::Tupled(3, 5));

        test_case(
            &tupled,
            writer,
            FormattingFlags::NEW.set_alternate(false),
            "StructWE(Tupled(3, 5))",
        );

        string.clear();
        write!(
            string,
            "StructWE({NL4}Tupled({NL8}3,{NL8}5,{NL4}),\n)",
            NL4 = "\n    ",
            NL8 = "\n        ",
        )
        .unwrap();

        test_case(
            &tupled,
            writer,
            FormattingFlags::NEW.set_alternate(true),
            string.as_str(),
        );
    }
}
