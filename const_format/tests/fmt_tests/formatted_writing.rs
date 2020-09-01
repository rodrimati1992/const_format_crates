use const_format::{
    __for_range,
    fmt::{ComputeStrLength, Error, Formatter, FormattingFlags, StrWriter},
    try_,
    wrapper_types::PWrapper,
};

#[derive(Debug, Copy, Clone)]
struct Foo {
    x: u32,
    y: &'static str,
    z: &'static [u8],
}

impl Foo {
    pub const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut f = f.debug_struct("Foo");
        try_!(PWrapper(self.x).const_debug_fmt(f.field("x")));
        try_!(PWrapper(self.y).const_debug_fmt(f.field("y")));
        try_!(PWrapper(self.z).const_debug_fmt(f.field("z")));
        f.finish()
    }
}

#[derive(Debug, Copy, Clone)]
struct Bar {
    x: u32,
    y: &'static str,
    z: [u8; 5],
    foo: Foo,
}

impl Bar {
    pub const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut f = f.debug_struct("Bar");
        try_!(PWrapper(self.x).const_debug_fmt(f.field("x")));
        try_!(PWrapper(self.y).const_debug_fmt(f.field("y")));
        try_!(PWrapper::slice(&self.z).const_debug_fmt(f.field("z")));
        try_!(self.foo.const_debug_fmt(f.field("foo")));
        f.finish()
    }
}

#[test]
fn check_debug_formatting() {
    // Ensuring that all of this code can run at compile-time
    const fn inner(
        bar: &Bar,
        writer: &mut StrWriter,
        flags: FormattingFlags,
    ) -> Result<usize, Error> {
        try_!(bar.const_debug_fmt(&mut writer.make_formatter(flags)));

        let mut str_len = ComputeStrLength::new();
        try_!(bar.const_debug_fmt(&mut str_len.make_formatter(flags)));

        Ok(str_len.len())
    }

    fn test_case(bar: &Bar, writer: &mut StrWriter, flags: FormattingFlags, expected: &str) {
        writer.clear();
        let len = inner(&bar, writer, flags).unwrap();

        assert_eq!(writer.as_str(), expected);
        assert_eq!(writer.len(), len, "{}", writer.as_str());
    }

    let foo = Foo {
        x: 100,
        y: "hello\nworld",
        z: &[3, 5, 8, 13],
    };

    let bar = Bar {
        x: 21,
        y: "foo\tbar",
        z: [34, 55, 89, 144, 233],
        foo,
    };

    let flags = FormattingFlags::DEFAULT;

    let writer: &mut StrWriter = &mut StrWriter::new([0; 1024]);

    // decimal + not alternate
    test_case(
        &bar,
        writer,
        flags.set_alternate(false),
        "\
            Bar { \
                x: 21, y: \"foo\\tbar\", z: [34, 55, 89, 144, 233], \
                foo: Foo { x: 100, y: \"hello\\nworld\", z: [3, 5, 8, 13] } \
            }\
        ",
    );

    // hexadecimal + not alternate
    test_case(
        &bar,
        writer,
        flags.set_alternate(false).set_hexadecimal(),
        "\
            Bar { \
                x: 15, y: \"foo\\tbar\", z: [22, 37, 59, 90, E9], \
                foo: Foo { x: 64, y: \"hello\\nworld\", z: [3, 5, 8, D] } \
            }\
        ",
    );

    test_case(
        &bar,
        writer,
        flags.set_alternate(false).set_binary(),
        "\
            Bar { \
                x: 10101, y: \"foo\\tbar\", z: [100010, 110111, 1011001, 10010000, 11101001], \
                foo: Foo { x: 1100100, y: \"hello\\nworld\", z: [11, 101, 1000, 1101] } \
            }\
        ",
    );

    const ALTERNATE: &str = "\
Bar {
    x: 21,
    y: \"foo\\tbar\",
    z: [
        34,
        55,
        89,
        144,
        233,
    ],
    foo: Foo {
        x: 100,
        y: \"hello\\nworld\",
        z: [
            3,
            5,
            8,
            13,
        ],
    },
}";
    const ALTERNATE_HEX: &str = "\
Bar {
    x: 0x15,
    y: \"foo\\tbar\",
    z: [
        0x22,
        0x37,
        0x59,
        0x90,
        0xE9,
    ],
    foo: Foo {
        x: 0x64,
        y: \"hello\\nworld\",
        z: [
            0x3,
            0x5,
            0x8,
            0xD,
        ],
    },
}";

    const ALTERNATE_BINARY: &str = "\
Bar {
    x: 0b10101,
    y: \"foo\\tbar\",
    z: [
        0b100010,
        0b110111,
        0b1011001,
        0b10010000,
        0b11101001,
    ],
    foo: Foo {
        x: 0b1100100,
        y: \"hello\\nworld\",
        z: [
            0b11,
            0b101,
            0b1000,
            0b1101,
        ],
    },
}";

    test_case(&bar, writer, flags.set_alternate(true), ALTERNATE);
    test_case(
        &bar,
        writer,
        flags.set_alternate(true).set_hexadecimal(),
        ALTERNATE_HEX,
    );
    test_case(
        &bar,
        writer,
        flags.set_alternate(true).set_binary(),
        ALTERNATE_BINARY,
    );
}

