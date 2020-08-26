// use crate::fmt::Error;

#[cfg(feature = "with_fmt")]
use crate::fmt::{Error, Formatter};

use core::fmt::{self, Display};

////////////////////////////////////////////////////////////////////////////////

/// An ascii string slice.
#[derive(Debug, Copy, Clone)]
pub struct AsciiStr<'a>(&'a [u8]);

impl<'a> AsciiStr<'a> {
    /// Constructs this  AsciiStr from a possibly non-ascii str slice.
    ///
    /// Returns a `NonAsciiError` error on the first non-ascii byte.
    #[inline(always)]
    pub const fn from_str(s: &'a str) -> Result<Self, NotAsciiError> {
        Self::new(s.as_bytes())
    }

    /// Constructs this  AsciiStr from a possibly non-ascii byte slice.
    ///
    /// Returns a `NonAsciiError` error on the first non-ascii byte.
    pub const fn new(s: &'a [u8]) -> Result<Self, NotAsciiError> {
        __for_range! {i in 0..s.len()=>
            if s[i] > 127 {
                return Err(NotAsciiError{invalid_from: i});
            }
        }
        Ok(AsciiStr(s))
    }

    #[inline(always)]
    pub const fn len(self) -> usize {
        self.0.len()
    }

    /// Accessor for the wrapped ascii string.
    #[inline(always)]
    pub const fn as_bytes(self) -> &'a [u8] {
        self.0
    }

    /// Accessor for the wrapped ascii string.
    #[inline]
    pub fn as_str(self) -> &'a str {
        unsafe { core::str::from_utf8_unchecked(self.0) }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Copy, Clone)]
pub struct NotAsciiError {
    /// The first non-ascii byte in the byte slice.
    pub invalid_from: usize,
}

// TODO
// impl NotAsciiError {
//     pub fn to_fmt_error() -> Error {
//         Error::NotAscii
//     }
// }

impl Display for NotAsciiError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        fmt.write_str("error: the input bytes were not valid ascii")
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "with_fmt")]
impl_fmt! {
    impl AsciiStr<'_>;

    pub const fn const_display_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_whole_ascii(*self)
    }

    pub const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_whole_ascii_debug(*self)
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "with_fmt")]
    use crate::fmt::ComputeStrLength;

    use arrayvec::ArrayString;

    #[test]
    fn basic() {
        {
            let ok = AsciiStr::new("hello!".as_bytes()).unwrap();
            assert_eq!(ok.as_bytes(), "hello!".as_bytes());
            assert_eq!(ok.as_str(), "hello!");
        }
        {
            let err = AsciiStr::from_str("Φοο!").unwrap_err();
            assert_eq!(err.invalid_from, 0)
        }
        {
            let err = AsciiStr::from_str("hello Φοο!").unwrap_err();
            assert_eq!(err.invalid_from, 6)
        }
    }
    #[test]
    fn only_ascii_constructible() {
        let mut string = ArrayString::<[u8; 1024]>::new();
        let min = '\u{20}';
        let max = '\u{80}';
        assert!(!max.is_ascii());
        for end in min..=max {
            for start in min..=end {
                string.clear();
                for n in start..=end {
                    string.push(n);
                }
                let res = AsciiStr::new(string.as_bytes());
                assert_eq!(res.is_ok(), string.as_bytes().is_ascii());

                if let Ok(ascii) = res {
                    assert_eq!(ascii.as_bytes(), string.as_bytes());
                }
            }
        }
    }

    #[cfg(feature = "with_fmt")]
    #[test]
    fn formatting() {
        use crate::fmt::{FormattingFlags, FormattingMode, StrWriter};

        const fn inner_debug(
            ascii: AsciiStr<'_>,
            writer: &mut StrWriter,
            flags: FormattingFlags,
        ) -> Result<usize, Error> {
            try_!(ascii.const_debug_fmt(&mut writer.make_formatter(flags)));

            let mut str_len = ComputeStrLength::new();
            try_!(ascii.const_debug_fmt(&mut str_len.make_formatter(flags)));

            Ok(str_len.len())
        }

        const fn inner_display(
            ascii: AsciiStr<'_>,
            writer: &mut StrWriter,
            flags: FormattingFlags,
        ) -> Result<usize, Error> {
            try_!(ascii.const_display_fmt(&mut writer.make_formatter(flags)));

            let mut str_len = ComputeStrLength::new();
            try_!(ascii.const_display_fmt(&mut str_len.make_formatter(flags)));

            Ok(str_len.len())
        }

        fn test_case(
            ascii: AsciiStr<'_>,
            writer: &mut StrWriter,
            flags: FormattingFlags,
            expected_debug: &str,
            expected_display: &str,
        ) {
            writer.clear();
            let len = inner_debug(ascii, writer, flags).unwrap();

            assert_eq!(writer.as_str(), expected_debug);
            assert_eq!(writer.len(), len, "{}", writer.as_str());

            writer.clear();
            let len = inner_display(ascii, writer, flags).unwrap();

            assert_eq!(writer.as_str(), expected_display);
            assert_eq!(writer.len(), len, "{}", writer.as_str());
        }

        let writer: &mut StrWriter = &mut StrWriter::new([0; 128]);

        let foo = AsciiStr::new("\0\x10hello\tworld\n".as_bytes()).unwrap();

        for mode in FormattingMode::ALL.iter().copied() {
            for is_alt in [false, true].iter().copied() {
                let flag = FormattingFlags::NEW.set_mode(mode).set_alternate(is_alt);
                test_case(
                    foo,
                    writer,
                    flag,
                    "\"\\x00\\x10hello\\tworld\\n\"",
                    foo.as_str(),
                );
            }
        }
    }
}
