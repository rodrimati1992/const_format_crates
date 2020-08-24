use crate::{
    format_args::{ExpandInto, FormatArgs},
    formatting::FormattingFlags,
    parse_utils::WithProcMacroArgs,
};

use proc_macro2::{Span, TokenStream as TokenStream2};

use quote::{quote, quote_spanned};

use syn::Ident;

#[cfg(test)]
mod tests;

////////////////////////////////////////////////////////////////////////////////

pub(crate) fn macro_impl(args: WithProcMacroArgs<FormatArgs>) -> Result<TokenStream2, syn::Error> {
    let cratep = args.crate_path;
    let fmt_args = args.value;

    let locals = fmt_args.args.iter().map(|arg| {
        let local_variable = &arg.local_variable;
        let expr = &arg.expr;
        let span = local_variable.span();
        quote_spanned!(span=> #local_variable, #expr)
    });

    let arg_pairs = fmt_args.expanded_into.iter().map(|ei| match ei {
        ExpandInto::Str(str) => {
            quote!(to_pargument_display, #cratep::pmr::FormattingFlags::NEW, #str)
        }
        ExpandInto::Formatted(fmted) => {
            let to_pargument_m = fmted.format.to_pargument_method_name();
            let formatting = fmted.format.tokens(&cratep);
            let local_variable = &fmted.local_variable;
            let span = local_variable.span();
            quote_spanned!(span=> #to_pargument_m, #formatting, #local_variable)
        }
    });

    Ok(quote!(
        #cratep::concatcp!(
            @with_fmt
            locals(#( (#locals) )*)
            #((#arg_pairs)) *
        )
    ))
}

////////////////////////////////////////////////////////////////////////////////

pub(crate) fn formatc_macro_impl(
    args: WithProcMacroArgs<FormatArgs>,
) -> Result<TokenStream2, syn::Error> {
    let cratep = args.crate_path;
    // Had to launder the path, to not get weird errors about crate being a visibility modifier.
    let cratep = syn::parse_str::<TokenStream2>(&cratep.to_string()).unwrap();

    let fmt_args = args.value;

    let formatting = fmt_args
        .expanded_into
        .iter()
        .map(|ei| {
            match ei {
                ExpandInto::Str { .. } => FormattingFlags::NEW,
                ExpandInto::Formatted(fmted) => fmted.format,
            }
            .tokens(&cratep)
        })
        .collect::<Vec<TokenStream2>>();

    let formatting = formatting.as_slice();

    let locals_a = fmt_args.args.iter().map(|arg| &arg.local_variable);
    let locals_b = locals_a.clone();
    let expr_a = fmt_args.args.iter().map(|arg| &arg.expr);
    let expr_b = expr_a.clone();

    let error_scope = quote!('error_scope);
    let strlen = Ident::new("strlen", Span::mixed_site());
    let strwriter = Ident::new("strwriter", Span::mixed_site());

    let length_computation = fmt_args
        .expanded_into
        .iter()
        .zip(formatting)
        .map(|(ei, flags)| match ei {
            ExpandInto::Str(str) => {
                let len = str.len();
                quote!( #strlen.add_len(#len); )
            }
            ExpandInto::Formatted(fmted) => {
                let len_method = fmted.format.len_method_name();
                let local_variable = &fmted.local_variable;
                let span = local_variable.span();

                quote_spanned!(span=>
                    #cratep::coerce_to_fmt!(#local_variable)
                        .#len_method(&mut #strlen.make_formatting_length(#flags));
                )
            }
        });

    let writing_formatted =
        fmt_args
            .expanded_into
            .iter()
            .zip(formatting)
            .map(|(ei, flags)| match ei {
                ExpandInto::Str(str) => quote!( #strwriter.write_whole_str(#str) ),
                ExpandInto::Formatted(fmted) => {
                    let fmt_method = fmted.format.fmt_method_name();
                    let local_variable = &fmted.local_variable;
                    let span = local_variable.span();

                    quote_spanned!(span=>
                        #cratep::coerce_to_fmt!(&#local_variable)
                            .#fmt_method(&mut #strwriter.make_formatter(#flags))
                    )
                }
            });

    Ok(quote!({
        const fn len_NHPMWYD3NJA() ->  usize {
            let mut #strlen = #cratep::pmr::ComputeStrLength::new();
            match (#(&(#expr_a),)*) {
                (#((#locals_a),)*) => {
                    #(#length_computation)*
                }
            };
            #strlen.len()
        }
        const LEN_NHPMWYD3NJA: usize = len_NHPMWYD3NJA();

        const fn str_writer_NHPMWYD3NJA(
        )-> #cratep::msg::ErrorTupleAndStrWriter<[u8; LEN_NHPMWYD3NJA]> {
            let mut #strwriter = #cratep::pmr::StrWriter::new([0; LEN_NHPMWYD3NJA]);
            let mut error = #cratep::block!{ #error_scope:
                let #strwriter: &mut #cratep::pmr::StrWriter = &mut #strwriter;

                match (#(&(#expr_b),)*) {
                    (#(#locals_b,)*) => {
                        #(
                            #cratep::unwrap_or_else!(
                                #writing_formatted,
                                |e| break #error_scope Some(e)
                            );
                        )*
                    }
                }
                #cratep::pmr::None::<#cratep::pmr::Error>
            };


            #cratep::msg::ErrorTupleAndStrWriter{
                error: #cratep::msg::ErrorTuple::new(error, &#strwriter),
                writer: #strwriter,
            }
        }

        const STR_WRITER_NHPMWYD3NJA: &#cratep::msg::ErrorTupleAndStrWriter<[u8; LEN_NHPMWYD3NJA]>=
            &str_writer_NHPMWYD3NJA();

        const _: #cratep::msg::Ok = {
            <
                <
                    #cratep::msg::ErrorPicker<
                        [(); STR_WRITER_NHPMWYD3NJA.error.error_variant],
                        [(); STR_WRITER_NHPMWYD3NJA.error.capacity]
                    >
                    as #cratep::msg::ErrorAsType
                >::Type
            >::NEW
        };

        // #cratep::make_error_tuple!(
        //
        // );

        const STR_NHPMWYD3NJA: &str = unsafe{
            // This transmute truncates the length of the array to the amound of written bytes.
            let slice =
                #cratep::pmr::Transmute::<
                    &[u8; LEN_NHPMWYD3NJA],
                    &[u8; STR_WRITER_NHPMWYD3NJA.writer.len()]
                >{
                    from: STR_WRITER_NHPMWYD3NJA.writer.buffer(),
                }.to;

            #cratep::pmr::Transmute::<&[u8], &str>{from: slice}.to
        };

        STR_NHPMWYD3NJA
    }))
}
