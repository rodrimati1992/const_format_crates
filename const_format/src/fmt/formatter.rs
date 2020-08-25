use crate::{
    fmt::{str_writer::saturate_range, Error, FormattingFlags, StrWriter},
    wrapper_types::{AsciiStr, PWrapper},
};

use core::ops::Range;

enum WriterBackend<'w> {
    Str(&'w mut StrWriter),
    Length(&'w mut ComputeStrLength),
}

////////////////////////////////////////////////////////////////////////////////

pub struct ComputeStrLength {
    len: usize,
}

impl ComputeStrLength {
    pub const fn new() -> Self {
        Self { len: 0 }
    }

    pub const fn make_formatter(&mut self, flags: FormattingFlags) -> Formatter<'_> {
        Formatter {
            flags,
            writer: WriterBackend::Length(self),
        }
    }

    pub const fn add_len(&mut self, len: usize) {
        self.len += len;
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    // For borrowing this mutably in macros, without getting nested mutable references.
    #[inline(always)]
    pub const fn borrow_mutably(&mut self) -> &mut Self {
        self
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct Formatter<'w> {
    flags: FormattingFlags,
    writer: WriterBackend<'w>,
}

impl<'w> Formatter<'w> {
    #[inline]
    pub const fn with_strwriter(flags: FormattingFlags, writer: &'w mut StrWriter) -> Self {
        Self {
            flags,
            writer: WriterBackend::Str(writer),
        }
    }

    #[inline(always)]
    pub const fn flags(&self) -> FormattingFlags {
        self.flags
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
    pub const fn debug_struct(&mut self, name: &str) -> DebugStruct<'_, 'w> {
        let err = self.write_whole_str(name);
        DebugStruct {
            fmt: self.increment_margin(),
            wrote_field: false,
            err,
        }
    }

    #[inline]
    pub const fn debug_tuple(&mut self, name: &str) -> DebugTuple<'_, 'w> {
        let err = self.write_whole_str(name);
        DebugTuple {
            fmt: self.increment_margin(),
            wrote_field: false,
            err,
        }
    }

    #[inline]
    pub const fn debug_list(&mut self) -> DebugList<'_, 'w> {
        DebugList {
            fmt: self.increment_margin(),
            wrote_field: false,
            err: Ok(()),
        }
    }

    #[inline]
    pub const fn debug_set(&mut self) -> DebugSet<'_, 'w> {
        DebugSet {
            fmt: self.increment_margin(),
            wrote_field: false,
            err: Ok(()),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

macro_rules! trys {
    ($e:expr,$self:ident) => {
        if let result @ Err(_) = $e {
            $self.err = result;
        }
    };
}

const COLON_SPACE_LEN: usize = ": ".len();
const COMMA_SPACE_LEN: usize = ", ".len();
const COMMA_NL_LEN: usize = ",\n".len();

macro_rules! field_method_impl {
    ($
        self: ident, $open_space:expr, $open_newline:expr;
        len(|$fmt_len:ident| $($write_name_len:tt)*)
        fmt(|$writer:ident| $($write_name_fmt:tt)*)
    ) => ({
        match &mut $self.fmt.writer {
            WriterBackend::Length($fmt_len)=>{
                let $fmt_len = &mut **$fmt_len;

                const OPEN_SPACE: usize = $open_space.len();
                const OPEN_NEWLINE: usize = $open_newline.len();

                let is_alternate = $self.fmt.flags.is_alternate();
                $fmt_len.add_len(match ($self.wrote_field, is_alternate) {
                    (false, false) => OPEN_SPACE,
                    (false, true) => OPEN_NEWLINE + $self.fmt.flags.margin(),
                    (true , false) => COMMA_SPACE_LEN,
                    (true , true) => COMMA_NL_LEN + $self.fmt.flags.margin(),
                });
                $($write_name_len)*
            }
            WriterBackend::Str($writer)=>{
                let $writer = &mut **$writer;

                let is_alternate = $self.fmt.flags.is_alternate();
                let sep = match ($self.wrote_field, is_alternate) {
                    (false, false)=>$open_space,
                    (false, true)=>$open_newline,
                    (true, false)=>", ",
                    (true, true)=>",\n",
                };
                trys!($writer.write_whole_str(sep), $self);
                if is_alternate {
                    trys!($writer.write_ascii_repeated(b' ', $self.fmt.flags.margin()), $self);
                }
                $($write_name_fmt)*
            }
        }
        $self.wrote_field = true;

        $self.fmt
    })
}

macro_rules! finish_method_impl {
    ($self: ident, $close_token:expr, $space_close:expr) => {{
        if let result @ Err(_) = $self.err {
            return result;
        }

        $self.fmt.decrement_margin();
        if $self.wrote_field {
            match &mut $self.fmt.writer {
                WriterBackend::Length(fmt_len) => {
                    let fmt_len = &mut **fmt_len;

                    const CLOSE_TOKEN: usize = $close_token.len();
                    const SPACE_CLOSE: usize = $space_close.len();

                    if $self.fmt.flags.is_alternate() {
                        fmt_len.add_len(COMMA_NL_LEN + $self.fmt.flags.margin() + CLOSE_TOKEN);
                    } else {
                        fmt_len.add_len(SPACE_CLOSE);
                    }
                    Ok(())
                }
                WriterBackend::Str(writer) => {
                    let writer = &mut **writer;

                    if $self.fmt.flags.is_alternate() {
                        try_!(writer.write_whole_str(",\n"));
                        try_!(writer.write_ascii_repeated(b' ', $self.fmt.flags.margin()));
                        writer.write_whole_str($close_token)
                    } else {
                        writer.write_whole_str($space_close)
                    }
                }
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
    err: Result<(), Error>,
}

impl<'f, 'w> DebugStruct<'f, 'w> {
    pub const fn field(&mut self, name: &str) -> &mut Formatter<'w> {
        field_method_impl!(
            self, " { ", " {\n";
            len(|fmt_len|
                fmt_len.add_len(name.len() + COLON_SPACE_LEN);
            )
            fmt(|writer|
                trys!(writer.write_whole_str(name), self);
                trys!(writer.write_whole_str(": "), self);
            )
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
    err: Result<(), Error>,
}

impl<'f, 'w> DebugTuple<'f, 'w> {
    pub const fn field(&mut self) -> &mut Formatter<'w> {
        field_method_impl!(self, "(", "(\n"; len(|fmt_len|) fmt(|writer|) )
    }

    pub const fn finish(self) -> Result<(), Error> {
        finish_method_impl!(self, ")", ")")
    }
}

////////////////////////////////////////////////////////////////////////////////

macro_rules! finish_listset_method_impl {
    ($self: ident, $close_token:expr, $open_close:expr) => {{
        if let result @ Err(_) = $self.err {
            return result;
        }

        match &mut $self.fmt.writer {
            WriterBackend::Length(fmt_len) => {
                let fmt_len = &mut **fmt_len;
                const CLOSE_TOKEN: usize = $close_token.len();
                const OPEN_CLOSE: usize = $open_close.len();

                $self.fmt.flags = $self.fmt.flags.decrement_margin();
                if $self.wrote_field {
                    if $self.fmt.flags.is_alternate() {
                        fmt_len.add_len(COMMA_NL_LEN + $self.fmt.flags.margin());
                    }
                    fmt_len.add_len(CLOSE_TOKEN);
                } else {
                    fmt_len.add_len(OPEN_CLOSE);
                }
                Ok(())
            }
            WriterBackend::Str(writer) => {
                let writer = &mut **writer;

                $self.fmt.flags = $self.fmt.flags.decrement_margin();
                let margin = $self.fmt.flags.margin();
                if $self.wrote_field {
                    if $self.fmt.flags.is_alternate() {
                        try_!(writer.write_whole_str(",\n"));
                        try_!(writer.write_ascii_repeated(b' ', margin));
                    }
                    writer.write_whole_str($close_token)
                } else {
                    writer.write_whole_str($open_close)
                }
            }
        }
    }};
}

////////////////////////////////////////////////////////////////////////////////

pub struct DebugList<'f, 'w> {
    fmt: &'f mut Formatter<'w>,
    wrote_field: bool,
    err: Result<(), Error>,
}

impl<'f, 'w> DebugList<'f, 'w> {
    pub const fn entry(&mut self) -> &mut Formatter<'w> {
        field_method_impl!(self, "[", "[\n"; len(|fmt_len|) fmt(|writer|) )
    }

    pub const fn finish(self) -> Result<(), Error> {
        finish_listset_method_impl!(self, "]", "[]")
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct DebugSet<'f, 'w> {
    fmt: &'f mut Formatter<'w>,
    wrote_field: bool,
    err: Result<(), Error>,
}

impl<'f, 'w> DebugSet<'f, 'w> {
    pub const fn entry(&mut self) -> &mut Formatter<'w> {
        field_method_impl!(self, "{", "{\n"; len(|fmt_len|) fmt(|writer|) )
    }

    pub const fn finish(self) -> Result<(), Error> {
        finish_listset_method_impl!(self, "}", "{}")
    }
}

////////////////////////////////////////////////////////////////////////////////

macro_rules! delegate_write_methods {
    (
        $(
            $(#[$attrs:meta])*
            fn $method:ident($($arg:ident: $arg_ty:ty ),* $(,)* )
            length = $len:expr;
        )*
    ) => (
        $(
            impl Formatter<'_>{
                $(#[$attrs:meta])*
                #[inline(always)]
                pub const fn $method(&mut self, $($arg: $arg_ty ),*  ) -> Result<(), Error> {
                    match &mut self.writer {
                        WriterBackend::Length(fmt_len)=>{
                            fmt_len.add_len($len);
                            Ok(())
                        }
                        WriterBackend::Str(writer)=>{
                            writer.$method($($arg,)*)
                        }
                    }
                }
            }

        )*
    )
}

delegate_write_methods! {
    fn write_u8_display(n: u8)
    length = PWrapper(n).compute_display_len(FormattingFlags::NEW);

    fn write_u8_debug(n: u8, flags: FormattingFlags)
    length = PWrapper(n).compute_debug_len(flags);

    fn write_u16_display(n: u16)
    length = PWrapper(n).compute_display_len(FormattingFlags::NEW);

    fn write_u16_debug(n: u16, flags: FormattingFlags)
    length = PWrapper(n).compute_debug_len(flags);

    fn write_u32_display(n: u32)
    length = PWrapper(n).compute_display_len(FormattingFlags::NEW);

    fn write_u32_debug(n: u32, flags: FormattingFlags)
    length = PWrapper(n).compute_debug_len(flags);

    fn write_u64_display(n: u64)
    length = PWrapper(n).compute_display_len(FormattingFlags::NEW);

    fn write_u64_debug(n: u64, flags: FormattingFlags)
    length = PWrapper(n).compute_debug_len(flags);

    fn write_u128_display(n: u128)
    length = PWrapper(n).compute_display_len(FormattingFlags::NEW);

    fn write_u128_debug(n: u128, flags: FormattingFlags)
    length = PWrapper(n).compute_debug_len(flags);

    fn write_usize_display(n: usize)
    length = PWrapper(n).compute_display_len(FormattingFlags::NEW);

    fn write_usize_debug(n: usize, flags: FormattingFlags)
    length = PWrapper(n).compute_debug_len(flags);

    fn write_i8_display(n: i8)
    length = PWrapper(n).compute_display_len(FormattingFlags::NEW);

    fn write_i8_debug(n: i8, flags: FormattingFlags)
    length = PWrapper(n).compute_debug_len(flags);

    fn write_i16_display(n: i16)
    length = PWrapper(n).compute_display_len(FormattingFlags::NEW);

    fn write_i16_debug(n: i16, flags: FormattingFlags)
    length = PWrapper(n).compute_debug_len(flags);

    fn write_i32_display(n: i32)
    length = PWrapper(n).compute_display_len(FormattingFlags::NEW);

    fn write_i32_debug(n: i32, flags: FormattingFlags)
    length = PWrapper(n).compute_debug_len(flags);

    fn write_i64_display(n: i64)
    length = PWrapper(n).compute_display_len(FormattingFlags::NEW);

    fn write_i64_debug(n: i64, flags: FormattingFlags)
    length = PWrapper(n).compute_debug_len(flags);

    fn write_i128_display(n: i128)
    length = PWrapper(n).compute_display_len(FormattingFlags::NEW);

    fn write_i128_debug(n: i128, flags: FormattingFlags)
    length = PWrapper(n).compute_debug_len(flags);

    fn write_isize_display(n: isize)
    length = PWrapper(n).compute_display_len(FormattingFlags::NEW);

    fn write_isize_debug(n: isize, flags: FormattingFlags)
    length = PWrapper(n).compute_debug_len(flags);


    fn write_str(s: &str, range: &Range<usize>)
    length = calculate_display_len(s.as_bytes(), range);

    fn write_whole_str(s: &str)
    length = s.len();

    fn write_ascii(ascii: AsciiStr<'_>, range: &Range<usize>)
    length = calculate_display_len(ascii.as_bytes(), range);

    fn write_whole_ascii( ascii: AsciiStr<'_>)
    length = ascii.len();

    fn write_ascii_repeated(character: u8,repeated: usize)
    length = repeated;

    fn write_str_debug(s: &str, range: &Range<usize>)
    length = calculate_display_len_debug_range(s.as_bytes(), range);

    fn write_whole_str_debug(str: &str)
    length = PWrapper(str.as_bytes()).compute_utf8_debug_len();

    fn write_ascii_debug(ascii: AsciiStr<'_>,range: &Range<usize>)
    length = calculate_display_len_debug_range(ascii.as_bytes(), range);

    fn write_whole_ascii_debug(ascii: AsciiStr<'_>)
    length = PWrapper(ascii.as_bytes()).compute_utf8_debug_len();

}

#[inline(always)]
const fn calculate_display_len(b: &[u8], range: &Range<usize>) -> usize {
    let Range { start, end } = saturate_range(b, range);
    end - start
}

#[inline(always)]
const fn calculate_display_len_debug_range(b: &[u8], range: &Range<usize>) -> usize {
    let Range { start, end } = saturate_range(b, range);
    PWrapper(b).compute_utf8_debug_len_in_range(start..end)
}

impl Formatter<'_> {
    pub const fn len(&self) -> usize {
        match &self.writer {
            WriterBackend::Str(x) => x.len(),
            WriterBackend::Length(x) => x.len(),
        }
    }
    pub const fn capacity(&self) -> usize {
        match &self.writer {
            WriterBackend::Str(x) => x.capacity(),
            WriterBackend::Length(_) => isize::MAX as usize,
        }
    }

    // For borrowing this mutably in macros, without getting nested mutable references.
    #[inline(always)]
    pub const fn borrow_mutably(&mut self) -> &mut Self {
        self
    }

    ///
    ///
    /// Keeps the margin the same in the returned Formatter,
    /// so that alternate debug formatting isn't unindented.
    pub const fn make_formatter(&mut self, flags: FormattingFlags) -> Formatter<'_> {
        let flags = flags.copy_margin_of(self.flags);
        match &mut self.writer {
            WriterBackend::Str(x) => x.make_formatter(flags),
            WriterBackend::Length(x) => x.make_formatter(flags),
        }
    }
}
