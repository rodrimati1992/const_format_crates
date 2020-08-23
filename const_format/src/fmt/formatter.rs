use crate::fmt::{Error, FormattingFlags, StrWriter};

pub struct Formatter<'w> {
    flags: FormattingFlags,
    writer: &'w mut StrWriter,
}

impl<'w> Formatter<'w> {
    #[inline]
    pub const fn new(flags: FormattingFlags, writer: &'w mut StrWriter) -> Self {
        Self { flags, writer }
    }

    #[inline(always)]
    pub const fn flags(&self) -> FormattingFlags {
        self.flags
    }

    #[inline(always)]
    pub const fn w(&mut self) -> &mut StrWriter {
        self.writer
    }

    /// For the
    #[inline(always)]
    pub const fn strwriter(&mut self) -> &mut StrWriter {
        self.writer
    }

    #[inline(always)]
    const fn increment_margin(&mut self) -> &mut Self {
        self.flags = self.flags.increment_margin();
        self
    }

    #[inline(always)]
    const fn decrement_margin(&mut self) {
        self.flags = self.flags.decrement_margin();
    }

    #[inline]
    pub const fn debug_struct(&mut self, name: &str) -> Result<DebugStruct<'_, 'w>, Error> {
        try_!(self.writer.write_whole_str(name));
        Ok(DebugStruct {
            fmt: self.increment_margin(),
            wrote_field: false,
        })
    }

    #[inline]
    pub const fn debug_tuple(&mut self, name: &str) -> Result<DebugTuple<'_, 'w>, Error> {
        try_!(self.writer.write_whole_str(name));
        Ok(DebugTuple {
            fmt: self.increment_margin(),
            wrote_field: false,
        })
    }

    #[inline]
    pub const fn debug_list(&mut self) -> Result<DebugList<'_, 'w>, Error> {
        Ok(DebugList {
            fmt: self.increment_margin(),
            wrote_field: false,
        })
    }

    #[inline]
    pub const fn debug_set(&mut self) -> Result<DebugSet<'_, 'w>, Error> {
        Ok(DebugSet {
            fmt: self.increment_margin(),
            wrote_field: false,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////

macro_rules! field_method_impl {
    ($
        self: ident, $open_space:expr, $open_newline:expr;
        $($write_name:tt)*
    ) => ({
        let is_alternate = $self.fmt.flags.is_alternate();
        let sep = match ($self.wrote_field, is_alternate) {
            (false, false)=>$open_space,
            (false, true)=>$open_newline,
            (true, false)=>", ",
            (true, true)=>",\n",
        };
        try_!($self.fmt.writer.write_whole_str(sep));
        if is_alternate {
            try_!($self.fmt.writer.write_ascii_repeated(b' ', $self.fmt.flags.margin()));
        }
        $($write_name)*
        $self.wrote_field = true;

        Ok($self.fmt)
    })
}

macro_rules! finish_method_impl {
    ($self: ident, $close_token:expr, $space_close:expr) => {{
        $self.fmt.decrement_margin();
        if $self.wrote_field {
            if $self.fmt.flags.is_alternate() {
                try_!($self.fmt.writer.write_whole_str(",\n"));
                try_!($self
                    .fmt
                    .writer
                    .write_ascii_repeated(b' ', $self.fmt.flags.margin()));
                $self.fmt.writer.write_whole_str($close_token)
            } else {
                $self.fmt.writer.write_whole_str($space_close)
            }
        } else {
            Ok(())
        }
    }};
}

////////////////////////////////////////////////////////////////////////////////

pub struct DebugStruct<'f, 'w> {
    fmt: &'f mut Formatter<'w>,
    wrote_field: bool,
}

impl<'f, 'w> DebugStruct<'f, 'w> {
    pub const fn field(&mut self, name: &str) -> Result<&mut Formatter<'w>, Error> {
        field_method_impl!(
            self, " { ", " {\n";
            try_!(self.fmt.writer.write_whole_str(name));
            try_!(self.fmt.writer.write_whole_str(": "));
        )
    }

    pub const fn finish(self) -> Result<(), Error> {
        finish_method_impl!(self, "}", " }")
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct DebugTuple<'f, 'w> {
    fmt: &'f mut Formatter<'w>,
    wrote_field: bool,
}

impl<'f, 'w> DebugTuple<'f, 'w> {
    pub const fn field(&mut self) -> Result<&mut Formatter<'w>, Error> {
        field_method_impl!(self, "(", "(\n"; )
    }

    pub const fn finish(self) -> Result<(), Error> {
        finish_method_impl!(self, ")", ")")
    }
}

////////////////////////////////////////////////////////////////////////////////

macro_rules! finish_listset_method_impl {
    ($self: ident, $close_token:expr, $open_close:expr) => {{
        $self.fmt.decrement_margin();
        let margin = $self.fmt.flags.margin();
        if $self.wrote_field {
            if $self.fmt.flags.is_alternate() {
                try_!($self.fmt.writer.write_whole_str(",\n"));
                try_!($self.fmt.writer.write_ascii_repeated(b' ', margin));
            }
            $self.fmt.writer.write_whole_str($close_token)
        } else {
            $self.fmt.writer.write_whole_str($open_close)
        }
    }};
}

////////////////////////////////////////////////////////////////////////////////

pub struct DebugList<'f, 'w> {
    fmt: &'f mut Formatter<'w>,
    wrote_field: bool,
}

impl<'f, 'w> DebugList<'f, 'w> {
    pub const fn entry(&mut self) -> Result<&mut Formatter<'w>, Error> {
        field_method_impl!(self, "[", "[\n"; )
    }

    pub const fn finish(self) -> Result<(), Error> {
        finish_listset_method_impl!(self, "]", "[]")
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct DebugSet<'f, 'w> {
    fmt: &'f mut Formatter<'w>,
    wrote_field: bool,
}

impl<'f, 'w> DebugSet<'f, 'w> {
    pub const fn entry(&mut self) -> Result<&mut Formatter<'w>, Error> {
        field_method_impl!(self, "{", "{\n"; )
    }

    pub const fn finish(self) -> Result<(), Error> {
        finish_listset_method_impl!(self, "}", "{}")
    }
}

////////////////////////////////////////////////////////////////////////////////
