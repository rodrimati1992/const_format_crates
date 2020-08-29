use crate::{
    format_args::{ExpandInto, FormatArgs, WriteArgs},
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

    let locals_a = fmt_args.args.iter().map(|arg| &arg.local_variable);
    let locals_b = locals_a.clone();
    let expr_a = fmt_args.args.iter().map(|arg| &arg.expr);
    let expr_b = expr_a.clone();

    let strlen = Ident::new("strlen", Span::mixed_site());
    let strwriter = Ident::new("strwriter", Span::mixed_site());

    let length_computation = fmt_args
        .expanded_into
        .iter()
        .map(|ei| ei.len_call(&cratep, &strlen));

    let writing_formatted = fmt_args
        .expanded_into
        .iter()
        .map(|ei| ei.fmt_call(&cratep, &strwriter));

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
            let mut error = match (#(&(#expr_b),)*) {
                (#(#locals_b,)*) => loop {
                    let mut #strwriter = #cratep::pmr::StrWriterMut::new(&mut #strwriter);
                    #(
                        #cratep::unwrap_or_else!(
                            #writing_formatted,
                            |e| break Some(e)
                        );
                    )*
                    break #cratep::pmr::None::<#cratep::pmr::Error>;
                },
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
                #cratep::pmr::transmute::<
                    &[u8; LEN_NHPMWYD3NJA],
                    &[u8; STR_WRITER_NHPMWYD3NJA.writer.len()],
                >(
                    STR_WRITER_NHPMWYD3NJA.writer.buffer(),
                );

            #cratep::pmr::transmute::<&[u8], &str>(slice)
        };

        STR_NHPMWYD3NJA
    }))
}

pub(crate) fn writec_macro_impl(
    args: WithProcMacroArgs<WriteArgs>,
) -> Result<TokenStream2, syn::Error> {
    let cratep = args.crate_path;
    // Had to launder the path, to not get weird errors about crate being a visibility modifier.
    let cratep = syn::parse_str::<TokenStream2>(&cratep.to_string()).unwrap();

    let writer_expr = args.value.writer_expr;
    let FormatArgs {
        expanded_into,
        args,
    } = args.value.format_args;

    let locals = args.iter().map(|arg| &arg.local_variable);
    let expr = args.iter().map(|arg| &arg.expr);

    let strwriter = Ident::new("strwriter", Span::mixed_site());

    let writing_formatted = expanded_into
        .iter()
        .map(|ei| ei.fmt_call(&cratep, &strwriter));

    Ok(quote! {
        #[allow(non_snake_case)]
        match ((#writer_expr).borrow_mutably(), #(&(#expr),)*) {
            (#strwriter, #(#locals,)*) => {
                let mut marker = #cratep::pmr::IsAWriteMarker::NEW;
                if false {
                    marker = marker.infer_type(&#strwriter);
                }
                let mut #strwriter = marker.coerce(#strwriter);

                loop {
                    #(
                        #cratep::unwrap_or_else!(
                            #writing_formatted,
                            |e| break Err(e)
                        );
                    )*
                    break Ok(());
                }
            }
        }
    })
}
