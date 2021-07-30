use core::ops::{self, Range};

use crate::__hidden_utils::{max_usize, saturating_add};

pub struct StrReplaceArgsConv<T> {
    pub arg: T,
    pub str: &'static str,
    pub insert: &'static str,
}

#[allow(non_snake_case)]
pub const fn StrReplaceArgsConv<T>(
    str: &'static str,
    arg: T,
    insert: &'static str,
) -> StrReplaceArgsConv<T> {
    StrReplaceArgsConv { str, arg, insert }
}

macro_rules! define_conversions {
    (
        $( fn($self:ident, $ty:ty) $block:block )*
    ) => {
        $(
            impl StrReplaceArgsConv<$ty> {
                pub const fn conv($self) -> StrReplaceArgs {
                    let range = $block;
                    let str_len = $self.str.len();
                    let range_len = range.end - range.start;

                    StrReplaceArgs{
                        str: $self.str,
                        insert: $self.insert,
                        str_len,
                        start: range.start,
                        end: range.end,
                        range_len,
                        insert_len: $self.insert.len(),
                        suffix_len: str_len - range.end,
                        out_len: str_len - range_len + $self.insert.len(),
                    }
                }
            }
        )*
    };
}

define_conversions! {
    fn(self, usize) {
        self.arg .. saturating_add(self.arg, 1)
    }

    fn(self, ops::Range<usize>) {
        let Range{start, end} = self.arg;
        start .. max_usize(start, end)
    }

    fn(self, ops::RangeTo<usize>) {
        0..self.arg.end
    }

    fn(self, ops::RangeFrom<usize>) {
        self.arg.start..self.str.len()
    }

    fn(self, ops::RangeInclusive<usize>) {
        let start = *self.arg.start();
        start .. max_usize(saturating_add(*self.arg.end(), 1), start)
    }

    fn(self, ops::RangeToInclusive<usize>) {
        0 .. saturating_add(self.arg.end, 1)
    }

    fn(self, ops::RangeFull) {
        0 .. self.str.len()
    }
}

pub struct StrReplaceArgs {
    pub str: &'static str,
    pub insert: &'static str,
    pub str_len: usize,
    pub start: usize,
    pub end: usize,
    pub range_len: usize,
    pub insert_len: usize,
    pub suffix_len: usize,
    pub out_len: usize,
}

/// The return value of [`str_splice`](./macro.str_splice.html)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SplicedStr {
    /// A string that had `removed` replaced with some other string.
    pub output: &'static str,
    /// The part of the string that was removed.
    pub removed: &'static str,
}

#[repr(C, packed)]
pub struct DecomposedString<P, M, S> {
    pub prefix: P,
    pub middle: M,
    pub suffix: S,
}
