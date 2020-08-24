use crate::{
    fmt::{Error, Formatter, FormattingLength},
    marker_traits::type_kind,
};

/// An example of a type that implements the `const_*_len` mehtods wrong.
pub struct ErroneousFmt;

impl type_kind::GetTypeKind for ErroneousFmt {
    type Kind = type_kind::IsNotStdKind;
    type This = Self;
}

impl ErroneousFmt {
    pub const fn const_display_len(&self, f: &mut FormattingLength<'_>) {
        f.add_len(5);
    }

    pub const fn const_display_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.w().write_whole_str("hello world")
    }

    pub const fn const_debug_len(&self, f: &mut FormattingLength<'_>) {
        f.add_len(5);
    }

    pub const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.w().write_whole_str("hello world")
    }
}
