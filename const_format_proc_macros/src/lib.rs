use proc_macro::TokenStream as TokenStream1;

use proc_macro2::TokenStream as TokenStream2;

mod format_args;
mod format_str_parsing;

mod format_macro;

mod formatting;

mod parse_utils;

mod utils;

#[cfg(test)]
mod test_utils;

fn compile_err_empty_str(e: syn::Error) -> TokenStream2 {
    let e = e.to_compile_error();
    quote::quote!({
        #e;
        ""
    })
}

/// Input syntax: `"format string", (arg0), (name = arg1)` (with optional trailing comma).
///
/// The arguments are parenthesized to not require syn to parse `arg0` and `arg1` as syn::Expr,
/// they're just parsed as a `TokenStream2`.
///
/// They're guaranteed to be expressions when this macro is invoked by `const_format` macros,
/// which should be the only ones to do so.
#[doc(hidden)]
#[proc_macro]
pub fn __formatcp_impl(input: TokenStream1) -> TokenStream1 {
    syn::parse(input)
        .and_then(format_macro::macro_impl)
        .unwrap_or_else(compile_err_empty_str)
        .into()
}

#[doc(hidden)]
#[proc_macro]
pub fn __formatc_impl(input: TokenStream1) -> TokenStream1 {
    syn::parse(input)
        .and_then(format_macro::formatc_macro_impl)
        .unwrap_or_else(compile_err_empty_str)
        .into()
}

#[doc(hidden)]
#[proc_macro]
pub fn __writec_impl(input: TokenStream1) -> TokenStream1 {
    syn::parse(input)
        .and_then(format_macro::writec_macro_impl)
        .unwrap_or_else(compile_err_empty_str)
        .into()
}

#[allow(dead_code)]
fn parse_or_compile_err<P, F>(input: TokenStream1, f: F) -> TokenStream2
where
    P: syn::parse::Parse,
    F: FnOnce(P) -> Result<TokenStream2, syn::Error>,
{
    // println!("{}", input);

    syn::parse::<P>(input)
        .and_then(f)
        .unwrap_or_else(|e| e.to_compile_error())
}
