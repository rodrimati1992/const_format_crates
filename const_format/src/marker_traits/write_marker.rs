//! Marker trait for types that can be written to..
//!
//!

use crate::fmt::{Formatter, StrWriter, StrWriterMut};

use core::marker::PhantomData;

////////////////////////////////////////////////////////////////////////////////

/// Marker trait for types that implement the const formatting methods.
///
///
///
pub trait WriteMarker {
    /// Whether this is a StrWriter or not, this can be either of
    /// [`IsAStrWriter`] or [`IsNotAStrWriter`]
    ///
    /// [`IsAStrWriter`]: ./struct.IsAStrWriter.html
    /// [`IsNotAStrWriter`]: ./struct.IsNotAStrWriter.html
    type Kind;
    type This: ?Sized;

    const KIND: IsAWriteMarker<Self::Kind, Self::This, Self> = IsAWriteMarker::NEW;
}

/// Marker type for `StrWriter`'s [`Kind`] in [`WriteMarker`]s
///
/// [`Kind`]: ./trait.Write2Marker.html#associatedtype.Kind
/// [`WriteMarker`]: ./trait.Write2Marker.html
///
pub struct IsAStrWriter;

/// Marker type for the [`Kind`] of all non-`StrWriter` types that implement [`WriteMarker`]s
///
/// [`Kind`]: ./trait.Write2Marker.html#associatedtype.Kind
/// [`WriteMarker`]: ./trait.Write2Marker.html
///
pub struct IsNotAStrWriter;

///////////////////////////////////////////////////////////////////////////////

impl<T: ?Sized> WriteMarker for StrWriter<T> {
    type Kind = IsAStrWriter;
    type This = Self;
}

impl WriteMarker for StrWriterMut<'_> {
    type Kind = IsNotAStrWriter;
    type This = Self;
}

impl WriteMarker for Formatter<'_> {
    type Kind = IsNotAStrWriter;
    type This = Self;
}

impl<T> WriteMarker for &T
where
    T: ?Sized + WriteMarker,
{
    type Kind = T::Kind;
    type This = T::This;
}

impl<T> WriteMarker for &mut T
where
    T: ?Sized + WriteMarker,
{
    type Kind = T::Kind;
    type This = T::This;
}

///////////////////////////////////////////////////////////////////////////////

/// Hack used to automcatically automatically convert a
/// mutable reference to a [`StrWriter`] to a [`StrWriterMut`],
/// and do nothing with other types.
///
///
/// [`StrWriter`]: ../fmt/struct.StrWriter.html
///
/// [`StrWriterMut`]: ../fmt/struct.StrWriterMut.html
///
pub struct IsAWriteMarker<K, T: ?Sized, R: ?Sized>(
    PhantomData<(
        PhantomData<fn() -> PhantomData<K>>,
        PhantomData<fn() -> PhantomData<T>>,
        PhantomData<fn() -> PhantomData<R>>,
    )>,
);

impl<K, T: ?Sized, R: ?Sized> Copy for IsAWriteMarker<K, T, R> {}

impl<K, T: ?Sized, R: ?Sized> Clone for IsAWriteMarker<K, T, R> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<R> IsAWriteMarker<R::Kind, R::This, R>
where
    R: ?Sized + WriteMarker,
{
    pub const NEW: Self = Self(PhantomData);
}

/////////////////////////////////////////////////////////////////////////////

impl<K, T: ?Sized, R: ?Sized> IsAWriteMarker<K, T, R> {
    #[inline(always)]
    pub const fn infer_type(self, _: &R) -> Self {
        self
    }
}

/////////////////////////////////////////////////////////////////////////////

impl<T: ?Sized, R: ?Sized> IsAWriteMarker<IsAStrWriter, StrWriter<T>, R> {
    #[inline(always)]
    pub const fn coerce(self, mutref: &mut StrWriter) -> StrWriterMut<'_> {
        mutref.as_mut()
    }
}

impl<T: ?Sized, R: ?Sized> IsAWriteMarker<IsNotAStrWriter, T, R> {
    #[inline(always)]
    pub const fn coerce(self, mutref: &mut T) -> &mut T {
        mutref
    }
}

/////////////////////////////////////////////////////////////////////////////
