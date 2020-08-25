use super::{
    ExpandFormatted, ExpandInto, FormatArg, FormatArgs, UncheckedFormatArg, UncheckedFormatArgs,
    WriteArgs,
};

use crate::{
    format_str_parsing::{FmtStrComponent, FormatStr, WhichArg},
    parse_utils::ParseBufferExt,
    utils::{dummy_ident, LinearResult},
};

use proc_macro2::Span;

use syn::{
    parse::{Parse, ParseStream},
    Ident, LitStr, Token,
};

////////////////////////////////////////////////

impl Parse for UncheckedFormatArg {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let content;
        let _parentheses = syn::parenthesized!(content in input);

        content.parse_unwrap_tt(|content| {
            let ident;
            if content.peek2(Token!(=)) {
                ident = Some(content.parse()?);
                let _: Token!(=) = content.parse()?;
            } else {
                ident = None;
            };

            let (expr, span) = content.parse_token_stream_and_span();

            Ok(Self { span, ident, expr })
        })
    }
}

////////////////////////////////////////////////

impl Parse for UncheckedFormatArgs {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let lit = input.parse::<LitStr>()?;
        let lit_str = lit.value();
        let format_str_span = lit.span();
        let literal = lit
            .value()
            .parse::<FormatStr>()
            .map_err(|e| e.into_syn_err(format_str_span, &lit_str))?;

        let comma: Option<Token!(,)> = input.parse()?;

        if !input.is_empty() && comma.is_none() {
            return Err(syn::Error::new(
                format_str_span,
                "Expected comma or the end of the formatting arguments",
            ));
        }

        Ok(Self {
            format_str_span,
            literal,
            args: input.parse_terminated(Parse::parse)?,
        })
    }
}

////////////////////////////////////////////////

impl Parse for FormatArgs {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let prefix = Ident::new("const_fmt_local_", Span::call_site());
        FormatArgs::parse_with(input, prefix)
    }
}

impl FormatArgs {
    pub fn parse_with(input: ParseStream, prefix: Ident) -> Result<FormatArgs, syn::Error> {
        let mut res = LinearResult::ok(());

        let unchecked_fargs = input.parse::<UncheckedFormatArgs>()?;

        let mut first_named_arg = unchecked_fargs.args.len();

        let mut named_arg_names = Vec::<Ident>::new();
        let mut args = Vec::<FormatArg>::with_capacity(unchecked_fargs.args.len());

        let arg_span_idents: Vec<(Span, Option<Ident>)> = unchecked_fargs
            .args
            .iter()
            .map(|x| (x.span, x.ident.clone()))
            .collect();

        {
            let mut prev_is_named_arg = false;
            for (i, arg) in unchecked_fargs.args.into_iter().enumerate() {
                let expr_span = arg.span;

                let make_ident = |s: String| Ident::new(&s, expr_span);

                let is_named_arg = arg.ident.is_some();

                let var_name = if let Some(ident) = arg.ident {
                    if !prev_is_named_arg {
                        first_named_arg = i;
                    }

                    let name = make_ident(format!("{}{}", prefix, ident));
                    named_arg_names.push(ident);
                    name
                } else {
                    if prev_is_named_arg {
                        return Err(syn::Error::new(
                            expr_span,
                            "expected a named argument, \
                             named arguments cannot be followed by positional arguments.",
                        ));
                    }

                    make_ident(format!("{}{}", prefix, i))
                };

                args.push(FormatArg {
                    local_variable: var_name,
                    expr: arg.expr,
                });

                prev_is_named_arg = is_named_arg;
            }
        }

        let mut unused_args = vec![true; args.len()];

        let format_str_span = unchecked_fargs.format_str_span;
        let first_named_arg = first_named_arg;
        let named_arg_names = named_arg_names;
        let args = args;

        let positional_args = &args[..first_named_arg];
        let named_args = &args[first_named_arg..];

        let fmt_str_components = unchecked_fargs.literal.list;

        let expanded_into: Vec<ExpandInto> = {
            let mut current_pos_arg = 0;
            let mut get_variable_name = |which_arg: WhichArg| -> Ident {
                match which_arg {
                    WhichArg::Ident(ident) => {
                        if let Some(pos) = named_arg_names.iter().position(|x| *x == ident) {
                            unused_args[pos + first_named_arg] = false;
                            named_args[pos].local_variable.clone()
                        } else {
                            // `formatcp!("{FOO}")` assumes that FOO is a constant in scope
                            ident
                        }
                    }
                    WhichArg::Positional(opt_pos) => {
                        let pos = opt_pos.unwrap_or_else(|| {
                            let pos = current_pos_arg;
                            current_pos_arg += 1;
                            pos
                        });

                        match positional_args.get(pos) {
                            Some(arg) => {
                                unused_args[pos] = false;
                                arg.local_variable.clone()
                            }
                            None => {
                                res.push_err(syn::Error::new(
                                    format_str_span,
                                    format!(
                                        "attempting to use nonexistent  positional argument `{}`",
                                        pos,
                                    ),
                                ));
                                dummy_ident()
                            }
                        }
                    }
                }
            };

            fmt_str_components
                .into_iter()
                .map(|fmt_str_comp| match fmt_str_comp {
                    FmtStrComponent::Str(str) => ExpandInto::Str(str),
                    FmtStrComponent::Arg(arg) => ExpandInto::Formatted(ExpandFormatted {
                        local_variable: get_variable_name(arg.which_arg),
                        format: arg.formatting,
                    }),
                })
                .collect()
        };

        for (i, (is_it_unused, (span, ident))) in
            unused_args.iter().zip(&arg_span_idents).enumerate()
        {
            if *is_it_unused {
                let msg = if let Some(ident) = ident {
                    format!("the '{}' argument is unused", ident)
                } else {
                    format!("argument number {} is unused", i)
                };
                res.push_err(syn::Error::new(*span, msg));
            }
        }
        res.take()?;

        Ok(FormatArgs {
            args,
            expanded_into,
        })
    }
}

////////////////////////////////////////////////

impl Parse for WriteArgs {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let prefix = Ident::new("const_fmt_local_", Span::call_site());

        let content;
        let _parentheses = syn::parenthesized!(content in input);
        let (writer_expr, _span) =
            content.parse_unwrap_tt(|content| Ok(content.parse_token_stream_and_span()))?;

        let format_args = FormatArgs::parse_with(input, prefix)?;

        Ok(Self {
            writer_expr,
            format_args,
        })
    }
}
