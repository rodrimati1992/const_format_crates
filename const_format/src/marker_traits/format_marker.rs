//! Marker trait for types that implement the const formatting methods.
//!
//!

use crate::wrapper_types::PWrapper;

use core::marker::PhantomData;

////////////////////////////////////////////////////////////////////////////////

/// Marker trait for types that implement the const formatting methods.
///
///
///
pub trait FormatMarker {
    type Kind;
    type This: ?Sized;

    const KIND: IsAFormatMarker<Self::Kind, Self::This, Self> = IsAFormatMarker::NEW;
}

pub struct IsArrayKind<T>(PhantomData<T>);
pub struct IsStdKind;
pub struct IsNotStdKind;

macro_rules! std_kind_impls {
    ($($ty:ty),* $(,)* ) => (
        $(
            impl FormatMarker for $ty {
                type Kind = IsStdKind;
                type This = Self;
            }

            impl<T> IsAFormatMarker<IsStdKind, $ty, T> {
                #[inline(always)]
                pub const fn coerce(self, reference: &$ty) -> PWrapper<$ty> {
                    PWrapper(*reference)
                }
            }
        )*
    )
}

macro_rules! array_impls {
    ($($len:literal),* $(,)* ) => (
        $(
            impl<T> FormatMarker for [T; $len] {
                type Kind = IsArrayKind<T>;
                type This = Self;
            }
        )*
    )
}

std_kind_impls! {
    i8, u8,
    i16, u16,
    i32, u32,
    i64, u64,
    i128, u128,
    isize, usize,
    bool,
}

impl FormatMarker for str {
    type Kind = IsStdKind;
    type This = Self;
}

impl<R: ?Sized> IsAFormatMarker<IsStdKind, str, R> {
    #[inline(always)]
    pub const fn coerce(self, reference: &str) -> PWrapper<&str> {
        PWrapper(reference)
    }
}

array_impls! {
    0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,
    16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,
    32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,
    48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63,
}

impl<T> FormatMarker for [T] {
    type Kind = IsArrayKind<T>;
    type This = [T];
}

impl<T> FormatMarker for &T
where
    T: ?Sized + FormatMarker,
{
    type Kind = T::Kind;
    type This = T::This;
}

impl<T> FormatMarker for &mut T
where
    T: ?Sized + FormatMarker,
{
    type Kind = T::Kind;
    type This = T::This;
}

///////////////////////////////////////////////////////////////////////////////

/// Hack used to automcatically wrap standard library types inside [`PWrapper`],
/// while leaving user defined unwrapped.
pub struct IsAFormatMarker<K, T: ?Sized, R: ?Sized>(
    PhantomData<(
        K,
        PhantomData<fn() -> PhantomData<T>>,
        PhantomData<fn() -> PhantomData<R>>,
    )>,
);

impl<K, T: ?Sized, R: ?Sized> Copy for IsAFormatMarker<K, T, R> {}

impl<K, T: ?Sized, R: ?Sized> Clone for IsAFormatMarker<K, T, R> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<R> IsAFormatMarker<R::Kind, R::This, R>
where
    R: ?Sized + FormatMarker,
{
    pub const NEW: Self = Self(PhantomData);
}

impl<K, T: ?Sized, R: ?Sized> IsAFormatMarker<K, T, R> {
    #[inline(always)]
    pub const fn infer_type(self, _: &R) -> Self {
        self
    }

    #[inline(always)]
    pub const fn unreference(self, r: &T) -> &T {
        r
    }
}

/////////////////////////////////////////////////////////////////////////////

impl<U, T: ?Sized, R: ?Sized> IsAFormatMarker<IsArrayKind<U>, T, R> {
    #[inline(always)]
    pub const fn coerce(self, slice: &[U]) -> PWrapper<&[U]> {
        PWrapper(slice)
    }
}

impl<T: ?Sized, R: ?Sized> IsAFormatMarker<IsNotStdKind, T, R> {
    #[inline(always)]
    pub const fn coerce(self, reference: &T) -> &T {
        reference
    }
}

/////////////////////////////////////////////////////////////////////////////
