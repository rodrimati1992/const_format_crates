use crate::{
    fmt::{ComputeStrLength, Error, Formatter, FormattingFlags, StrWriter},
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
            flag.set_hexadecimal_mode(),
            "3  --\n5\n--  false  F",
        );
        test_case(
            &this,
            writer,
            flag.set_binary_mode(),
            "3  --\n5\n--  false  1111",
        );

        let altflag = FormattingFlags::NEW.set_alternate(true);
        test_case(&this, writer, altflag, "3  --\n5\n--  false  15");
        test_case(
            &this,
            writer,
            altflag.set_hexadecimal_mode(),
            "3  --\n5\n--  false  0xF",
        );
        test_case(
            &this,
            writer,
            altflag.set_binary_mode(),
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
            flag.set_hexadecimal_mode(),
            "3  --\n5\n--  true  FE",
        );
        test_case(
            &this,
            writer,
            flag.set_binary_mode(),
            "3  --\n5\n--  true  11111110",
        );

        let altflag = FormattingFlags::NEW.set_alternate(true);
        test_case(&this, writer, altflag, "3  --\n5\n--  true  -2");
        test_case(
            &this,
            writer,
            altflag.set_hexadecimal_mode(),
            "3  --\n5\n--  true  0xFE",
        );
        test_case(
            &this,
            writer,
            altflag.set_binary_mode(),
            "3  --\n5\n--  true  0b11111110",
        );
    }
}
