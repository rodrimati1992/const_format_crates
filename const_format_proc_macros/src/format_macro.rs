use crate::{
    format_args::{ExpandInto, FormatArgs, FormatIfArgs, LocalVariable, WriteArgs},
    parse_utils::{TokenStream2Ext, WithProcMacroArgs},
    shared_arg_parsing::{ExprArg, ExprArgs},
    Error,
};

use proc_macro2::{Ident, Span, TokenStream as TokenStream2};

use quote::{quote, quote_spanned};

#[cfg(test)]
mod tests;

////////////////////////////////////////////////////////////////////////////////

pub(crate) fn concatcp_impl(
    args: WithProcMacroArgs<ExprArgs>,
) -> Result<TokenStream2, crate::Error> {
    let cratep = args.crate_path.to_string().parse::<TokenStream2>().unwrap();

    let fmt_var = Ident::new("fmt", Span::mixed_site());

    let concat_args = args.value.args.iter().map(|ExprArg { expr, span }| {
        quote_spanned!(*span=>
            __cf_osRcTFl4A::pmr::PConvWrapper(#expr).to_pargument_display(#fmt_var)
        )
    });

    Ok(quote!(({
        use #cratep as __cf_osRcTFl4A;

        // The suffix is to avoid name collisions with identifiers in the passed-in expression.
        #[allow(unused_mut, non_snake_case)]
        const CONCATP_NHPMWYD3NJA : (usize, &[__cf_osRcTFl4A::pmr::PArgument]) = {
            let mut len = 0usize;

            let #fmt_var = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;

            let array = [
                #({
                    let arg = #concat_args;
                    len += arg.fmt_len;
                    arg
                }),*
            ];

            (len, &{array})
        };

        __cf_osRcTFl4A::__concatcp_inner!(CONCATP_NHPMWYD3NJA)
    })))
}

pub(crate) fn formatcp_impl(
    args: WithProcMacroArgs<FormatArgs>,
) -> Result<TokenStream2, crate::Error> {
    let cratep = args.crate_path.to_string().parse::<TokenStream2>().unwrap();
    let fmt_args = args.value;

    let locals = fmt_args
        .local_variables
        .iter()
        .map(|LocalVariable { ident, expr }| {
            let span = ident.span();
            quote_spanned!(span=> let #ident = #expr;)
        });

    for ei in fmt_args.expanded_into.iter() {
        if let ExpandInto::WithFormatter(wf) = ei {
            return Err(crate::Error::new(
                wf.fmt_ident.span(),
                "Cannot access the core_format::fmt::Formatter in the `formatcp` macro",
            ));
        }
    }

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
        ExpandInto::WithFormatter { .. } => unreachable!(),
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

pub(crate) fn formatc_if_macro_impl(
    WithProcMacroArgs { crate_path, value }: WithProcMacroArgs<FormatIfArgs>,
) -> Result<TokenStream2, crate::Error> {
    formatc_macro_impl(WithProcMacroArgs {
        crate_path,
        value: value.inner,
    })
}

////////////////////////////////////////////////////////////////////////////////

pub(crate) fn formatc_macro_impl(
    args: WithProcMacroArgs<FormatArgs>,
) -> Result<TokenStream2, crate::Error> {
    let cratep = args.crate_path.to_string().parse::<TokenStream2>().unwrap();

    let fmt_args = args.value;

    let locals = fmt_args.local_variables.iter().map(|arg| &arg.ident);
    let expr = fmt_args.local_variables.iter().map(|arg| &arg.expr);

    let strwriter = Ident::new("strwriter", Span::mixed_site());

    let writing_formatted = fmt_args
        .expanded_into
        .iter()
        .map(|ei| ei.fmt_call(&strwriter));

    let cond_a = fmt_args.condition.iter();
    let cond_b = fmt_args.condition.iter();

    Ok(quote!(({
        use #cratep as __cf_osRcTFl4A;

        #[allow(non_snake_case)]
        const fn fmt_NHPMWYD3NJA(
            mut #strwriter: __cf_osRcTFl4A::fmt::Formatter<'_>,
        ) -> __cf_osRcTFl4A::Result {
            match (#(&(#expr),)*) {
                (#(#locals,)*) => {
                    #(
                        __cf_osRcTFl4A::try_!(#writing_formatted);
                    )*
                },
            }
            __cf_osRcTFl4A::pmr::Ok(())
        }

        const fn len_nhpmwyd3nj() -> usize {
            if  #((#cond_a) && )* true  {
                let mut strlen = __cf_osRcTFl4A::pmr::ComputeStrLength::new();
                let fmt = strlen.make_formatter(__cf_osRcTFl4A::FormattingFlags::NEW);
                match fmt_NHPMWYD3NJA(fmt) {
                    __cf_osRcTFl4A::pmr::Ok(()) => strlen.len(),
                    __cf_osRcTFl4A::pmr::Err(_) => 0,
                }
            } else {
                0
            }
        }

        const LEN_NHPMWYD3NJA: usize = len_nhpmwyd3nj();

        const fn str_writer_NHPMWYD3NJA(
        )-> __cf_osRcTFl4A::msg::ErrorTupleAndStrWriter<[u8; LEN_NHPMWYD3NJA]> {
            let mut #strwriter = __cf_osRcTFl4A::pmr::StrWriter::new([0; LEN_NHPMWYD3NJA]);
            let mut error = if #((#cond_b) && )* true {
                fmt_NHPMWYD3NJA(
                    __cf_osRcTFl4A::pmr::Formatter::from_sw(
                        &mut #strwriter,
                        __cf_osRcTFl4A::FormattingFlags::NEW,
                    )
                )
            } else {
                __cf_osRcTFl4A::pmr::Ok(())
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
        condition: _,
        expanded_into,
        local_variables,
    } = args.value.format_args;

    let locals = local_variables.iter().map(|arg| &arg.ident);
    let expr = local_variables.iter().map(|arg| &arg.expr);

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
                            |e| break __cf_osRcTFl4A::pmr::Err(e)
                        );
                    )*
                    break __cf_osRcTFl4A::pmr::Ok(());
                }
            }
        }
    })})
}
