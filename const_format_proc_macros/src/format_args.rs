use crate::format_str_parsing::FormatStr;
use crate::formatting::FormattingFlags;

use proc_macro2::{Span, TokenStream as TokenStream2};

use syn::{punctuated::Punctuated, Ident, Token};

////////////////////////////////////////////////

mod parsing;

////////////////////////////////////////////////

struct UncheckedFormatArgs {
    format_str_span: Span,
    literal: FormatStr,
    args: Punctuated<UncheckedFormatArg, Token!(,)>,
}

struct UncheckedFormatArg {
    span: Span,
    ident: Option<Ident>,
    /// Using a TokenStream2 because it is validated to be a valid expression in
    /// the macro_rules! macros that call these proc macros.
    expr: TokenStream2,
}

pub(crate) struct FormatArgs {
    pub(crate) args: Vec<FormatArg>,
    pub(crate) expanded_into: Vec<ExpandInto>,
}

pub(crate) enum ExpandInto {
    Str(String),
    Formatted(ExpandFormatted),
}

pub(crate) struct ExpandFormatted {
    pub(crate) format: FormattingFlags,
    pub(crate) local_variable: Ident,
}

pub(crate) struct FormatArg {
    // The local variable that the macro will output for this argument,
    // so that it is not evaluated multiple times when it's used multiple times
    // in the format string..
    pub(crate) local_variable: Ident,
    /// Using a TokenStream2 because it is validated to be a valid expression in
    /// the macro_rules! macros that call these proc macros.
    pub(crate) expr: TokenStream2,
}

////////////////////////////////////////////////
