use crate::format_str_parsing::FormatStr;
use crate::formatting::FormattingFlags;

use proc_macro2::{Span, TokenStream as TokenStream2};

use syn::{punctuated::Punctuated, Ident, Token};

use quote::{quote, quote_spanned};

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

/// The arguments of `writec`
pub(crate) struct WriteArgs {
    pub(crate) writer_expr: TokenStream2,
    pub(crate) format_args: FormatArgs,
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

impl ExpandInto {
    pub(crate) fn formatting_flags(&self) -> FormattingFlags {
        match self {
            Self::Str { .. } => FormattingFlags::NEW,
            Self::Formatted(fmted) => fmted.format,
        }
    }
    pub(crate) fn len_call(&self, cratep: &TokenStream2, strlen: &Ident) -> TokenStream2 {
        let flags = self.formatting_flags().tokens(cratep);
        match self {
            ExpandInto::Str(str) => {
                let len = str.len();
                quote!( #strlen.add_len(#len); )
            }
            ExpandInto::Formatted(fmted) => {
                let len_method = fmted.format.len_method_name();
                let local_variable = &fmted.local_variable;
                let span = local_variable.span();

                quote_spanned!(span=>
                    let _ = #cratep::coerce_to_fmt!(#local_variable)
                        .#len_method(&mut #strlen.make_formatter(#flags));
                )
            }
        }
    }
    pub(crate) fn fmt_call(&self, cratep: &TokenStream2, formatter: &Ident) -> TokenStream2 {
        let flags = self.formatting_flags().tokens(cratep);
        match self {
            ExpandInto::Str(str) => quote!( #formatter.write_whole_str(#str) ),
            ExpandInto::Formatted(fmted) => {
                let fmt_method = fmted.format.fmt_method_name();
                let local_variable = &fmted.local_variable;
                let span = local_variable.span();

                quote_spanned!(span=>
                    #cratep::coerce_to_fmt!(&#local_variable)
                        .#fmt_method(&mut #formatter.make_formatter(#flags))
                )
            }
        }
    }
}
