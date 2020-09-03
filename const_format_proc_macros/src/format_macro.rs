use crate::{
    format_args::{ExpandInto, FormatArgs, WriteArgs},
    parse_utils::{TokenStream2Ext, WithProcMacroArgs},
    Error,
};

use proc_macro2::{Ident, Span, TokenStream as TokenStream2};

use quote::{quote, quote_spanned};

#[cfg(test)]
mod tests;

////////////////////////////////////////////////////////////////////////////////

pub(crate) fn macro_impl(
    args: WithProcMacroArgs<FormatArgs>,
) -> Result<TokenStream2, crate::Error> {
    let cratep = args.crate_path.to_string().parse::<TokenStream2>().unwrap();
    let fmt_args = args.value;

    let locals = fmt_args.args.iter().map(|arg| {
        let local_variable = &arg.local_variable;
        let expr = &arg.expr;
        let span = local_variable.span();
        quote_spanned!(span=> let #local_variable = #expr;)
    });

    let parg_constructor = fmt_args.expanded_into.iter().map(|ei| match ei {
        ExpandInto::Str(str, rawness) => {
            let str_tokens = rawness.tokenize_sub(str);
            quote!(
                __cf_osRcTFl4A::pmr::PConvWrapper(#str_tokens)
                    .to_pargument_display(__cf_osRcTFl4A::pmr::FormattingFlags::NEW)
            )
        }
        ExpandInto::Formatted(fmted) => {
            let to_pargument_m = fmted.format.to_pargument_method_name();
            let formatting = fmted.format;
            let local_variable = &fmted.local_variable;
            let span = local_variable.span();
            // I had to use `set_span_recursive` to set the span to that of the argument,
            // quote_span doesn't work for that somehow.
            quote!(
                __cf_osRcTFl4A::pmr::PConvWrapper(#local_variable).#to_pargument_m(#formatting)
            )
            .set_span_recursive(span)
        }
    });

    Ok(quote!(({
        use #cratep as __cf_osRcTFl4A;

        // The suffix is to avoid name collisions with identifiers in the passed-in expression.
        #[allow(unused_mut, non_snake_case)]
        const CONCATP_NHPMWYD3NJA : (usize, &[__cf_osRcTFl4A::pmr::PArgument]) = {
            let mut len = 0usize;

            #( #locals )*

            let array = [
                #({
                    let arg = #parg_constructor;
                    len += arg.fmt_len;
                    arg
                }),*
            ];

            (len, &{array})
        };

        __cf_osRcTFl4A::__concatcp_inner!(CONCATP_NHPMWYD3NJA)
    })))
}

////////////////////////////////////////////////////////////////////////////////

pub(crate) fn formatc_macro_impl(
    args: WithProcMacroArgs<FormatArgs>,
) -> Result<TokenStream2, crate::Error> {
    let cratep = args.crate_path.to_string().parse::<TokenStream2>().unwrap();

    let fmt_args = args.value;

    let locals_a = fmt_args.args.iter().map(|arg| &arg.local_variable);
    let locals_b = locals_a.clone();
    let expr_a = fmt_args.args.iter().map(|arg| &arg.expr);
    let expr_b = expr_a.clone();

    let strlen = Ident::new("strlen", Span::mixed_site());
    let strwriter = Ident::new("strwriter", Span::mixed_site());

    let length_computation = fmt_args.expanded_into.iter().map(|ei| ei.len_call(&strlen));

    let writing_formatted = fmt_args
        .expanded_into
        .iter()
        .map(|ei| ei.fmt_call(&strwriter));

    Ok(quote!(({
        use #cratep as __cf_osRcTFl4A;

        const fn len_NHPMWYD3NJA() ->  usize {
            let mut #strlen = __cf_osRcTFl4A::pmr::ComputeStrLength::new();
            match (#(&(#expr_a),)*) {
                (#((#locals_a),)*) => {
                    #(#length_computation)*
                }
            };
            #strlen.len()
        }
        const LEN_NHPMWYD3NJA: usize = len_NHPMWYD3NJA();

        const fn str_writer_NHPMWYD3NJA(
        )-> __cf_osRcTFl4A::msg::ErrorTupleAndStrWriter<[u8; LEN_NHPMWYD3NJA]> {
            let mut #strwriter = __cf_osRcTFl4A::pmr::StrWriter::new([0; LEN_NHPMWYD3NJA]);
            let mut error = match (#(&(#expr_b),)*) {
                (#(#locals_b,)*) => loop {
                    let mut #strwriter = __cf_osRcTFl4A::pmr::StrWriterMut::new(&mut #strwriter);
                    #(
                        __cf_osRcTFl4A::unwrap_or_else!(
                            #writing_formatted,
                            |e| break Some(e)
                        );
                    )*
                    break __cf_osRcTFl4A::pmr::None::<__cf_osRcTFl4A::pmr::Error>;
                },
            };

            __cf_osRcTFl4A::msg::ErrorTupleAndStrWriter{
                error: __cf_osRcTFl4A::msg::ErrorTuple::new(error, &#strwriter),
                writer: #strwriter,
            }
        }

        const STR_WRITER_NHPMWYD3NJA:
            &__cf_osRcTFl4A::msg::ErrorTupleAndStrWriter<[u8; LEN_NHPMWYD3NJA]>=
            &str_writer_NHPMWYD3NJA();

        const _: __cf_osRcTFl4A::msg::Ok = {
            <
                <
                    __cf_osRcTFl4A::msg::ErrorPicker<
                        [(); STR_WRITER_NHPMWYD3NJA.error.error_variant],
                        [(); STR_WRITER_NHPMWYD3NJA.error.capacity]
                    >
                    as __cf_osRcTFl4A::msg::ErrorAsType
                >::Type
            >::NEW
        };

        const STR_NHPMWYD3NJA: &str =
            __cf_osRcTFl4A::strwriter_as_str!(&STR_WRITER_NHPMWYD3NJA.writer);

        STR_NHPMWYD3NJA
    })))
}

pub(crate) fn writec_macro_impl(args: WithProcMacroArgs<WriteArgs>) -> Result<TokenStream2, Error> {
    let cratep = args.crate_path.to_string().parse::<TokenStream2>().unwrap();

    let writer_expr = args.value.writer_expr;
    let FormatArgs {
        expanded_into,
        args,
    } = args.value.format_args;

    let locals = args.iter().map(|arg| &arg.local_variable);
    let expr = args.iter().map(|arg| &arg.expr);

    let strwriter = Ident::new("strwriter", Span::mixed_site());

    let writing_formatted = expanded_into.iter().map(|ei| ei.fmt_call(&strwriter));

    Ok(quote! {({
        use #cratep as __cf_osRcTFl4A;

        #[allow(non_snake_case)]
        match ((#writer_expr).borrow_mutably(), #(&(#expr),)*) {
            (#strwriter, #(#locals,)*) => {
                let mut marker = __cf_osRcTFl4A::pmr::IsAWriteMarker::NEW;
                if false {
                    marker = marker.infer_type(&#strwriter);
                }
                let mut #strwriter = marker.coerce(#strwriter);
                let mut #strwriter =
                    #strwriter.make_formatter(__cf_osRcTFl4A::FormattingFlags::NEW);

                loop {
                    #(
                        __cf_osRcTFl4A::unwrap_or_else!(
                            #writing_formatted,
                            |e| break Err(e)
                        );
                    )*
                    break Ok(());
                }
            }
        }
    })})
}
