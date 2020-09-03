use const_format::fmt::{Error, Formatter, FormattingFlags, StrWriter};
use const_format::{coerce_to_fmt, impl_fmt};

#[test]
fn coercion() {
    let writer: &mut StrWriter = &mut StrWriter::new([0; 512]);

    let flags = FormattingFlags::NEW;

    writer.clear();
    coerce_to_fmt!(&100u8)
        .const_debug_fmt(&mut writer.make_formatter(flags))
        .unwrap();
    assert_eq!(writer.as_str(), "100");

    writer.clear();
    coerce_to_fmt!(&UnitStruct)
        .const_debug_fmt(&mut writer.make_formatter(flags))
        .unwrap();
    assert_eq!(writer.as_str(), "UnitStruct");

    writer.clear();
    let array = [0u8, 1, 2, 3];
    coerce_to_fmt!(&&&&&array)
        .const_debug_fmt(&mut writer.make_formatter(flags))
        .unwrap();
    assert_eq!(writer.as_str(), "[0, 1, 2, 3]");

    writer.clear();
    let array = [0u8, 1, 2, 3];
    coerce_to_fmt!(&&&array[..])
        .const_debug_fmt(&mut writer.make_formatter(flags))
        .unwrap();
    assert_eq!(writer.as_str(), "[0, 1, 2, 3]");
}

struct UnitStruct;

impl_fmt! {
    impl[] UnitStruct;

    const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_str("UnitStruct")
    }
}
