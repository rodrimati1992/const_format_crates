use const_format::{
    fmt::{ComputeStrLength, Error, Formatter, FormattingFlags, StrWriter},
    try_,
    wrapper_types::PWrapper,
};

#[derive(Copy, Clone)]
struct DisplayFoo {
    x: u32,
    y: &'static str,
    z: bool,
    debug: i8,
}

impl DisplayFoo {
    pub const fn const_display_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        try_!(PWrapper(self.x).const_display_fmt(f));
        try_!(PWrapper("  ").const_display_fmt(f));
        try_!(PWrapper(self.y).const_display_fmt(f));
        try_!(PWrapper("  ").const_display_fmt(f));
        try_!(PWrapper(self.z).const_display_fmt(f));
        try_!(PWrapper("  ").const_display_fmt(f));
        try_!(PWrapper(self.debug).const_debug_fmt(f));
        Ok(())
    }
}

const fn inner(
    this: &DisplayFoo,
    writer: &mut StrWriter,
    flags: FormattingFlags,
) -> Result<usize, Error> {
    try_!(this.const_display_fmt(&mut writer.make_formatter(flags)));

    let mut str_len = ComputeStrLength::new();
    try_!(this.const_display_fmt(&mut str_len.make_formatter(flags)));

    Ok(str_len.len())
}

fn test_case(this: &DisplayFoo, writer: &mut StrWriter, flags: FormattingFlags, expected: &str) {
    writer.clear();
    let len = inner(this, writer, flags).unwrap();

    assert_eq!(writer.as_str(), expected);
    assert_eq!(writer.len(), len, "{}", writer.as_str());
}

#[test]
fn test_display() {
    let writer: &mut StrWriter = &mut StrWriter::new([0; 1024]);

    {
        let this = DisplayFoo {
            x: 3,
            y: "--\n5\n--",
            z: false,
            debug: 15,
        };
        let flag = FormattingFlags::NEW.set_alternate(false);
        test_case(&this, writer, flag, "3  --\n5\n--  false  15");
        test_case(
            &this,
            writer,
            flag.set_hexadecimal(),
            "3  --\n5\n--  false  F",
        );
        test_case(
            &this,
            writer,
            flag.set_binary(),
            "3  --\n5\n--  false  1111",
        );

        let altflag = FormattingFlags::NEW.set_alternate(true);
        test_case(&this, writer, altflag, "3  --\n5\n--  false  15");
        test_case(
            &this,
            writer,
            altflag.set_hexadecimal(),
            "3  --\n5\n--  false  0xF",
        );
        test_case(
            &this,
            writer,
            altflag.set_binary(),
            "3  --\n5\n--  false  0b1111",
        );
    }

    {
        let this = DisplayFoo {
            x: 3,
            y: "--\n5\n--",
            z: true,
            debug: -2,
        };
        let flag = FormattingFlags::NEW.set_alternate(false);
        test_case(&this, writer, flag, "3  --\n5\n--  true  -2");
        test_case(
            &this,
            writer,
            flag.set_hexadecimal(),
            "3  --\n5\n--  true  FE",
        );
        test_case(
            &this,
            writer,
            flag.set_binary(),
            "3  --\n5\n--  true  11111110",
        );

        let altflag = FormattingFlags::NEW.set_alternate(true);
        test_case(&this, writer, altflag, "3  --\n5\n--  true  -2");
        test_case(
            &this,
            writer,
            altflag.set_hexadecimal(),
            "3  --\n5\n--  true  0xFE",
        );
        test_case(
            &this,
            writer,
            altflag.set_binary(),
            "3  --\n5\n--  true  0b11111110",
        );
    }
}

/*////////////////////////////////////////////////////////////////////////////////
Testing Display formatting for non-primitive integer or `&str` types,
because those types are already tested in other tests.
*/////////////////////////////////////////////////////////////////////////////////

use std::num::{NonZeroI8, NonZeroIsize, NonZeroU8, NonZeroUsize};

macro_rules! unwrap_opt {
    ($opt:expr) => {
        match $opt {
            Some(x) => x,
            None => loop {},
        }
    };
}

#[test]
fn display_fmt_other_types() {
    const fn inner(fmt: &mut Formatter<'_>) -> const_format::Result {
        const_format::writec!(
            fmt,
            concat!("{},{};", "{},{};{},{};{},{};{},{};",),
            false,
            true,
            unwrap_opt!(NonZeroI8::new(-13)),
            unwrap_opt!(NonZeroI8::new(21)),
            unwrap_opt!(NonZeroIsize::new(-13)),
            unwrap_opt!(NonZeroIsize::new(21)),
            unwrap_opt!(NonZeroU8::new(3)),
            unwrap_opt!(NonZeroU8::new(13)),
            unwrap_opt!(NonZeroUsize::new(3)),
            unwrap_opt!(NonZeroUsize::new(13)),
        )
    }

    let flags = [
        FormattingFlags::NEW.set_alternate(false).set_decimal(),
        FormattingFlags::NEW.set_alternate(false).set_hexadecimal(),
        FormattingFlags::NEW.set_alternate(false).set_binary(),
        FormattingFlags::NEW.set_alternate(true).set_decimal(),
        FormattingFlags::NEW.set_alternate(true).set_hexadecimal(),
        FormattingFlags::NEW.set_alternate(true).set_binary(),
    ];

    let writer: &mut StrWriter = &mut StrWriter::new([0; 1024]);
    for flag in flags.iter().copied() {
        writer.clear();

        inner(&mut writer.make_formatter(flag)).unwrap();

        assert_eq!(writer.as_str(), "false,true;-13,21;-13,21;3,13;3,13;");
    }
}
