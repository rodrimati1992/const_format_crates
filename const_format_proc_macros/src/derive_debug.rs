use crate::datastructure::{DataStructure, DataVariant, Field, StructKind};

use proc_macro2::{Span, TokenStream as TokenStream2};

use quote::{quote, quote_spanned, TokenStreamExt};

use syn::{DeriveInput, Ident};

mod attribute_parsing;
mod syntax;

use self::attribute_parsing::HowToFmt;

pub(crate) fn derive_constdebug_impl(input: DeriveInput) -> Result<TokenStream2, syn::Error> {
    let ds = &DataStructure::new(&input);
    let config = attribute_parsing::parse_attrs_for_derive(ds)?;
    let cratep = quote!(::const_format);
    let vis = ds.vis;

    let name = ds.name;

    let mut impl_headers;

    if config.impls.is_empty() {
        let impl_params = ds.generics.params.iter();
        let (_, tygen, _) = ds.generics.split_for_impl();
        let where_clause = get_where_clause_tokens(&ds.generics.where_clause);

        impl_headers = quote!(
            impl[#( #impl_params ,)*] #name #tygen
            #where_clause;
        );
    } else {
        impl_headers = TokenStream2::new();

        for imp in config.impls.iter() {
            let params = imp.generics.params.iter();
            let self_ty = &imp.self_ty;
            let where_clause = get_where_clause_tokens(&imp.generics.where_clause);

            impl_headers.append_all(quote!(
                impl[#(#params)*] #self_ty
                #where_clause;
            ));
        }
    };

    let enum_prefix = match ds.data_variant {
        DataVariant::Enum => quote!(#name::),
        DataVariant::Struct => TokenStream2::new(),
        DataVariant::Union => panic!("Cannot derive ConstDebug on unions"),
    };

    let variant_branches = ds.variants.iter().map(|variant| {
        let vname = variant.name;

        let debug_method = match variant.kind {
            StructKind::Braced => Ident::new("debug_struct", Span::call_site()),
            StructKind::Tupled => Ident::new("debug_tuple", Span::call_site()),
        };

        let patt = variant
            .fields
            .iter()
            .filter_map(|f| -> Option<TokenStream2> {
                if let HowToFmt::Ignore = config.field_map[f].how_to_fmt {
                    return None;
                }

                let pat = &f.ident;
                let variable = f.pattern_ident();

                Some(quote!(#pat : #variable,))
            });

        let fmt_call = variant
            .fields
            .iter()
            .filter_map(|f| -> Option<TokenStream2> {
                let how_to_fmt = &config.field_map[f].how_to_fmt;
                if let HowToFmt::Ignore = how_to_fmt {
                    return None;
                }

                let field_span = f.pattern_ident().span();

                let field_name_str = match variant.kind {
                    StructKind::Braced => Some(f.ident.to_string()),
                    StructKind::Tupled => None,
                }
                .into_iter();

                let mut field_ts = quote_spanned!(field_span=>
                    let mut field_formatter = formatter.field(#(#field_name_str)*);
                );

                match how_to_fmt {
                    HowToFmt::Regular => field_ts.append_all(coerce_and_fmt(&cratep, f)),
                    HowToFmt::Ignore => unreachable!(),
                }

                Some(field_ts)
            });

        quote!(
            #enum_prefix #vname { #(#patt)* .. } => {
                let mut formatter = formatter.#debug_method(stringify!(#vname));
                #(#fmt_call)*
                formatter.finish()
            }
        )
    });

    let ret = quote!(
        #cratep::impl_fmt!{
            #impl_headers

            #vis const fn const_debug_fmt(
                &self,
                formatter: &mut #cratep::pmr::Formatter<'_>,
            ) -> #cratep::pmr::Result<(), #cratep::pmr::Error> {
                match self {
                    #(
                        #variant_branches
                    )*
                }
            }
        }
    );

    if config.debug_print {
        panic!("\n\n\n{}\n\n\n", ret);
    }
    Ok(ret)
}

// Copying the definitino of the `const_format::coerce_to_fn` macro here
// because the compiler points inside the coerce_to_fn macro otherwise
fn coerce_and_fmt(cratep: &TokenStream2, field: &Field<'_>) -> TokenStream2 {
    let field_ty = field.ty;
    let var = field.pattern_ident();
    let field_span = var.span();

    quote_spanned!(field_span=>
        let marker = <#field_ty as #cratep::pmr::GetTypeKind>::KIND;
        #cratep::try_!(
            marker.coerce(marker.unreference(#var))
                .const_debug_fmt(field_formatter)
        );
    )
}

fn get_where_clause_tokens(where_clause: &Option<syn::WhereClause>) -> TokenStream2 {
    match where_clause {
        Some(x) => {
            let preds = x.predicates.iter();
            quote!(where[ #(#preds,)* ])
        }
        None => TokenStream2::new(),
    }
}
