use cfmt_a::fmt::{Error, StrWriterMut};

#[test]
fn from_custom() -> Result<(), Error> {
    let mut len = 4;
    let mut buffer = [b' '; 256];

    let mut writer = StrWriterMut::from_custom(&mut buffer, &mut len);
    assert_eq!(writer.as_bytes(), b"    ");

    writer.write_str("hello")?;
    assert_eq!(writer.as_bytes(), b"    hello");

    assert_eq!(writer.len(), 9);
    assert!(writer.buffer().starts_with(b"    hello"));

    assert_eq!(len, 9);
    assert!(buffer.starts_with(b"    hello"));

    len = 6;
    let mut writer = StrWriterMut::from_custom(&mut buffer, &mut len);

    assert_eq!(writer.len(), 6);
    assert_eq!(writer.as_bytes(), b"    he");

    writer.write_str("roic")?;
    assert_eq!(writer.len(), 10);
    assert_eq!(writer.as_bytes(), b"    heroic");
    assert!(writer.buffer().starts_with(b"    heroic"));

    assert_eq!(len, 10);
    assert!(buffer.starts_with(b"    heroic"));

    Ok(())
}

#[test]
fn from_custom_cleared() -> Result<(), Error> {
    let mut len = 4;
    let mut buffer = [b' '; 256];

    let mut writer = StrWriterMut::from_custom_cleared(&mut buffer, &mut len);
    assert_eq!(writer.as_str(), "");

    writer.write_str("hello")?;
    assert_eq!(writer.len(), 5);
    assert_eq!(writer.as_str(), "hello");
    assert!(writer.buffer().starts_with(b"hello"));

    assert_eq!(len, 5);
    assert!(buffer.starts_with(b"hello"));

    let mut writer = StrWriterMut::from_custom_cleared(&mut buffer, &mut len);

    assert!(writer.is_empty());
    assert_eq!(writer.len(), 0);
    assert_eq!(writer.as_str(), "");

    writer.write_str("he")?;
    assert!(!writer.is_empty());
    assert_eq!(writer.len(), 2);
    assert_eq!(writer.as_str(), "he");
    assert!(writer.buffer().starts_with(b"he"));

    writer.write_str("roic")?;
    assert!(!writer.is_empty());
    assert_eq!(writer.len(), 6);
    assert_eq!(writer.as_str(), "heroic");
    assert!(writer.buffer().starts_with(b"heroic"));

    assert_eq!(len, 6);
    assert!(buffer.starts_with(b"heroic"));

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

#[test]
fn clear_test() {
    const CAP: usize = 512;
    let mut buffer = [0; CAP];
    let mut len = 0;
    let mut writer = StrWriterMut::from_custom_cleared(&mut buffer, &mut len);

    writer.write_str("hello").unwrap();
    assert_eq!(writer.as_str(), "hello");

    writer.write_str("world").unwrap();
    assert_eq!(writer.as_str(), "helloworld");

    writer.clear();
    assert_eq!(writer.as_str(), "");
    assert_eq!(writer.len(), 0);

    assert_eq!(len, 0);

    let mut writer = StrWriterMut::from_custom_cleared(&mut buffer, &mut len);

    writer.write_str("foo").unwrap();
    assert_eq!(writer.as_str(), "foo");

    writer.write_str("bar").unwrap();
    assert_eq!(writer.as_str(), "foobar");
}

#[test]
fn as_bytes() {
    const CAP: usize = 512;
    let mut buffer = [0; CAP];
    let mut len = 0;
    let mut writer = StrWriterMut::from_custom_cleared(&mut buffer, &mut len);
    let mut string = String::new();

    for i in 0..CAP {
        assert_eq!(writer.capacity(), CAP);
        assert_eq!(writer.remaining_capacity(), CAP - i);

        let ascii_char = ((i % 61) + 32) as u8;
        writer.write_ascii_repeated(ascii_char, 1).unwrap();
        string.push(ascii_char as char);

        assert_eq!(writer.as_bytes(), string.as_bytes());
        assert_eq!(writer.as_bytes_alt(), string.as_bytes());
        assert_eq!(writer.as_str(), string.as_str());

        assert_eq!(writer.remaining_capacity(), CAP - i - 1);
    }
}
