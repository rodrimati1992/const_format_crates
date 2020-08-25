use proc_macro2::{Span, TokenStream as TokenStream2, TokenTree};

use syn::parse::{Parse, ParseBuffer, ParseStream, Parser};

pub(crate) trait ParseBufferExt {
    fn as_parse_buffer(&self) -> &ParseBuffer<'_>;

    fn parse_token_stream_and_span(&self) -> (TokenStream2, Span) {
        let input = self.as_parse_buffer();
        let mut span = input.span();
        let ts = std::iter::from_fn(|| {
            if input.is_empty() {
                None
            } else {
                if let Some(x) = span.join(input.span()) {
                    span = x;
                }

                let tt = input
                    .parse::<proc_macro2::TokenTree>()
                    .expect("A non-empty ParseStream cannot fail to parse a TokenTree");

                Some(tt)
            }
        })
        .collect::<TokenStream2>();

        (ts, span)
    }

    /// Unwraps a none-delimited token tree to parse a type,
    /// if the first token is not a none-delimited token tree it parses the type in
    /// the passed in ParseStream.
    fn parse_unwrap_tt<F, T>(&self, f: F) -> Result<T, syn::Error>
    where
        F: FnOnce(ParseStream) -> Result<T, syn::Error>,
    {
        let input = self.as_parse_buffer();
        if input.peek(syn::token::Group) {
            if let TokenTree::Group(group) = input.parse::<TokenTree>()? {
                Parser::parse2(f, group.stream())
            } else {
                unreachable!("But I peeked for a syn::Token::Group!!")
            }
        } else {
            f(input)
        }
    }
}

impl ParseBufferExt for ParseBuffer<'_> {
    #[inline(always)]
    fn as_parse_buffer(&self) -> &ParseBuffer<'_> {
        self
    }
}

///////////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////////////////////////

/// Configuration for all function-like proc macros,
/// parsed from the first tokens of function-like proc macros.
pub(crate) struct WithProcMacroArgs<P> {
    /// The path to the `const_format` crate
    pub(crate) crate_path: TokenStream2,

    pub(crate) value: P,
}

impl<P> Parse for WithProcMacroArgs<P>
where
    P: Parse,
{
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let content;
        let _ = syn::parenthesized!(content in input);

        let crate_path = {
            let inside;
            let _ = syn::parenthesized!(inside in content);
            inside.parse::<TokenStream2>().unwrap()
        };

        Ok(Self {
            crate_path,
            value: P::parse(input)?,
        })
    }
}
