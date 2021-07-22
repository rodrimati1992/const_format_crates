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
                arr[o] = $e;
                o += 1;
            }
        };
    }

    const CASE_DIFF: u8 = b'a' - b'A';

    match case {
        Case::Upper => {
            map_byte!(b => {
                if let b'a'..=b'z' = b {
                    b - CASE_DIFF
                } else {
                    b
                }
            })
        }
        Case::Lower => {
            map_byte!(b => {
                if let b'A'..=b'Z' = b {
                    b + CASE_DIFF
                } else {
                    b
                }
            })
        }
    }

    arr
}

/// The casing style of a string.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Case {
    /// Uppercase
    Upper,
    /// Lowercase
    Lower,
}
