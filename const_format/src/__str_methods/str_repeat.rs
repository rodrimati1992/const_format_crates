pub struct StrRepeatArgs {
    pub str: &'static str,
    pub str_len: usize,
    pub out_len: usize,
    pub overflowed_len: Option<usize>,
    pub repeat: usize,
}

pub const fn StrRepeatArgs(str: &'static str, repeat: usize) -> StrRepeatArgs {
    let str_len = str.len();
    let (mul, overflowed) = str_len.overflowing_mul(repeat);

    let (out_len, overflowed_len, repeat) = if overflowed {
        (str_len, Some(mul), 1)
    } else {
        (mul, None, repeat)
    };

    StrRepeatArgs {
        str,
        str_len,
        out_len,
        overflowed_len,
        repeat,
    }
}
