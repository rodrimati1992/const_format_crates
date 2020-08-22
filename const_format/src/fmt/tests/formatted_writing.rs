use crate::{
    fmt::{Error, Formatter, FormattingFlags, FormattingMode, StrWriter},
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
        let mut f = try_!(f.debug_struct("Foo"));
        try_!(PWrapper(self.x).const_debug_fmt(try_!(f.field("x"))));
        try_!(PWrapper(self.y).const_debug_fmt(try_!(f.field("y"))));
        try_!(PWrapper(self.z).const_debug_fmt(try_!(f.field("z"))));
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
        let mut f = try_!(f.debug_struct("Bar"));
        try_!(PWrapper(self.x).const_debug_fmt(try_!(f.field("x"))));
        try_!(PWrapper(self.y).const_debug_fmt(try_!(f.field("y"))));
        try_!(PWrapper::slice(&self.z).const_debug_fmt(try_!(f.field("z"))));
        try_!(self.foo.const_debug_fmt(try_!(f.field("foo"))));
        f.finish()
    }
}

#[test]
fn check_debug_formatting() {
    // Ensuring that all of this code can run at compile-time
    const fn inner(bar: &Bar, writer: &mut StrWriter, flags: FormattingFlags) -> Result<(), Error> {
        try_!(bar.const_debug_fmt(&mut writer.make_formatter(flags)));

        Ok(())
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
    {
        writer.clear();
        inner(&bar, writer, flags.set_alternate(false)).unwrap();

        assert_eq!(
            writer.as_str(),
            "\
                Bar { \
                    x: 21, y: \"foo\\tbar\", z: [34, 55, 89, 144, 233], \
                    foo: Foo { x: 100, y: \"hello\\nworld\", z: [3, 5, 8, 13] } \
                }\
            ",
        );
    }
    // hexadecimal + not alternate
    {
        writer.clear();
        inner(
            &bar,
            writer,
            flags.set_alternate(false).set_hexadecimal_mode(),
        )
        .unwrap();

        assert_eq!(
            writer.as_str(),
            "\
                Bar { \
                    x: 15, y: \"foo\\tbar\", z: [22, 37, 59, 90, E9], \
                    foo: Foo { x: 64, y: \"hello\\nworld\", z: [3, 5, 8, D] } \
                }\
            ",
        );
    }
    // binary + not alternate
    {
        writer.clear();
        inner(&bar, writer, flags.set_alternate(false).set_binary_mode()).unwrap();

        assert_eq!(
            writer.as_str(),
            "\
                Bar { \
                    x: 10101, y: \"foo\\tbar\", z: [100010, 110111, 1011001, 10010000, 11101001], \
                    foo: Foo { x: 1100100, y: \"hello\\nworld\", z: [11, 101, 1000, 1101] } \
                }\
            ",
        );
    }

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

    {
        writer.clear();
        inner(&bar, writer, flags.set_alternate(true)).unwrap();

        assert_eq!(writer.as_str(), ALTERNATE, "\n\n{}\n\n", writer.as_str());
    }
    {
        writer.clear();
        inner(
            &bar,
            writer,
            flags.set_alternate(true).set_hexadecimal_mode(),
        )
        .unwrap();

        assert_eq!(
            writer.as_str(),
            ALTERNATE_HEX,
            "\n\n{}\n\n",
            writer.as_str()
        );
    }

    {
        writer.clear();
        inner(&bar, writer, flags.set_alternate(true).set_binary_mode()).unwrap();

        assert_eq!(
            writer.as_str(),
            ALTERNATE_BINARY,
            "\n\n{}\n\n",
            writer.as_str()
        );
    }
}
