use crate::{
    datastructure::{DataStructure, Field, FieldMap},
    utils::LinearResult,
};

use super::syntax::ImplHeader;

use quote::ToTokens;

use syn::{Attribute, Meta, MetaList, NestedMeta};

use std::marker::PhantomData;

pub(crate) struct ConstDebugConfig<'a> {
    pub(crate) debug_print: bool,
    pub(crate) impls: Vec<ImplHeader>,
    pub(crate) field_map: FieldMap<FieldConfig>,
    _marker: PhantomData<&'a ()>,
}

impl<'a> ConstDebugConfig<'a> {
    fn new(roa: ConstDebugAttrs<'a>) -> Result<Self, syn::Error> {
        let ConstDebugAttrs {
            debug_print,
            impls,
            field_map,
            errors: _,
            _marker: PhantomData,
        } = roa;

        Ok(Self {
            debug_print,
            impls,
            field_map,
            _marker: PhantomData,
        })
    }
}

struct ConstDebugAttrs<'a> {
    debug_print: bool,
    impls: Vec<ImplHeader>,
    field_map: FieldMap<FieldConfig>,
    errors: LinearResult<()>,
    _marker: PhantomData<&'a ()>,
}

////////////////////////////////////////////////////////////////////////////////

pub(crate) struct FieldConfig {
    pub(crate) how_to_fmt: HowToFmt,
}

pub(crate) enum HowToFmt {
    // `coerce_to_fmt!(&field).const_debug_fmt(f)`
    Regular,
    // `Doesn't print the field.
    Ignore,
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone)]
enum ParseContext<'a> {
    TypeAttr,
    Field { field: &'a Field<'a> },
}

pub(crate) fn parse_attrs_for_derive<'a>(
    ds: &'a DataStructure<'a>,
) -> Result<ConstDebugConfig<'a>, syn::Error> {
    let mut this = ConstDebugAttrs {
        debug_print: false,
        impls: Vec::new(),
        field_map: FieldMap::with(ds, |_| FieldConfig {
            how_to_fmt: HowToFmt::Regular,
        }),
        errors: LinearResult::ok(()),
        _marker: PhantomData,
    };

    let ty_ctx = ParseContext::TypeAttr;
    parse_inner(&mut this, ds.attrs, ty_ctx)?;

    for variant in &ds.variants {
        for field in variant.fields.iter() {
            parse_inner(&mut this, field.attrs, ParseContext::Field { field })?;
        }
    }

    this.errors.take()?;

    ConstDebugConfig::new(this)
}

/// Parses an individual attribute
fn parse_inner<'a, I>(
    this: &mut ConstDebugAttrs<'a>,
    attrs: I,
    pctx: ParseContext<'a>,
) -> Result<(), syn::Error>
where
    I: IntoIterator<Item = &'a Attribute>,
{
    for attr in attrs {
        match attr.parse_meta() {
            Ok(Meta::List(list)) => {
                let x = parse_attr_list(this, pctx, list);
                this.errors.combine_err(x);
            }
            Err(e) => {
                this.errors.push_err(e);
            }
            _ => {}
        }
    }
    Ok(())
}

/// Parses an individual attribute list (A `#[attribute( .. )] attribute`).
fn parse_attr_list<'a>(
    this: &mut ConstDebugAttrs<'a>,
    pctx: ParseContext<'a>,
    list: MetaList,
) -> Result<(), syn::Error> {
    if list.path.is_ident("cdeb") {
        with_nested_meta("cdeb", list.nested, |attr| {
            let x = parse_sabi_attr(this, pctx, attr);
            this.errors.combine_err(x);
            Ok(())
        })?;
    }

    Ok(())
}

/// Parses the contents of a `#[sabi( .. )]` attribute.
fn parse_sabi_attr<'a>(
    this: &mut ConstDebugAttrs<'a>,
    pctx: ParseContext<'a>,
    attr: Meta,
) -> Result<(), syn::Error> {
    fn make_err(tokens: &dyn ToTokens) -> syn::Error {
        spanned_err!(tokens, "unrecognized attribute")
    }
    match (pctx, attr) {
        (ParseContext::Field { field, .. }, Meta::Path(path)) => {
            let f_config = &mut this.field_map[field.index];

            if path.is_ident("ignore") {
                f_config.how_to_fmt = HowToFmt::Ignore;
            } else {
                return Err(make_err(&path));
            }
        }
        (ParseContext::TypeAttr { .. }, Meta::Path(path)) => {
            if path.is_ident("debug_print") {
                this.debug_print = true;
            } else {
                return Err(make_err(&path));
            }
        }
        (ParseContext::TypeAttr { .. }, Meta::List(list)) => {
            if list.path.is_ident("impls") {
                for x in list.nested {
                    let lit = match x {
                        NestedMeta::Meta(attr) => return Err(make_err(&attr)),
                        NestedMeta::Lit(lit) => lit,
                    };
                    this.impls.push(parse_lit::<ImplHeader>(&lit)?);
                }
            } else {
                return Err(make_err(&list));
            }
        }
        (_, x) => return Err(make_err(&x)),
    }
    Ok(())
}

///////////////////////////////////////////////////////////////////////////////

fn parse_lit<T>(lit: &syn::Lit) -> Result<T, syn::Error>
where
    T: syn::parse::Parse,
{
    match lit {
        syn::Lit::Str(x) => x.parse(),
        _ => Err(spanned_err!(
            lit,
            "Expected string literal containing identifier"
        )),
    }
}

#[allow(dead_code)]
fn parse_expr(lit: syn::Lit) -> Result<syn::Expr, syn::Error> {
    match lit {
        syn::Lit::Str(x) => x.parse(),
        syn::Lit::Int(x) => syn::parse_str(x.base10_digits()),
        _ => return_spanned_err!(lit, "Expected string or integer literal"),
    }
}

///////////////////////////////////////////////////////////////////////////////

pub fn with_nested_meta<I, F>(attr_name: &str, iter: I, mut f: F) -> Result<(), syn::Error>
where
    F: FnMut(Meta) -> Result<(), syn::Error>,
    I: IntoIterator<Item = NestedMeta>,
{
    for repr in iter {
        match repr {
            NestedMeta::Meta(attr) => {
                f(attr)?;
            }
            NestedMeta::Lit(lit) => {
                return_spanned_err!(
                    lit,
                    "the #[{}(...)] attribute does not allow literals in the attribute list",
                    attr_name,
                );
            }
        }
    }
    Ok(())
}
