use crate::fmt::FormattingFlags;

pub struct FormattingLength {
    flags: FormattingFlags,
    len: usize,
}

impl FormattingLength {
    pub const fn new(flags: FormattingFlags) -> Self {
        Self { flags, len: 0 }
    }

    #[inline(always)]
    pub const fn flags(&self) -> FormattingFlags {
        self.flags
    }

    #[inline(always)]
    pub const fn add_len(&mut self, len: usize) -> &mut Self {
        self.len += len;
        self
    }

    #[inline(always)]
    pub const fn len(&self) -> usize {
        self.len
    }

    #[inline(always)]
    const fn increment_margin(&mut self) -> &mut Self {
        self.flags = self.flags.increment_margin();
        self
    }

    #[inline]
    pub const fn debug_struct(&mut self, name: &str) -> DebugStructLength<'_> {
        DebugStructLength {
            fmt: self.increment_margin().add_len(name.len()),
            wrote_field: false,
        }
    }

    #[inline]
    pub const fn debug_tuple(&mut self, name: &str) -> DebugTupleLength<'_> {
        DebugTupleLength {
            fmt: self.increment_margin().add_len(name.len()),
            wrote_field: false,
        }
    }

    #[inline]
    pub const fn debug_list(&mut self) -> DebugListLength<'_> {
        DebugListLength {
            fmt: self.increment_margin(),
            wrote_field: false,
        }
    }

    #[inline]
    pub const fn debug_set(&mut self) -> DebugSetLength<'_> {
        DebugSetLength {
            fmt: self.increment_margin(),
            wrote_field: false,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

const COMMA_SPACE_LEN: usize = ", ".len();
const COMMA_NL_LEN: usize = ",\n".len();

macro_rules! field_method_impl {
    ($
        self: ident, $open_space:expr, $open_newline:expr;
        $($write_name:tt)*
    ) => ({
        const OPEN_SPACE: usize = $open_space.len();
        const OPEN_NEWLINE: usize = $open_newline.len();

        let is_alternate = $self.fmt.flags.is_alternate();
        $self.fmt.add_len(match ($self.wrote_field, is_alternate) {
            (false, false) => OPEN_SPACE,
            (false, true) => OPEN_NEWLINE + $self.fmt.flags.margin(),
            (true , false) => COMMA_SPACE_LEN,
            (true , true) => COMMA_NL_LEN + $self.fmt.flags.margin(),
        });
        $($write_name)*
        $self.wrote_field = true;

        $self.fmt
    })
}

macro_rules! finish_method_impl {
    ($self: ident, $close_token:expr, $space_close:expr) => {{
        const CLOSE_TOKEN: usize = $close_token.len();
        const SPACE_CLOSE: usize = $space_close.len();

        $self.fmt.flags = $self.fmt.flags.decrement_margin();
        if $self.wrote_field {
            if $self.fmt.flags.is_alternate() {
                $self
                    .fmt
                    .add_len(COMMA_NL_LEN + $self.fmt.flags.margin() + CLOSE_TOKEN);
            } else {
                $self.fmt.add_len(SPACE_CLOSE);
            }
        }
    }};
}

////////////////////////////////////////////////////////////////////////////////

pub struct DebugStructLength<'f> {
    fmt: &'f mut FormattingLength,
    wrote_field: bool,
}

impl DebugStructLength<'_> {
    pub const fn field(&mut self, name: &str) -> &mut FormattingLength {
        const COLON_SPACE_LEN: usize = ": ".len();

        field_method_impl!(
            self, " { ", " {\n";
            self.fmt.add_len(name.len() + COLON_SPACE_LEN);
        )
    }

    pub const fn finish(&mut self) {
        finish_method_impl!(self, "}", " }")
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct DebugTupleLength<'f> {
    fmt: &'f mut FormattingLength,
    wrote_field: bool,
}

impl DebugTupleLength<'_> {
    pub const fn field(&mut self) -> &mut FormattingLength {
        field_method_impl!(self, "(", "(\n"; )
    }

    pub const fn finish(&mut self) {
        finish_method_impl!(self, ")", ")")
    }
}

////////////////////////////////////////////////////////////////////////////////

macro_rules! finish_listset_method_impl {
    ($self: ident, $close_token:expr, $open_close:expr) => {{
        const CLOSE_TOKEN: usize = $close_token.len();
        const OPEN_CLOSE: usize = $open_close.len();

        $self.fmt.flags = $self.fmt.flags.decrement_margin();
        if $self.wrote_field {
            if $self.fmt.flags.is_alternate() {
                $self.fmt.add_len(COMMA_NL_LEN + $self.fmt.flags.margin());
            }
            $self.fmt.add_len(CLOSE_TOKEN);
        } else {
            $self.fmt.add_len(OPEN_CLOSE);
        }
    }};
}

////////////////////////////////////////////////////////////////////////////////

pub struct DebugListLength<'f> {
    fmt: &'f mut FormattingLength,
    wrote_field: bool,
}

impl DebugListLength<'_> {
    pub const fn entry(&mut self) -> &mut FormattingLength {
        field_method_impl!(self, "[", "[\n";)
    }

    pub const fn finish(&mut self) {
        finish_listset_method_impl!(self, "]", "[]")
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct DebugSetLength<'f> {
    fmt: &'f mut FormattingLength,
    wrote_field: bool,
}

impl DebugSetLength<'_> {
    pub const fn entry(&mut self) -> &mut FormattingLength {
        field_method_impl!(self, "{", "{\n";)
    }

    pub const fn finish(&mut self) {
        finish_listset_method_impl!(self, "}", "{}")
    }
}

////////////////////////////////////////////////////////////////////////////////
