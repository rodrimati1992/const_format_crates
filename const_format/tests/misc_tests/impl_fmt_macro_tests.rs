#![allow(non_camel_case_types)]

use cfmt_b::{
    fmt::{ComputeStrLength, Error, Formatter, FormattingFlags, StrWriter, StrWriterMut},
    impl_fmt, try_,
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

impl_fmt! {
    impl[] BracedStruct<u32>;

    #[allow(dead_code)]
    impl[] BracedStruct<u64>;

    const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut f = f.debug_struct("BracedStruct");
        try_!(PWrapper(self.a).const_debug_fmt(f.field("a")));
        try_!(PWrapper(self.b).const_debug_fmt(f.field("b")));
        try_!(self.c.const_debug_fmt(f.field("c")));
        try_!(self.d.const_debug_fmt(f.field("d")));
        f.finish()
    }
}

impl_fmt! {
    #[allow(dead_code)]
    impl[] TupleStruct<u64>;

    impl[] TupleStruct<u32>;

    const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut f = f.debug_tuple("TupleStruct");
        try_!(PWrapper(self.0).const_debug_fmt(f.field()));
        try_!(PWrapper(self.1).const_debug_fmt(f.field()));
        f.finish()
    }
}

impl_fmt! {
    impl[] UnitStruct;

    const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.debug_tuple("UnitStruct").finish()
    }
}

macro_rules! declare_test_case_fns {
    ( $Ty:ty ) => {
        impl_fmt! {
            impl[] Delegating<&$Ty>;

            const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
                self.0.const_debug_fmt(f)
            }
        }

        const fn inner_delegating(
            this: Delegating<&$Ty>,
            writer: &mut StrWriterMut<'_>,
            flags: FormattingFlags,
        ) -> Result<usize, Error> {
            try_!(this.const_debug_fmt(&mut writer.make_formatter(flags)));

            let mut str_len = ComputeStrLength::new();
            try_!(this.const_debug_fmt(&mut str_len.make_formatter(flags)));

            Ok(str_len.len())
        }

        const fn inner(
            this: &$Ty,
            writer: &mut StrWriterMut<'_>,
            flags: FormattingFlags,
        ) -> Result<usize, Error> {
            try_!(this.const_debug_fmt(&mut writer.make_formatter(flags)));

            let mut str_len = ComputeStrLength::new();
            try_!(this.const_debug_fmt(&mut str_len.make_formatter(flags)));

            Ok(str_len.len())
        }

        fn test_case(this: &$Ty, writer: &mut StrWriter, flags: FormattingFlags, expected: &str) {
            let writer = &mut writer.as_mut();
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
        FormattingFlags::NEW.set_alternate(true).set_hexadecimal(),
        ALT_HEX,
    );
}

#[allow(dead_code)]
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

impl_fmt! {
    impl BracedStructNE<u32>;

    #[allow(dead_code)]
    impl[] BracedStructNE<u64>;

    const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut f = f.debug_struct("BracedStructNE");
        try_!(PWrapper(self.a).const_debug_fmt(f.field("a")));
        try_!(PWrapper(self.b).const_debug_fmt(f.field("b")));
        try_!(self.c.const_debug_fmt(f.field("c")));
        try_!(self.d.const_debug_fmt(f.field("d")));
        f.finish()
    }
}

impl_fmt! {
    #[allow(dead_code)]
    impl[] TupleStructNE<u64, UnDebug>;

    impl[T,] TupleStructNE<u32, T>
    where[ T: 'static, ];

    const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut f = f.debug_tuple("TupleStructNE");
        try_!(PWrapper(self.0).const_debug_fmt(f.field()));
        try_!(PWrapper(self.1).const_debug_fmt(f.field()));
        f.finish()
    }
}

#[test]
fn struct_nonexhaustive_debug_impl() {
    declare_test_case_fns!(BracedStructNE<u32>);

    let foo = BracedStructNE {
        a: 10u32,
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

impl_fmt! {
    impl[] EnumA<u32>;

    #[allow(dead_code)]
    impl[] EnumA<u64>;

    const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Self::Tupled(f0, f1)=>{
                let mut f = f.debug_tuple("Tupled");
                try_!(PWrapper(*f0).const_debug_fmt(f.field()));
                try_!(PWrapper(*f1).const_debug_fmt(f.field()));
                f.finish()
            }
            Self::Braced {a,b,c,d} => {
                let mut f = f.debug_struct("Braced");
                try_!(PWrapper(*a).const_debug_fmt(f.field("a")));
                try_!(PWrapper(*b).const_debug_fmt(f.field("b")));
                try_!(c.const_debug_fmt(f.field("c")));
                try_!(d.const_debug_fmt(f.field("d")));
                f.finish()
            }
            Self::Unit => f.debug_struct("Unit").finish()
        }
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

#[allow(dead_code)]
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

impl_fmt! {
    impl[] EnumA_NE<u32>;

    #[allow(dead_code)]
    impl[] EnumA_NE<u64>;


    const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Self::Tupled(f0, f1,..)=>{
                let mut f = f.debug_tuple("Tupled");
                try_!(PWrapper(*f0).const_debug_fmt(f.field()));
                try_!(PWrapper(*f1).const_debug_fmt(f.field()));
                f.finish()
            }
            Self::Braced {a,b,c,d,..} => {
                let mut f = f.debug_struct("Braced");
                try_!(PWrapper(*a).const_debug_fmt(f.field("a")));
                try_!(PWrapper(*b).const_debug_fmt(f.field("b")));
                try_!(c.const_debug_fmt(f.field("c")));
                try_!(d.const_debug_fmt(f.field("d")));
                f.finish()
            }
            Self::Unit => f.debug_struct("<unknown_variant>").finish()
        }
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

impl_fmt! {
    impl[] StructWE<u32>;

    #[allow(dead_code)]
    impl[] StructWE<u64>;

    const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut f = f.debug_tuple("StructWE");
        try_!(self.0.const_debug_fmt(f.field()));
        f.finish()
    }
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
