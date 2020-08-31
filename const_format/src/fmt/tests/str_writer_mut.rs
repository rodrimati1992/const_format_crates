#[allow(unused_imports)]
use crate::{
    fmt::str_writer_mut::saturate_range,
    fmt::{Error, FormattingFlags, NumberFormatting, StrWriter, StrWriterMut},
    formatting::Formatting,
    test_utils::{RngExt, ALL_ASCII, ALL_ASCII_ESCAPED},
    wrapper_types::{AsciiStr, PWrapper},
};

pub trait Foo {
    fn as_str() -> u64 {
        0xDEAD
    }
}

impl<'w, E> Foo for StrWriterMut<'w, E> {}

#[test]
fn from_custom() -> Result<(), Error> {
    let mut len = 4;
    let mut buffer = [b' '; 256];

    let mut writer = StrWriterMut::from_custom(&mut buffer, &mut len);
    assert_eq!(writer.as_bytes(), b"    ");

    writer.write_str("hello")?;
    assert_eq!(writer.as_bytes(), b"    hello");

    assert_eq!(len, 9);
    assert!(buffer.starts_with(b"    hello"));

    len = 6;
    let mut writer = StrWriterMut::from_custom(&mut buffer, &mut len);

    assert_eq!(writer.as_bytes(), b"    he");
    writer.write_str("roic")?;
    assert_eq!(writer.as_bytes(), b"    heroic");

    Ok(())
}

#[test]
fn from_custom_cleared() -> Result<(), Error> {
    let mut len = 4;
    let mut buffer = [b' '; 256];

    let mut writer = StrWriterMut::from_custom_cleared(&mut buffer, &mut len);
    assert_eq!(writer.as_str(), "");

    writer.write_str("hello")?;
    assert_eq!(writer.as_str(), "hello");

    let mut writer = StrWriterMut::from_custom_cleared(&mut buffer, &mut len);

    assert_eq!(writer.as_str(), "");

    writer.write_str("he")?;
    assert_eq!(writer.as_str(), "he");

    writer.write_str("roic")?;
    assert_eq!(writer.as_str(), "heroic");

    Ok(())
}
