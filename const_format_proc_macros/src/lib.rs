use proc_macro::TokenStream as TokenStream1;

use proc_macro2::TokenStream as TokenStream2;

mod format_args;
mod format_str_parsing;

mod format_macro;

mod parse_utils;

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
    parse_or_compile_err(input, format_macro::macro_impl).into()
}

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
