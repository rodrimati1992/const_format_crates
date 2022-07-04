use super::AsciiByte;

pub struct SplitInputConv<T>(pub &'static str, pub T);

impl SplitInputConv<u8> {
    pub const fn conv(self) -> SplitInput {
        SplitInput {
            str: self.0,
            pattern: SplitPattern::AsciiByte(AsciiByte::new(self.1)),
            length: usize::MAX,
        }
        .compute_length()
    }
}

impl SplitInputConv<&'static str> {
    pub const fn conv(self) -> SplitInput {
        let str = self.1;

        let pattern = if let [b @ 0..=127] = *str.as_bytes() {
            SplitPattern::AsciiByte(AsciiByte::new(b))
        } else {
            SplitPattern::Str(str)
        };

        SplitInput {
            str: self.0,
            pattern,
            length: usize::MAX,
        }
        .compute_length()
    }
}

#[derive(Copy, Clone)]
pub struct SplitInput {
    str: &'static str,
    pattern: SplitPattern,
    length: usize,
}

impl SplitInput {
    const fn compute_length(mut self) -> Self {
        self.length = count_splits(self);
        self
    }

    pub const fn split_it<const LEN: usize>(self) -> [&'static str; LEN] {
        split_it(self)
    }

    pub const fn length(&self) -> usize {
        self.length
    }
}

#[derive(Copy, Clone)]
pub enum SplitPattern {
    AsciiByte(AsciiByte),
    Str(&'static str),
}

pub const fn count_splits(
    SplitInput {
        mut str, pattern, ..
    }: SplitInput,
) -> usize {
    let mut count = 1;

    match pattern {
        SplitPattern::AsciiByte(ascii_c) => {
            let mut bytes = str.as_bytes();
            let ascii_c = ascii_c.get();

            while let [byte, rem @ ..] = bytes {
                bytes = rem;

                if *byte == ascii_c {
                    count += 1;
                }
            }
        }
        SplitPattern::Str(str_pat) => {
            if str_pat.is_empty() {
                let mut char_i = 0;
                count += 1;
                while let Some(next) = find_next_char_boundary(str, char_i) {
                    char_i = next;
                    count += 1;
                }
            } else {
                while let Some(next) = konst::string::find_skip(str, str_pat) {
                    str = next;
                    count += 1;
                }
            }
        }
    }

    count
}

const fn find_u8(mut slice: &[u8], byte: u8) -> Option<usize> {
    let mut i = 0;

    while let [b, ref rem @ ..] = *slice {
        if byte == b {
            return Some(i);
        }
        slice = rem;
        i += 1;
    }
    None
}

const fn find_next_char_boundary(str: &str, mut index: usize) -> Option<usize> {
    if index == str.len() {
        None
    } else {
        loop {
            index += 1;
            if index == str.len() || (str.as_bytes()[index] as i8) >= -0x40 {
                break Some(index);
            }
        }
    }
}

pub const fn split_it<const LEN: usize>(args: SplitInput) -> [&'static str; LEN] {
    let SplitInput {
        mut str,
        pattern,
        length: _,
    } = args;

    let mut out = [""; LEN];
    let mut out_i = 0;

    macro_rules! write_out {
        ($string:expr) => {
            out[out_i] = $string;
            out_i += 1;
        };
    }

    match pattern {
        SplitPattern::AsciiByte(ascii_c) => {
            let ascii_c = ascii_c.get();

            while let Some(found_at) = find_u8(str.as_bytes(), ascii_c) {
                write_out! {konst::string::str_up_to(str, found_at)}
                str = konst::string::str_from(str, found_at + 1);
            }
        }
        SplitPattern::Str(str_pat) => {
            if str_pat.is_empty() {
                out_i += 1;
                while let Some(next) = find_next_char_boundary(str, 0) {
                    write_out! {konst::string::str_up_to(str, next)}
                    str = konst::string::str_from(str, next);
                }
            } else {
                while let Some(found_at) = konst::string::find(str, str_pat, 0) {
                    write_out! {konst::string::str_up_to(str, found_at)}
                    str = konst::string::str_from(str, found_at + str_pat.len());
                }
            }
        }
    }

    write_out! {str}

    assert!(out_i == LEN);
    out
}
