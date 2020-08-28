use proc_macro2::Span;

use quote::ToTokens;

use std::{
    fmt::Display,
    mem::{self, ManuallyDrop},
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
pub struct LinearResult<T> {
    errors: ManuallyDrop<Result<T, syn::Error>>,
}

impl<T> Drop for LinearResult<T> {
    fn drop(&mut self) {
        let res = unsafe { ManuallyDrop::take(&mut self.errors) };
        res.expect("Expected LinearResult to be handled");
    }
}

impl<T> LinearResult<T> {
    #[inline]
    pub fn new(res: Result<T, syn::Error>) -> Self {
        Self {
            errors: ManuallyDrop::new(res),
        }
    }

    #[inline]
    pub fn ok(value: T) -> Self {
        Self::new(Ok(value))
    }
}

impl<T> From<Result<T, syn::Error>> for LinearResult<T> {
    #[inline]
    fn from(res: Result<T, syn::Error>) -> Self {
        Self::new(res)
    }
}

impl<T> Deref for LinearResult<T> {
    type Target = Result<T, syn::Error>;

    fn deref(&self) -> &Result<T, syn::Error> {
        &self.errors
    }
}

impl<T> DerefMut for LinearResult<T> {
    fn deref_mut(&mut self) -> &mut Result<T, syn::Error> {
        &mut self.errors
    }
}

impl<T> Into<Result<T, syn::Error>> for LinearResult<T> {
    #[inline]
    fn into(self) -> Result<T, syn::Error> {
        self.into_result()
    }
}

#[allow(dead_code)]
impl<T> LinearResult<T> {
    #[inline]
    pub fn into_result(self) -> Result<T, syn::Error> {
        let mut this = ManuallyDrop::new(self);
        unsafe { ManuallyDrop::take(&mut this.errors) }
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
        mem::replace(&mut *self.errors, other)
    }

    #[inline]
    pub fn push_err(&mut self, err: syn::Error) {
        match &mut *self.errors {
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
