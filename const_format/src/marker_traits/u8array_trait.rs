use core::marker::PhantomData;

mod sealed {
    pub trait Sealed {}
}
use self::sealed::Sealed;

/// Marker trait required to safely construct `StrWriter` before const generics are stable.
///
pub trait U8Array: Sealed + Copy + Sized + 'static {
    const MARKER: IsU8Array<Self> = IsU8Array::NEW;
}

macro_rules! impl_array_for_sizes {
    ($($expr:expr,)* ) => (
        $(
            impl U8Array for [u8; $expr] {}
            impl Sealed for [u8; $expr] {}
        )*
    )
}

impl_array_for_sizes! {
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
    16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
    32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
    48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63,
    64, 80, 96, 112, 128, 160, 192, 256, 384, 512, 768,
    1024, 2048, 4096, 8192,
    16384, 32768, 65536,
    131072, 262144, 524288,
}

/// A marker type used as a proof that `T` is a `u8` array.
pub struct IsU8Array<T>(PhantomData<fn() -> T>);

impl<T> IsU8Array<T>
where
    T: U8Array,
{
    pub const NEW: Self = Self(PhantomData);
}
