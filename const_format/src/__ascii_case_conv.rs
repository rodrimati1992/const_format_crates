mod word_iterator;

/// The casing style of a string.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Case {
    /// Uppercase
    Upper,
    /// Lowercase
    Lower,
}

pub const fn size_after_conversion(case: Case, s: &str) -> usize {
    match case {
        Case::Upper | Case::Lower => s.len(),
    }
}

pub const fn convert_str<const N: usize>(case: Case, s: &str) -> [u8; N] {
    let mut arr = [0; N];
    let mut inp = s.as_bytes();
    let mut o = 0;

    macro_rules! map_byte {
        ($byte:ident => $e:expr) => {
            while let [$byte, rem @ ..] = inp {
                let $byte = *$byte;
                inp = rem;
                arr[o] = if $byte < 128 { $e } else { $byte };
                o += 1;
            }
        };
    }

    match case {
        Case::Upper => map_byte!(b => uppercase_u8(b)),
        Case::Lower => map_byte!(b => lowercase_u8(b)),
    }

    arr
}

const CASE_DIFF: u8 = b'a' - b'A';

const fn uppercase_u8(b: u8) -> u8 {
    if let b'a'..=b'z' = b {
        b - CASE_DIFF
    } else {
        b
    }
}

const fn lowercase_u8(b: u8) -> u8 {
    if let b'A'..=b'Z' = b {
        b + CASE_DIFF
    } else {
        b
    }
}
