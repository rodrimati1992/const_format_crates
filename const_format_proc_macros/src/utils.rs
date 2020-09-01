use proc_macro2::Span;

#[cfg(feature = "derive")]
use quote::ToTokens;

use std::{
    fmt::Display,
    mem,
    ops::{Deref, DerefMut},
};

pub(crate) fn dummy_ident() -> syn::Ident {
    syn::Ident::new("__dummy__", Span::mixed_site())
}

////////////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "derive")]
pub fn spanned_err(tokens: &dyn ToTokens, display: &dyn Display) -> syn::Error {
    syn::Error::new_spanned(tokens, display)
}

#[allow(dead_code)]
pub fn syn_err(span: Span, display: &dyn Display) -> syn::Error {
    syn::Error::new(span, display)
}

////////////////////////////////////////////////////////////////////////////////////

/// A result wrapper which panics if it's the error variant is not handled,
/// by calling `.into_result()`.
#[derive(Debug, Clone)]
pub struct LinearResult<T: Default> {
    errors: Result<T, syn::Error>,
}

impl<T: Default> Drop for LinearResult<T> {
    fn drop(&mut self) {
        mem::replace(&mut self.errors, Ok(T::default()))
            .expect("Expected LinearResult to be handled");
    }
}

impl<T: Default> LinearResult<T> {
    #[inline]
    pub fn new(res: Result<T, syn::Error>) -> Self {
        Self { errors: res }
    }

    #[inline]
    pub fn ok(value: T) -> Self {
        Self::new(Ok(value))
    }
}

impl<T: Default> From<Result<T, syn::Error>> for LinearResult<T> {
    #[inline]
    fn from(res: Result<T, syn::Error>) -> Self {
        Self::new(res)
    }
}

impl<T: Default> Deref for LinearResult<T> {
    type Target = Result<T, syn::Error>;

    fn deref(&self) -> &Result<T, syn::Error> {
        &self.errors
    }
}

impl<T: Default> DerefMut for LinearResult<T> {
    fn deref_mut(&mut self) -> &mut Result<T, syn::Error> {
        &mut self.errors
    }
}

impl<T: Default> Into<Result<T, syn::Error>> for LinearResult<T> {
    #[inline]
    fn into(self) -> Result<T, syn::Error> {
        self.into_result()
    }
}

#[allow(dead_code)]
impl<T: Default> LinearResult<T> {
    #[inline]
    pub fn into_result(mut self) -> Result<T, syn::Error> {
        mem::replace(&mut self.errors, Ok(T::default()))
    }

    #[inline]
    pub fn take(&mut self) -> Result<T, syn::Error>
    where
        T: Default,
    {
        self.replace(Ok(Default::default()))
    }

    #[inline]
    pub fn replace(&mut self, other: Result<T, syn::Error>) -> Result<T, syn::Error> {
        mem::replace(&mut self.errors, other)
    }

    #[inline]
    pub fn push_err(&mut self, err: syn::Error) {
        match &mut self.errors {
            this @ Ok(_) => *this = Err(err),
            Err(e) => e.combine(err),
        }
    }

    #[inline]
    pub fn combine_err<T2>(&mut self, res: Result<T2, syn::Error>) {
        if let Err(err) = res {
            self.push_err(err);
        }
    }
}
