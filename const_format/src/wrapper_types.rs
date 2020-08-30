//! Some wrapper types.

#[cfg(feature = "fmt")]
pub(crate) mod ascii_str;

pub(crate) mod pwrapper;

#[cfg(feature = "fmt")]
pub(crate) mod sliced;

#[cfg(feature = "fmt")]
pub use self::ascii_str::NotAsciiError;

#[doc(no_inline)]
pub use crate::{AsciiStr, Sliced};

#[doc(no_inline)]
pub use crate::PWrapper;
