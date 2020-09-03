use crate::parse_utils::{MyParse, ParseBuffer, ParseStream};

use proc_macro2::{Span, TokenStream as TokenStream2};

////////////////////////////////////////////////

pub(crate) struct ConcatArg {
    pub(crate) span: Span,
    /// Using a TokenStream2 because it is validated to be a valid expression in
    /// the macro_rules! macros that call these proc macros.
    pub(crate) expr: TokenStream2,
}
pub(crate) struct ConcatArgs {
    pub(crate) args: Vec<ConcatArg>,
}

////////////////////////////////////////////////

impl MyParse for ConcatArg {
    fn parse(input: ParseStream<'_>) -> Result<Self, crate::Error> {
        let paren = input.parse_paren()?;

        let mut content = ParseBuffer::new(paren.contents);

        content.parse_unwrap_tt(|content| {
            let (expr, span) = content.parse_token_stream_and_span();

            Ok(Self { span, expr })
        })
    }
}

////////////////////////////////////////////////

impl MyParse for ConcatArgs {
    fn parse(input: ParseStream<'_>) -> Result<Self, crate::Error> {
        let mut args = Vec::new();

        if !input.is_empty() {
            input.parse_punct(',')?;
        }

        while !input.is_empty() {
            args.push(ConcatArg::parse(input)?);

            if !input.is_empty() {
                input.parse_punct(',')?;
            }
        }

        Ok(Self { args })
    }
}
