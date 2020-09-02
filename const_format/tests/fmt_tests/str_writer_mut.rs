use const_format::fmt::{Error, StrWriterMut};

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

#[test]
fn truncate_no_encoding() -> Result<(), Error> {
    let bytes = {
        let mut bytes = "foo".as_bytes().to_vec();
        // These are unicode continuation bytes,
        // ensuring that the StrWriterMut can be truncated into a continuation byte
        bytes.push(0xBE);
        bytes.push(0xBF);
        bytes
    };

    // This isn't valid unicode.
    assert!(std::str::from_utf8(&bytes).is_err());

    let mut buffer = [0; 32];
    buffer[..bytes.len()].copy_from_slice(&bytes);
    let mut len = bytes.len();
    let mut writer = StrWriterMut::from_custom(&mut buffer, &mut len);

    assert_eq!(writer.as_bytes(), b"foo\xBE\xBF");

    writer.write_str(" bar baz")?;

    let all = b"foo\xBE\xBF bar baz";

    writer.truncate(usize::MAX / 2);
    assert_eq!(writer.as_bytes(), all);

    let fbb_len = writer.len();

    writer.truncate(fbb_len + 1);
    assert_eq!(writer.as_bytes(), all);

    for truncate_to in (3..fbb_len).rev() {
        writer.truncate(truncate_to);
        assert_eq!(writer.as_bytes(), &all[..truncate_to]);
    }

    writer.write_str("ooooooo")?;
    assert_eq!(writer.as_bytes(), b"fooooooooo");

    writer.truncate(4);
    assert_eq!(writer.as_bytes(), b"fooo");

    Ok(())
}
