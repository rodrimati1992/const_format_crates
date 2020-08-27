//! Wrappers for standard library types.

#[cfg(feature = "fmt")]
mod ascii_str;

mod pwrapper;

#[cfg(feature = "fmt")]
mod sliced;

#[cfg(feature = "fmt")]
pub use self::{ascii_str::AsciiStr, sliced::Sliced};

pub use self::pwrapper::PWrapper;
