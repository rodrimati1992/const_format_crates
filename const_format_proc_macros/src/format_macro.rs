use crate::{
    format_args::{ExpandInto, FormatArgs},
    parse_utils::WithProcMacroArgs,
};

use proc_macro2::TokenStream as TokenStream2;

use quote::{quote, quote_spanned};

#[cfg(test)]
mod tests;

////////////////////////////////////////////////////////////////////////////////

pub(crate) fn macro_impl(args: WithProcMacroArgs<FormatArgs>) -> Result<TokenStream2, syn::Error> {
    let crate_path = args.crate_path;
    let fmt_args = args.value;

    let locals = fmt_args.args.iter().map(|arg| {
        let local_variable = &arg.local_variable;
        let expr = &arg.expr;
        let span = local_variable.span();
        quote_spanned!(span=> #local_variable, #expr)
    });

    let arg_pairs = fmt_args.expanded_into.iter().map(|ei| match ei {
        ExpandInto::Str(str) => quote!(#crate_path::pmr::Formatting::Display, #str),
        ExpandInto::Formatted(fmted) => {
            let formatting = fmted.format.tokens(&crate_path);
            let local_variable = &fmted.local_variable;
            let span = local_variable.span();
            quote_spanned!(span=> #formatting, #local_variable)
        }
    });

    Ok(quote!(
        #crate_path::concatcp!(
            @with_fmt
            locals(#( (#locals) )*)
            #((#arg_pairs)) *
        )
    ))
}