////////////////////////////////////////////////////////////////////////////////

pub struct Set(&'static [Foo]);

impl Set {
    pub const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut f = f.debug_set();
        __for_range! {i in 0..self.0.len()=>
            try_!(self.0[i].const_debug_fmt(f.entry()));
        }
        f.finish()
    }
}

#[test]
fn check_set_formatting() {
    const fn inner(
        set: &Set,
        writer: &mut StrWriter,
        flags: FormattingFlags,
    ) -> Result<usize, Error> {
        try_!(set.const_debug_fmt(&mut writer.make_formatter(flags)));

        let mut str_len = ComputeStrLength::new();
        try_!(set.const_debug_fmt(&mut str_len.make_formatter(flags)));

        Ok(str_len.len())
    }

    fn test_case(bar: &Set, writer: &mut StrWriter, flags: FormattingFlags, expected: &str) {
        writer.clear();
        let len = inner(&bar, writer, flags).unwrap();

        assert_eq!(writer.as_str(), expected);
        assert_eq!(writer.len(), len, "{}", writer.as_str());
    }

    let flags = FormattingFlags::DEFAULT;

    let writer: &mut StrWriter = &mut StrWriter::new([0; 1024]);

    let set = Set(&[Foo {
        x: 100,
        y: "hello\nworld",
        z: &[],
    }]);

    test_case(
        &set,
        writer,
        flags,
        "{Foo { x: 100, y: \"hello\\nworld\", z: [] }}",
    );
    test_case(
        &set,
        writer,
        flags.set_hexadecimal(),
        "{Foo { x: 64, y: \"hello\\nworld\", z: [] }}",
    );
    test_case(
        &set,
        writer,
        flags.set_binary(),
        "{Foo { x: 1100100, y: \"hello\\nworld\", z: [] }}",
    );

    const ALTERNATE: &str = "\
{
    Foo {
        x: 100,
        y: \"hello\\nworld\",
        z: [],
    },
}\
    ";

    const ALTERNATE_HEX: &str = "\
{
    Foo {
        x: 0x64,
        y: \"hello\\nworld\",
        z: [],
    },
}\
    ";

    const ALTERNATE_BINARY: &str = "\
{
    Foo {
        x: 0b1100100,
        y: \"hello\\nworld\",
        z: [],
    },
}\
    ";

    test_case(&set, writer, flags.set_alternate(true), ALTERNATE);
    test_case(
        &set,
        writer,
        flags.set_alternate(true).set_hexadecimal(),
        ALTERNATE_HEX,
    );
    test_case(
        &set,
        writer,
        flags.set_alternate(true).set_binary(),
        ALTERNATE_BINARY,
    );
}
