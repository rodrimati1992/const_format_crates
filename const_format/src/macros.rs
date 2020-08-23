#[macro_use]
mod constructors;

#[macro_use]
mod helper_macros;

#[macro_use]
mod fmt_macros;

#[macro_use]
#[cfg(feature = "with_fmt")]
mod impl_debug;

/// Equivalent to the old `try` macro, or the `?` operator.
#[macro_export]
macro_rules! try_ {
    ($e:expr) => {
        match $e {
            $crate::pmr::Ok(x) => x,
            $crate::pmr::Err(e) => return Err(e),
        }
    };
}
