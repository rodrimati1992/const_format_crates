use crate::{
    format_str_parsing::FormatStr, formatting::FormattingFlags, parse_utils::StrRawness,
    shared_arg_parsing::ExprArg,
};

use proc_macro2::{Ident, Span, TokenStream as TokenStream2};

use quote::quote_spanned;

////////////////////////////////////////////////

mod parsing;

////////////////////////////////////////////////

struct UncheckedFormatArgs {
    literal: FormatStr,
    args: Vec<UncheckedFormatArg>,
}

struct UncheckedFormatArg {
    span: Span,
    ident: Option<Ident>,
    /// Using a TokenStream2 because it is validated to be a valid expression in
    /// the macro_rules! macros that call these proc macros.
    expr: TokenStream2,
}

pub(crate) struct FormatArgs {
    pub(crate) condition: Option<ExprArg>,
    pub(crate) args: Vec<FormatArg>,
    pub(crate) expanded_into: Vec<ExpandInto>,
}

pub(crate) struct FormatIfArgs {
    pub(crate) inner: FormatArgs,
}

/// The arguments of `writec`
pub(crate) struct WriteArgs {
    pub(crate) writer_expr: TokenStream2,
    pub(crate) format_args: FormatArgs,
}

pub(crate) enum ExpandInto {
    Str(String, StrRawness),
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

impl ExpandInto {
    pub(crate) fn formatting_flags(&self) -> FormattingFlags {
        match self {
            Self::Str { .. } => FormattingFlags::NEW,
            Self::Formatted(fmted) => fmted.format,
        }
    }
    pub(crate) fn fmt_call(&self, formatter: &Ident) -> TokenStream2 {
        let flags = self.formatting_flags();
        match self {
            ExpandInto::Str(str, rawness) => {
                let str_tokens = rawness.tokenize_sub(str);

                quote_spanned!(rawness.span()=> #formatter.write_str(#str_tokens) )
            }
            ExpandInto::Formatted(fmted) => {
                let fmt_method = fmted.format.fmt_method_name();
                let local_variable = &fmted.local_variable;
                let span = local_variable.span();

                quote_spanned!(span=>
                    __cf_osRcTFl4A::coerce_to_fmt!(&#local_variable)
                        .#fmt_method(&mut #formatter.make_formatter(#flags))
                )
            }
        }
    }
}
